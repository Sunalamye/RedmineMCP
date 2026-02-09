//! Versions 子命令

use anyhow::Context;
use crate::cli::app::VersionsAction;
use crate::cli::output::Output;
use crate::RedmineClient;

pub async fn run(client: &RedmineClient, out: &Output, action: VersionsAction) -> anyhow::Result<()> {
    match action {
        VersionsAction::List { project_id } => {
            let resp = client.get_versions(&project_id).await
                .with_context(|| format!("取得版本列表失敗 (project: {project_id})"))?;
            let rows: Vec<Vec<String>> = resp
                .versions
                .iter()
                .map(|v| {
                    vec![
                        v.id.to_string(),
                        v.name.clone(),
                        v.status.clone(),
                        v.due_date.clone().unwrap_or("-".into()),
                        v.sharing.clone(),
                    ]
                })
                .collect();
            out.print_table(
                &["ID", "Name", "Status", "Due Date", "Sharing"],
                rows,
                &serde_json::to_value(&resp)?,
            );
        }
        VersionsAction::Show { id } => {
            let resp = client.get_version(id).await
                .with_context(|| format!("取得版本 #{id} 失敗"))?;
            let v = &resp.version;
            let pairs: Vec<(&str, String)> = vec![
                ("ID", v.id.to_string()),
                ("Name", v.name.clone()),
                ("Project", v.project.name.clone()),
                ("Status", v.status.clone()),
                ("Due Date", v.due_date.clone().unwrap_or("-".into())),
                ("Sharing", v.sharing.clone()),
                ("Description", v.description.clone().unwrap_or("-".into())),
                ("Created", v.created_on.clone()),
                ("Updated", v.updated_on.clone()),
            ];
            out.print_detail(&pairs, &serde_json::to_value(&resp)?);
        }
    }
    Ok(())
}
