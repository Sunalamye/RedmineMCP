//! Time 子命令

use anyhow::Context;
use crate::cli::app::TimeAction;
use crate::cli::output::Output;
use crate::client::*;
use crate::RedmineClient;

pub async fn run(client: &RedmineClient, out: &Output, action: TimeAction) -> anyhow::Result<()> {
    match action {
        TimeAction::List {
            project_id,
            user_id,
            from,
            to,
            limit,
        } => {
            let params = TimeEntryListParams {
                project_id,
                user_id,
                from,
                to,
                limit: Some(limit),
                ..Default::default()
            };
            let resp = client.get_time_entries(&params).await.context("取得工時列表失敗")?;
            let rows: Vec<Vec<String>> = resp
                .time_entries
                .iter()
                .map(|t| {
                    vec![
                        t.id.to_string(),
                        t.spent_on.clone(),
                        t.user.name.clone(),
                        t.project.name.clone(),
                        t.issue.as_ref().map(|i| format!("#{}", i.id)).unwrap_or_default(),
                        format!("{:.1}h", t.hours),
                        t.activity.name.clone(),
                        t.comments.clone().unwrap_or_default(),
                    ]
                })
                .collect();
            out.print_table(
                &["ID", "Date", "User", "Project", "Issue", "Hours", "Activity", "Comment"],
                rows,
                &serde_json::to_value(&resp)?,
            );
        }
        TimeAction::Log {
            hours,
            issue_id,
            project_id,
            activity_id,
            comments,
            spent_on,
        } => {
            let params = TimeEntryCreateParams {
                hours,
                issue_id,
                project_id,
                activity_id,
                comments,
                spent_on,
            };
            let resp = client.create_time_entry(&params).await.context("建立工時失敗")?;
            out.print_ok(&format!("工時已建立 (ID: {})", resp.time_entry.id));
        }
        TimeAction::Activities => {
            let resp = client.get_time_entry_activities().await.context("取得活動類型失敗")?;
            let items: Vec<(u64, String)> = resp
                .time_entry_activities
                .iter()
                .map(|a| (a.id, a.name.clone()))
                .collect();
            out.print_id_name_list("活動類型:", &items, &serde_json::to_value(&resp)?);
        }
    }
    Ok(())
}
