//! Metadata 子命令 — trackers, statuses, priorities, search, etc.

use anyhow::Context;
use crate::cli::output::Output;
use crate::client::*;
use crate::RedmineClient;

pub async fn trackers(client: &RedmineClient, out: &Output) -> anyhow::Result<()> {
    let resp = client.get_trackers().await.context("取得 Trackers 失敗")?;
    let items: Vec<(u64, String)> = resp.trackers.iter().map(|t| (t.id, t.name.clone())).collect();
    out.print_id_name_list("Trackers:", &items, &serde_json::to_value(&resp)?);
    Ok(())
}

pub async fn statuses(client: &RedmineClient, out: &Output) -> anyhow::Result<()> {
    let resp = client.get_statuses().await.context("取得狀態列表失敗")?;
    let rows: Vec<Vec<String>> = resp
        .issue_statuses
        .iter()
        .map(|s| {
            vec![
                s.id.to_string(),
                s.name.clone(),
                if s.is_closed { "closed" } else { "open" }.into(),
            ]
        })
        .collect();
    out.print_table(&["ID", "Name", "State"], rows, &serde_json::to_value(&resp)?);
    Ok(())
}

pub async fn priorities(client: &RedmineClient, out: &Output) -> anyhow::Result<()> {
    let resp = client.get_priorities().await.context("取得優先權列表失敗")?;
    let items: Vec<(u64, String)> = resp
        .issue_priorities
        .iter()
        .map(|p| (p.id, format!("{}{}", p.name, if p.is_default { " *" } else { "" })))
        .collect();
    out.print_id_name_list("Priorities:", &items, &serde_json::to_value(&resp)?);
    Ok(())
}

pub async fn categories(client: &RedmineClient, out: &Output, project_id: &str) -> anyhow::Result<()> {
    let resp = client.get_issue_categories(project_id).await
        .with_context(|| format!("取得分類失敗 (project: {project_id})"))?;
    let rows: Vec<Vec<String>> = resp
        .issue_categories
        .iter()
        .map(|c| {
            vec![
                c.id.to_string(),
                c.name.clone(),
                c.assigned_to.as_ref().map(|a| a.name.clone()).unwrap_or_default(),
            ]
        })
        .collect();
    out.print_table(&["ID", "Name", "Assignee"], rows, &serde_json::to_value(&resp)?);
    Ok(())
}

pub async fn queries(client: &RedmineClient, out: &Output) -> anyhow::Result<()> {
    let resp = client.get_queries().await.context("取得已存查詢失敗")?;
    let rows: Vec<Vec<String>> = resp
        .queries
        .iter()
        .map(|q| {
            vec![
                q.id.to_string(),
                q.name.clone(),
                if q.is_public { "public" } else { "private" }.into(),
            ]
        })
        .collect();
    out.print_table(&["ID", "Name", "Visibility"], rows, &serde_json::to_value(&resp)?);
    Ok(())
}

pub async fn roles(client: &RedmineClient, out: &Output) -> anyhow::Result<()> {
    let resp = client.get_roles().await.context("取得角色列表失敗")?;
    let items: Vec<(u64, String)> = resp.roles.iter().map(|r| (r.id, r.name.clone())).collect();
    out.print_id_name_list("Roles:", &items, &serde_json::to_value(&resp)?);
    Ok(())
}

pub async fn groups(client: &RedmineClient, out: &Output) -> anyhow::Result<()> {
    let resp = client.get_groups().await.context("取得群組列表失敗")?;
    let items: Vec<(u64, String)> = resp.groups.iter().map(|g| (g.id, g.name.clone())).collect();
    out.print_id_name_list("Groups:", &items, &serde_json::to_value(&resp)?);
    Ok(())
}

pub async fn news(client: &RedmineClient, out: &Output, project_id: Option<&str>) -> anyhow::Result<()> {
    let resp = client.get_news(project_id).await.context("取得新聞失敗")?;
    let rows: Vec<Vec<String>> = resp
        .news
        .iter()
        .map(|n| {
            vec![
                n.id.to_string(),
                n.project.name.clone(),
                n.title.clone(),
                n.author.name.clone(),
                n.created_on.clone(),
            ]
        })
        .collect();
    out.print_table(&["ID", "Project", "Title", "Author", "Date"], rows, &serde_json::to_value(&resp)?);
    Ok(())
}

pub async fn search(
    client: &RedmineClient,
    out: &Output,
    query: &str,
    project_id: Option<&str>,
    limit: u64,
) -> anyhow::Result<()> {
    let params = SearchParams {
        project_id: project_id.map(String::from),
        limit: Some(limit),
        ..Default::default()
    };
    let resp = client.search(query, &params).await
        .with_context(|| format!("搜尋 '{query}' 失敗"))?;
    let rows: Vec<Vec<String>> = resp
        .results
        .iter()
        .map(|r| {
            vec![
                r.id.to_string(),
                r.result_type.clone(),
                r.title.clone(),
                r.datetime.clone(),
            ]
        })
        .collect();
    out.print_table(&["ID", "Type", "Title", "Date"], rows, &serde_json::to_value(&resp)?);
    if !out.json {
        println!("({}/{})", resp.results.len(), resp.total_count);
    }
    Ok(())
}

pub async fn api(
    client: &RedmineClient,
    out: &Output,
    path: &str,
    method: &str,
    data: Option<&str>,
) -> anyhow::Result<()> {
    let data_value: Option<serde_json::Value> = data
        .map(|d| serde_json::from_str(d))
        .transpose()
        .context("JSON data 解析失敗")?;
    let resp = client.request(path, method, data_value.as_ref(), None).await
        .with_context(|| format!("{method} {path} 失敗"))?;
    if let Some(body) = &resp.body {
        out.print_json(body);
    } else {
        out.print_ok(&format!("HTTP {}", resp.status_code));
    }
    Ok(())
}
