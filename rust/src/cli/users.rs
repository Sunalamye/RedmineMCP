//! Users 子命令

use anyhow::Context;
use crate::cli::app::UsersAction;
use crate::cli::output::Output;
use crate::client::*;
use crate::RedmineClient;

pub async fn run(client: &RedmineClient, out: &Output, action: UsersAction) -> anyhow::Result<()> {
    match action {
        UsersAction::List { name, group_id, limit } => {
            let params = UserListParams {
                name,
                group_id,
                limit: Some(limit),
                ..Default::default()
            };
            let resp = client.get_users(&params).await.context("取得使用者列表失敗")?;
            let rows: Vec<Vec<String>> = resp
                .users
                .iter()
                .map(|u| {
                    vec![
                        u.id.to_string(),
                        u.login.clone(),
                        format!("{} {}", u.firstname, u.lastname),
                        u.mail.clone().unwrap_or_default(),
                    ]
                })
                .collect();
            out.print_table(&["ID", "Login", "Name", "Email"], rows, &serde_json::to_value(&resp)?);
        }
        UsersAction::Show { id } => {
            let resp = client.get_user(id).await
                .with_context(|| format!("取得使用者 #{id} 失敗"))?;
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
        }
    }
    Ok(())
}
