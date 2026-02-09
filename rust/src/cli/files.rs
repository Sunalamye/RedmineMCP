//! Files 子命令

use anyhow::Context;
use crate::cli::app::FilesAction;
use crate::cli::output::Output;
use crate::RedmineClient;

pub async fn run(client: &RedmineClient, out: &Output, action: FilesAction) -> anyhow::Result<()> {
    match action {
        FilesAction::List { project_id } => {
            let resp = client.get_files(&project_id).await
                .with_context(|| format!("取得檔案列表失敗 (project: {project_id})"))?;
            let rows: Vec<Vec<String>> = resp
                .files
                .iter()
                .map(|f| {
                    vec![
                        f.id.to_string(),
                        f.filename.clone(),
                        format_size(f.filesize),
                        f.created_on.clone(),
                    ]
                })
                .collect();
            out.print_table(&["ID", "Filename", "Size", "Created"], rows, &serde_json::to_value(&resp)?);
        }
        FilesAction::Upload { file, description } => {
            let resp = client.upload_file(&file, description.as_deref()).await
                .with_context(|| format!("上傳檔案 {file} 失敗"))?;
            out.print_ok(&format!("上傳成功 (token: {})", resp.upload.token));
        }
        FilesAction::Download { id, output } => {
            let save_path = match output {
                Some(p) => p,
                None => {
                    let info = client.get_attachment(id).await
                        .with_context(|| format!("取得附件 #{id} 資訊失敗"))?;
                    info.attachment.filename
                }
            };
            let result = client.download_attachment(id, &save_path).await
                .with_context(|| format!("下載附件 #{id} 失敗"))?;
            out.print_ok(&format!("已下載: {} → {}", result.filename, result.saved_to));
        }
    }
    Ok(())
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;
    match bytes {
        b if b < KB => format!("{b} B"),
        b if b < MB => format!("{:.1} KB", b as f64 / KB as f64),
        b => format!("{:.1} MB", b as f64 / MB as f64),
    }
}
