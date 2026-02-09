//! Issues 子命令

use anyhow::Context;
use crate::cli::app::IssuesAction;
use crate::cli::output::Output;
use crate::client::*;
use crate::RedmineClient;

pub async fn run(client: &RedmineClient, out: &Output, action: IssuesAction) -> anyhow::Result<()> {
    match action {
        IssuesAction::List {
            project_id,
            status,
            assigned_to,
            tracker_id,
            limit,
            offset,
            sort,
        } => {
            let params = IssueListParams {
                project_id: project_id.clone(),
                status_id: status,
                assigned_to_id: assigned_to,
                tracker_id,
                limit: Some(limit),
                offset,
                sort,
            };
            let resp = client.get_issues(&params).await
                .with_context(|| format!("取得 Issues 失敗 (project: {:?})", project_id))?;
            let rows: Vec<Vec<String>> = resp
                .issues
                .iter()
                .map(|i| {
                    vec![
                        i.id.to_string(),
                        i.tracker.name.clone(),
                        i.status.name.clone(),
                        i.priority.name.clone(),
                        i.assigned_to.as_ref().map(|a| a.name.clone()).unwrap_or_default(),
                        i.subject.clone(),
                    ]
                })
                .collect();
            out.print_table(
                &["#", "Tracker", "Status", "Priority", "Assignee", "Subject"],
                rows,
                &serde_json::to_value(&resp)?,
            );
            if !out.json {
                println!("({}/{})", resp.issues.len(), resp.total_count);
            }
        }
        IssuesAction::Show { id } => {
            let resp = client.get_issue(id).await
                .with_context(|| format!("取得 Issue #{id} 失敗"))?;
            let i = &resp.issue;
            let pairs: Vec<(&str, String)> = vec![
                ("ID", i.id.to_string()),
                ("Subject", i.subject.clone()),
                ("Project", i.project.name.clone()),
                ("Tracker", i.tracker.name.clone()),
                ("Status", i.status.name.clone()),
                ("Priority", i.priority.name.clone()),
                ("Author", i.author.name.clone()),
                ("Assignee", i.assigned_to.as_ref().map(|a| a.name.clone()).unwrap_or("-".into())),
                ("Done", format!("{}%", i.done_ratio)),
                ("Version", i.fixed_version.as_ref().map(|v| v.name.clone()).unwrap_or("-".into())),
                ("Created", i.created_on.clone()),
                ("Updated", i.updated_on.clone()),
            ];
            out.print_detail(&pairs, &serde_json::to_value(&resp)?);
            if !out.json {
                if let Some(desc) = &i.description {
                    if !desc.is_empty() {
                        println!("\n--- Description ---\n{desc}");
                    }
                }
            }
        }
        IssuesAction::Update {
            id,
            notes,
            status_id,
            assigned_to_id,
            priority_id,
            done_ratio,
        } => {
            let params = IssueUpdateParams {
                notes,
                status_id,
                assigned_to_id,
                priority_id,
                done_ratio,
            };
            client.update_issue(id, &params).await
                .with_context(|| format!("更新 Issue #{id} 失敗"))?;
            out.print_ok(&format!("Issue #{id} 已更新"));
        }
        IssuesAction::Journals { id } => {
            let resp = client.get_journals(id).await
                .with_context(|| format!("取得 Issue #{id} 歷史失敗"))?;
            let journals = resp.issue.journals.clone().unwrap_or_default();
            let rows: Vec<Vec<String>> = journals
                .iter()
                .map(|j| {
                    let changes: Vec<String> = j
                        .details
                        .iter()
                        .map(|d| {
                            format!(
                                "{}: {} → {}",
                                d.name,
                                d.old_value.as_deref().unwrap_or("-"),
                                d.new_value.as_deref().unwrap_or("-")
                            )
                        })
                        .collect();
                    vec![
                        j.id.to_string(),
                        j.user.name.clone(),
                        j.created_on.clone(),
                        j.notes.clone().unwrap_or_default(),
                        changes.join("; "),
                    ]
                })
                .collect();
            out.print_table(
                &["#", "User", "Date", "Notes", "Changes"],
                rows,
                &serde_json::to_value(&resp)?,
            );
        }
    }
    Ok(())
}
