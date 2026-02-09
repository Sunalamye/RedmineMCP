//! CLI dispatcher

pub mod app;
mod output;
mod issues;
mod projects;
mod users;
mod time;
mod versions;
mod wiki;
mod files;
mod metadata;

pub use app::Cli;
use app::Command;
use output::Output;
use crate::credential::{CredentialFile, LocalConfig, Profile};
use crate::Config;
use crate::RedmineClient;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let out = Output::new(cli.json);

    match &cli.command {
        Some(Command::Login { url, token, global, profile, set_default }) => {
            return do_login(url, token, *global, profile.as_deref(), *set_default, &out).await;
        }
        Some(Command::Logout { global, profile }) => {
            return do_logout(*global, profile.as_deref(), &out);
        }
        Some(Command::Log { web, follow }) => {
            return do_log(*web, *follow).await;
        }
        None => {
            use clap::CommandFactory;
            Cli::command().print_help()?;
            return Ok(());
        }
        _ => {}
    }

    // 其餘命令需要憑證 — 提前驗證
    let (config, resolved) = Config::resolve(
        cli.url.as_deref(),
        cli.token.as_deref(),
        cli.profile.as_deref(),
    )?;
    let client = RedmineClient::new(&config.redmine_url, &config.redmine_token)?;

    match cli.command.unwrap() {
        Command::Status { all } => do_status(&client, &out, &resolved, all).await,
        Command::Me => do_me(&client, &out).await,
        Command::Issues { action } => issues::run(&client, &out, action).await,
        Command::Projects { action } => projects::run(&client, &out, action).await,
        Command::Users { action } => users::run(&client, &out, action).await,
        Command::Time { action } => time::run(&client, &out, action).await,
        Command::Versions { action } => versions::run(&client, &out, action).await,
        Command::Wiki { action } => wiki::run(&client, &out, action).await,
        Command::Files { action } => files::run(&client, &out, action).await,
        Command::Search { query, project_id, limit } => {
            metadata::search(&client, &out, &query, project_id.as_deref(), limit).await
        }
        Command::Trackers => metadata::trackers(&client, &out).await,
        Command::Statuses => metadata::statuses(&client, &out).await,
        Command::Priorities => metadata::priorities(&client, &out).await,
        Command::Categories { project_id } => metadata::categories(&client, &out, &project_id).await,
        Command::Queries => metadata::queries(&client, &out).await,
        Command::Roles => metadata::roles(&client, &out).await,
        Command::Groups => metadata::groups(&client, &out).await,
        Command::News { project_id } => {
            metadata::news(&client, &out, project_id.as_deref()).await
        }
        Command::Api { path, method, data } => {
            metadata::api(&client, &out, &path, &method, data.as_deref()).await
        }
        Command::Login { .. } | Command::Logout { .. } | Command::Log { .. } => unreachable!(),
    }
}

/// 從 URL 產生 profile 名稱（如 https://redmine.company.com → redmine-company-com）
fn profile_name_from_url(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .replace([':', '.', '/'], "-")
}

async fn do_login(
    url: &str,
    token: &str,
    global: bool,
    profile_name: Option<&str>,
    set_default: bool,
    out: &Output,
) -> anyhow::Result<()> {
    // 先驗證連線
    let client = RedmineClient::new(url, token)?;
    let resp = client.get_current_user().await?;
    let u = &resp.user;

    let name = profile_name
        .map(String::from)
        .unwrap_or_else(|| profile_name_from_url(url));

    // 儲存 profile 到全域 credentials.toml
    let mut cred_file = CredentialFile::load_or_default();
    cred_file.profiles.insert(name.clone(), Profile {
        url: url.to_string(),
        token: token.to_string(),
    });

    // 第一個 profile 或明確指定 → 設為預設
    if set_default || cred_file.default_profile.is_none() {
        cred_file.default_profile = Some(name.clone());
    }
    cred_file.save()?;

    if global {
        out.print_ok(&format!(
            "登入成功: {} {} ({})\nProfile '{name}' 已儲存至 {}",
            u.firstname, u.lastname, u.login,
            CredentialFile::path().map(|p| p.display().to_string()).unwrap_or_default()
        ));
    } else {
        // 本地模式：額外建立 .redmine
        let local = LocalConfig { profile: name.clone() };
        let path = local.save_to_cwd()?;
        out.print_ok(&format!(
            "登入成功: {} {} ({})\nProfile '{name}' 已儲存，本地 .redmine → {}",
            u.firstname, u.lastname, u.login,
            path.display()
        ));
    }
    Ok(())
}

fn do_logout(global: bool, profile_name: Option<&str>, out: &Output) -> anyhow::Result<()> {
    if global {
        if let Some(name) = profile_name {
            let mut cred_file = CredentialFile::load_or_default();
            if cred_file.profiles.remove(name).is_some() {
                if cred_file.default_profile.as_deref() == Some(name) {
                    cred_file.default_profile = None;
                }
                cred_file.save()?;
                out.print_ok(&format!("已移除 profile '{name}'"));
            } else {
                out.print_ok(&format!("Profile '{name}' 不存在"));
            }
        } else {
            CredentialFile::remove()?;
            out.print_ok("已移除全部全域憑證");
        }
    } else {
        if LocalConfig::remove_from_cwd()? {
            out.print_ok("已移除本地 .redmine");
        } else {
            out.print_ok("本地無 .redmine");
        }
    }
    Ok(())
}

async fn do_status(
    client: &RedmineClient,
    out: &Output,
    resolved: &crate::credential::ResolvedCredential,
    show_all: bool,
) -> anyhow::Result<()> {
    let resp = client.get_current_user().await?;
    let u = &resp.user;

    let pairs: Vec<(&str, String)> = vec![
        ("User", format!("{} {} ({})", u.firstname, u.lastname, u.login)),
        ("URL", resolved.url.clone()),
        ("Source", resolved.source.to_string()),
    ];
    out.print_detail(&pairs, &serde_json::to_value(&resp)?);

    if show_all {
        let cred_file = CredentialFile::load_or_default();
        let default = cred_file.default_profile.as_deref();
        println!("\nProfiles:");
        if cred_file.profiles.is_empty() {
            println!("  (無)");
        }
        let mut names: Vec<_> = cred_file.profiles.keys().collect();
        names.sort();
        for name in names {
            let p = &cred_file.profiles[name];
            let marker = if default == Some(name.as_str()) { " *" } else { "" };
            println!("  {name}{marker}  {}", p.url);
        }
        if let Some((path, local)) = LocalConfig::find() {
            println!("\nLocal .redmine: profile '{}' ({})", local.profile, path.display());
        }
    }
    Ok(())
}

async fn do_log(web: bool, follow: bool) -> anyhow::Result<()> {
    let log_file = std::env::var("LOG_FILE")
        .unwrap_or_else(|_| "/tmp/redmine-mcp.log".into());
    let path = std::path::Path::new(&log_file);

    if web {
        // 啟動 Web Log Viewer
        let port = crate::start_log_viewer().await;
        match port {
            Some(p) => {
                let url = format!("http://localhost:{}", p);
                println!("Log Viewer: {url}");
                // 如果有 log 檔，讀入歷史
                if path.exists() {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        for line in content.lines() {
                            if !line.is_empty() {
                                crate::log_to_viewer("INFO", None, line, None);
                            }
                        }
                    }
                }
                let _ = open::that(&url);
                println!("按 Ctrl+C 結束");
                // 持續監聽（保持 server 運行）
                tokio::signal::ctrl_c().await?;
            }
            None => {
                anyhow::bail!("無法啟動 Log Viewer（埠號被占用）");
            }
        }
    } else if !path.exists() {
        println!("Log 檔案不存在: {log_file}");
        println!("提示: MCP 模式 (redmine --mcp) 會產生日誌");
    } else if follow {
        // tail -f 模式
        println!("=== {log_file} (Ctrl+C 結束) ===");
        let mut last_pos = 0u64;
        loop {
            let metadata = std::fs::metadata(path)?;
            let size = metadata.len();
            if size > last_pos {
                use std::io::{Read, Seek};
                let mut f = std::fs::File::open(path)?;
                f.seek(std::io::SeekFrom::Start(last_pos))?;
                let mut buf = String::new();
                f.read_to_string(&mut buf)?;
                print!("{buf}");
                last_pos = size;
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    } else {
        // 顯示最後 50 行
        let content = std::fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();
        let start = if lines.len() > 50 { lines.len() - 50 } else { 0 };
        if start > 0 {
            println!("... ({start} 行已省略)");
        }
        for line in &lines[start..] {
            println!("{line}");
        }
        println!("\n提示: 使用 -f 持續追蹤，--web 開啟 Web Log Viewer");
    }
    Ok(())
}

async fn do_me(client: &RedmineClient, out: &Output) -> anyhow::Result<()> {
    let resp = client.get_current_user().await?;
    let u = &resp.user;
    let pairs: Vec<(&str, String)> = vec![
        ("ID", u.id.to_string()),
        ("Login", u.login.clone()),
        ("Name", format!("{} {}", u.firstname, u.lastname)),
        ("Email", u.mail.clone().unwrap_or("-".into())),
        ("Created", u.created_on.clone()),
        ("Last Login", u.last_login_on.clone().unwrap_or("-".into())),
    ];
    out.print_detail(&pairs, &serde_json::to_value(&resp)?);
    Ok(())
}
