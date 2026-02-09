//! Wiki 子命令

use anyhow::Context;
use crate::cli::app::WikiAction;
use crate::cli::output::Output;
use crate::client::*;
use crate::RedmineClient;

pub async fn run(client: &RedmineClient, out: &Output, action: WikiAction) -> anyhow::Result<()> {
    match action {
        WikiAction::List { project_id } => {
            let resp = client.get_wiki_pages(&project_id).await
                .with_context(|| format!("取得 Wiki 列表失敗 (project: {project_id})"))?;
            let rows: Vec<Vec<String>> = resp
                .wiki_pages
                .iter()
                .map(|p| {
                    vec![
                        p.title.clone(),
                        p.version.to_string(),
                        p.updated_on.clone(),
                    ]
                })
                .collect();
            out.print_table(&["Title", "Version", "Updated"], rows, &serde_json::to_value(&resp)?);
        }
        WikiAction::Show { project_id, title } => {
            let resp = client.get_wiki_page(&project_id, &title).await
                .with_context(|| format!("取得 Wiki {project_id}/{title} 失敗"))?;
            if out.json {
                out.print_json(&serde_json::to_value(&resp)?);
            } else {
                let p = &resp.wiki_page;
                println!("# {} (v{})\n", p.title, p.version);
                println!("{}", p.text);
            }
        }
        WikiAction::Update {
            project_id,
            title,
            text,
            file,
            comments,
        } => {
            let content = if let Some(f) = file {
                tokio::fs::read_to_string(&f).await
                    .with_context(|| format!("讀取檔案 {f} 失敗"))?
            } else if let Some(t) = text {
                t
            } else {
                anyhow::bail!("請提供 --text 或 --file");
            };
            let params = WikiPageParams { text: content, comments };
            client.update_wiki_page(&project_id, &title, &params).await
                .with_context(|| format!("更新 Wiki {project_id}/{title} 失敗"))?;
            out.print_ok(&format!("Wiki {project_id}/{title} 已更新"));
        }
    }
    Ok(())
}
