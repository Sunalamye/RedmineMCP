//! Redmine API 客戶端

mod types;

pub use types::*;

use crate::error::{RedmineError, Result};
use reqwest::{header, Client, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;
use tracing::{debug, info};

/// Redmine API 客戶端
#[derive(Clone)]
pub struct RedmineClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl RedmineClient {
    /// 建立新的客戶端
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-Redmine-API-Key",
            header::HeaderValue::from_str(api_key)
                .map_err(|_| RedmineError::Config("無效的 API Key".into()))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("MitakeRedmineMCP/1.0"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        })
    }

    /// 測試連線
    pub async fn login(&self) -> Result<bool> {
        match self.get_current_user().await {
            Ok(_) => {
                info!("Redmine 認證驗證成功");
                Ok(true)
            }
            Err(e) => {
                info!("Redmine 認證失敗: {}", e);
                Ok(false)
            }
        }
    }

    // ========== Issues ==========

    /// 取得 Issue 列表
    pub async fn get_issues(&self, params: &IssueListParams) -> Result<IssueListResponse> {
        self.get_with_query("/issues.json", params).await
    }

    /// 取得單一 Issue
    pub async fn get_issue(&self, id: u64) -> Result<IssueResponse> {
        self.get(&format!("/issues/{}.json?include=journals,attachments", id))
            .await
    }

    /// 更新 Issue
    pub async fn update_issue(&self, id: u64, params: &IssueUpdateParams) -> Result<()> {
        let body = serde_json::json!({ "issue": params });
        self.put(&format!("/issues/{}.json", id), &body).await
    }

    /// 取得 Issue Journals
    pub async fn get_journals(&self, issue_id: u64) -> Result<IssueResponse> {
        self.get(&format!("/issues/{}.json?include=journals", issue_id))
            .await
    }

    // ========== Projects ==========

    /// 取得專案列表
    pub async fn get_projects(&self) -> Result<ProjectListResponse> {
        self.get("/projects.json?limit=100").await
    }

    /// 取得專案成員
    pub async fn get_project_members(&self, project_id: &str) -> Result<MembershipListResponse> {
        self.get(&format!(
            "/projects/{}/memberships.json?limit=100",
            project_id
        ))
        .await
    }

    // ========== Users ==========

    /// 取得當前使用者
    pub async fn get_current_user(&self) -> Result<UserResponse> {
        self.get("/users/current.json").await
    }

    /// 取得使用者列表
    pub async fn get_users(&self, params: &UserListParams) -> Result<UserListResponse> {
        self.get_with_query("/users.json", params).await
    }

    /// 取得使用者詳情
    pub async fn get_user(&self, id: u64) -> Result<UserResponse> {
        self.get(&format!("/users/{}.json?include=groups,memberships", id))
            .await
    }

    // ========== Metadata ==========

    /// 取得 Trackers
    pub async fn get_trackers(&self) -> Result<TrackerListResponse> {
        self.get("/trackers.json").await
    }

    /// 取得狀態列表
    pub async fn get_statuses(&self) -> Result<StatusListResponse> {
        self.get("/issue_statuses.json").await
    }

    /// 取得優先權列表
    pub async fn get_priorities(&self) -> Result<PriorityListResponse> {
        self.get("/enumerations/issue_priorities.json").await
    }

    // ========== Time Entries ==========

    /// 取得工時列表
    pub async fn get_time_entries(
        &self,
        params: &TimeEntryListParams,
    ) -> Result<TimeEntryListResponse> {
        self.get_with_query("/time_entries.json", params).await
    }

    /// 建立工時
    pub async fn create_time_entry(&self, params: &TimeEntryCreateParams) -> Result<TimeEntryCreatedResponse> {
        let body = serde_json::json!({ "time_entry": params });
        self.post("/time_entries.json", &body).await
    }

    /// 取得工時活動類型
    pub async fn get_time_entry_activities(&self) -> Result<TimeEntryActivityListResponse> {
        self.get("/enumerations/time_entry_activities.json").await
    }

    // ========== Versions ==========

    /// 取得版本列表
    pub async fn get_versions(&self, project_id: &str) -> Result<VersionListResponse> {
        self.get(&format!("/projects/{}/versions.json", project_id))
            .await
    }

    /// 取得版本詳情
    pub async fn get_version(&self, id: u64) -> Result<VersionResponse> {
        self.get(&format!("/versions/{}.json", id)).await
    }

    // ========== Relations ==========

    /// 取得 Issue 關聯
    pub async fn get_issue_relations(&self, issue_id: u64) -> Result<RelationListResponse> {
        self.get(&format!("/issues/{}/relations.json", issue_id))
            .await
    }

    /// 建立 Issue 關聯
    pub async fn create_issue_relation(
        &self,
        issue_id: u64,
        params: &IssueRelationParams,
    ) -> Result<RelationResponse> {
        let body = serde_json::json!({ "relation": params });
        self.post(&format!("/issues/{}/relations.json", issue_id), &body)
            .await
    }

    /// 刪除 Issue 關聯
    pub async fn delete_issue_relation(&self, relation_id: u64) -> Result<()> {
        self.delete(&format!("/relations/{}.json", relation_id))
            .await
    }

    // ========== Categories ==========

    /// 取得 Issue 分類
    pub async fn get_issue_categories(&self, project_id: &str) -> Result<IssueCategoryListResponse> {
        self.get(&format!(
            "/projects/{}/issue_categories.json",
            project_id
        ))
        .await
    }

    // ========== Wiki ==========

    /// 取得 Wiki 頁面列表
    pub async fn get_wiki_pages(&self, project_id: &str) -> Result<WikiPageListResponse> {
        self.get(&format!("/projects/{}/wiki/index.json", project_id))
            .await
    }

    /// 取得 Wiki 頁面
    pub async fn get_wiki_page(&self, project_id: &str, title: &str) -> Result<WikiPageResponse> {
        let encoded_title = urlencoding::encode(title);
        self.get(&format!(
            "/projects/{}/wiki/{}.json",
            project_id, encoded_title
        ))
        .await
    }

    /// 更新 Wiki 頁面
    pub async fn update_wiki_page(
        &self,
        project_id: &str,
        title: &str,
        params: &WikiPageParams,
    ) -> Result<()> {
        let encoded_title = urlencoding::encode(title);
        let body = serde_json::json!({ "wiki_page": params });
        self.put(
            &format!("/projects/{}/wiki/{}.json", project_id, encoded_title),
            &body,
        )
        .await
    }

    // ========== Files ==========

    /// 取得專案檔案
    pub async fn get_files(&self, project_id: &str) -> Result<FileListResponse> {
        self.get(&format!("/projects/{}/files.json", project_id))
            .await
    }

    /// 取得附件資訊
    pub async fn get_attachment(&self, id: u64) -> Result<AttachmentResponse> {
        self.get(&format!("/attachments/{}.json", id)).await
    }

    /// 上傳檔案
    pub async fn upload_file(&self, file_path: &str, description: Option<&str>) -> Result<UploadResponse> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(RedmineError::FileNotFound {
                path: file_path.to_string(),
            });
        }

        let content = tokio::fs::read(path).await?;
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");

        let mut url = format!(
            "{}/uploads.json?filename={}",
            self.base_url,
            urlencoding::encode(filename)
        );
        if let Some(desc) = description {
            url.push_str(&format!("&description={}", urlencoding::encode(desc)));
        }

        let response = self
            .client
            .post(&url)
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .body(content)
            .send()
            .await?;

        Self::handle_response(response).await
    }

    /// 下載附件
    pub async fn download_attachment(&self, attachment_id: u64, save_path: &str) -> Result<DownloadResult> {
        let info = self.get_attachment(attachment_id).await?;
        let content_url = &info.attachment.content_url;
        let filename = info.attachment.filename.clone();

        let response = self
            .client
            .get(content_url)
            .header("X-Redmine-API-Key", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(RedmineError::from_response(response).await);
        }

        let bytes = response.bytes().await?;
        tokio::fs::write(save_path, &bytes).await?;

        Ok(DownloadResult {
            saved_to: save_path.to_string(),
            filename,
        })
    }

    // ========== Search ==========

    /// 全文搜尋
    pub async fn search(&self, query: &str, params: &SearchParams) -> Result<SearchResponse> {
        let mut full_params = params.clone();
        full_params.q = Some(query.to_string());
        self.get_with_query("/search.json", &full_params).await
    }

    // ========== Others ==========

    /// 取得已存查詢
    pub async fn get_queries(&self) -> Result<QueryListResponse> {
        self.get("/queries.json").await
    }

    /// 取得角色列表
    pub async fn get_roles(&self) -> Result<RoleListResponse> {
        self.get("/roles.json").await
    }

    /// 取得群組列表
    pub async fn get_groups(&self) -> Result<GroupListResponse> {
        self.get("/groups.json").await
    }

    /// 取得新聞
    pub async fn get_news(&self, project_id: Option<&str>) -> Result<NewsListResponse> {
        let path = match project_id {
            Some(id) => format!("/projects/{}/news.json", id),
            None => "/news.json".to_string(),
        };
        self.get(&path).await
    }

    /// 通用 API 請求
    pub async fn request(
        &self,
        path: &str,
        method: &str,
        data: Option<&serde_json::Value>,
        params: Option<&std::collections::HashMap<String, String>>,
    ) -> Result<GenericResponse> {
        let normalized_path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        let mut url = format!("{}{}", self.base_url, normalized_path);

        if let Some(p) = params {
            if !p.is_empty() {
                let query: String = p
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&");
                url.push_str(if url.contains('?') { "&" } else { "?" });
                url.push_str(&query);
            }
        }

        let method = method.to_uppercase();
        let mut request = match method.as_str() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "PATCH" => self.client.patch(&url),
            "DELETE" => self.client.delete(&url),
            _ => self.client.get(&url),
        };

        if let Some(d) = data {
            if ["POST", "PUT", "PATCH"].contains(&method.as_str()) {
                request = request.json(d);
            }
        }

        let response = request.send().await?;
        let status_code = response.status().as_u16();
        let is_success = response.status().is_success();

        let body: Option<serde_json::Value> = if is_success && status_code != 204 {
            response.json().await.ok()
        } else if !is_success {
            let text = response.text().await.ok();
            text.and_then(|t| serde_json::from_str(&t).ok())
        } else {
            None
        };

        Ok(GenericResponse {
            status_code,
            body,
            error: if is_success {
                String::new()
            } else {
                format!("HTTP {}", status_code)
            },
        })
    }

    // ========== Internal Helpers ==========

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("GET {}", url);
        let response = self.client.get(&url).send().await?;
        Self::handle_response(response).await
    }

    async fn get_with_query<T: DeserializeOwned, Q: Serialize>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T> {
        let base_url = format!("{}{}", self.base_url, path);
        let query_string = serde_urlencoded::to_string(query)
            .map_err(|e| RedmineError::Config(format!("Failed to encode query: {}", e)))?;
        let url = if query_string.is_empty() {
            base_url
        } else {
            format!("{}?{}", base_url, query_string)
        };
        debug!("GET {}", url);
        let response = self.client.get(&url).send().await?;
        Self::handle_response(response).await
    }

    async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST {}", url);
        let response = self.client.post(&url).json(body).send().await?;
        Self::handle_response(response).await
    }

    async fn put<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        debug!("PUT {}", url);
        let response = self.client.put(&url).json(body).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(RedmineError::from_response(response).await)
        }
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        debug!("DELETE {}", url);
        let response = self.client.delete(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(RedmineError::from_response(response).await)
        }
    }

    async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T> {
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(RedmineError::from_response(response).await)
        }
    }
}
