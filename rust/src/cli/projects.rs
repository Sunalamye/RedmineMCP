//! Projects 子命令

use anyhow::Context;
use crate::cli::app::ProjectsAction;
use crate::cli::output::Output;
use crate::RedmineClient;

pub async fn run(client: &RedmineClient, out: &Output, action: ProjectsAction) -> anyhow::Result<()> {
    match action {
        ProjectsAction::List => {
            let resp = client.get_projects().await.context("取得專案列表失敗")?;
            let rows: Vec<Vec<String>> = resp
                .projects
                .iter()
                .map(|p| {
                    vec![
                        p.id.to_string(),
                        p.identifier.clone(),
                        p.name.clone(),
                        p.description.clone().unwrap_or_default(),
                    ]
                })
                .collect();
            out.print_table(
                &["ID", "Identifier", "Name", "Description"],
                rows,
                &serde_json::to_value(&resp)?,
            );
        }
        ProjectsAction::Members { project_id } => {
            let resp = client.get_project_members(&project_id).await
                .with_context(|| format!("取得專案 {project_id} 成員失敗"))?;
            let rows: Vec<Vec<String>> = resp
                .memberships
                .iter()
                .map(|m| {
                    let user = m.user.as_ref().map(|u| u.name.clone()).unwrap_or("-".into());
                    let roles: Vec<String> = m.roles.iter().map(|r| r.name.clone()).collect();
                    vec![m.id.to_string(), user, roles.join(", ")]
                })
                .collect();
            out.print_table(&["ID", "User", "Roles"], rows, &serde_json::to_value(&resp)?);
        }
    }
    Ok(())
}
