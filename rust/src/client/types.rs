//! Redmine API 類型定義

use serde::{Deserialize, Serialize};

// ========== Common Types ==========

/// ID + Name 結構
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdName {
    pub id: u64,
    pub name: String,
}

/// ID + Name + Default 結構
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdNameDefault {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub is_default: bool,
}

// ========== Issue Types ==========

/// Issue 詳情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub project: IdName,
    pub tracker: IdName,
    pub status: IdName,
    pub priority: IdName,
    pub author: IdName,
    #[serde(default)]
    pub assigned_to: Option<IdName>,
    pub subject: String,
    #[serde(default)]
    pub description: Option<String>,
    pub done_ratio: u8,
    pub created_on: String,
    pub updated_on: String,
    #[serde(default)]
    pub fixed_version: Option<IdName>,
    #[serde(default)]
    pub category: Option<IdName>,
    #[serde(default)]
    pub journals: Option<Vec<Journal>>,
    #[serde(default)]
    pub attachments: Option<Vec<Attachment>>,
}

/// Issue Journal (歷史記錄)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    pub id: u64,
    pub user: IdName,
    #[serde(default)]
    pub notes: Option<String>,
    pub created_on: String,
    #[serde(default)]
    pub details: Vec<JournalDetail>,
}

/// Journal 詳情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalDetail {
    pub property: String,
    pub name: String,
    #[serde(default)]
    pub old_value: Option<String>,
    #[serde(default)]
    pub new_value: Option<String>,
}

/// Issue 列表查詢參數
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssueListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

/// Issue 更新參數
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IssueUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_ratio: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_id: Option<u64>,
}

/// Issue 關聯參數
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueRelationParams {
    pub issue_to_id: u64,
    pub relation_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
}

/// Issue 關聯
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueRelation {
    pub id: u64,
    pub issue_id: u64,
    pub issue_to_id: u64,
    pub relation_type: String,
    #[serde(default)]
    pub delay: Option<i32>,
}

/// Issue 分類
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCategory {
    pub id: u64,
    pub project: IdName,
    pub name: String,
    #[serde(default)]
    pub assigned_to: Option<IdName>,
}

// ========== Project Types ==========

/// 專案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub identifier: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// 專案成員
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id: u64,
    #[serde(default)]
    pub user: Option<IdName>,
    pub roles: Vec<IdName>,
}

// ========== User Types ==========

/// 使用者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(default)]
    pub mail: Option<String>,
    pub created_on: String,
    #[serde(default)]
    pub last_login_on: Option<String>,
    #[serde(default)]
    pub status: Option<u8>,
    #[serde(default)]
    pub groups: Option<Vec<IdName>>,
    #[serde(default)]
    pub memberships: Option<Vec<UserMembership>>,
}

/// 使用者的專案成員資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMembership {
    pub project: IdName,
    pub roles: Vec<IdName>,
}

/// 使用者列表查詢參數
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

// ========== Time Entry Types ==========

/// 工時記錄
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: u64,
    pub project: IdName,
    #[serde(default)]
    pub issue: Option<IssueRef>,
    pub user: IdName,
    pub activity: IdName,
    pub hours: f64,
    #[serde(default)]
    pub comments: Option<String>,
    pub spent_on: String,
    pub created_on: String,
    pub updated_on: String,
}

/// Issue 參考 (只有 id)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueRef {
    pub id: u64,
}

/// 工時列表查詢參數
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TimeEntryListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

/// 工時建立參數
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    pub hours: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_on: Option<String>,
}

// ========== Version Types ==========

/// 版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: u64,
    pub project: IdName,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub status: String,
    #[serde(default)]
    pub due_date: Option<String>,
    pub sharing: String,
    pub created_on: String,
    pub updated_on: String,
}

// ========== Wiki Types ==========

/// Wiki 頁面摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPageSummary {
    pub title: String,
    pub version: u64,
    pub created_on: String,
    pub updated_on: String,
}

/// Wiki 頁面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPage {
    pub title: String,
    pub text: String,
    pub version: u64,
    pub author: IdName,
    pub created_on: String,
    pub updated_on: String,
}

/// Wiki 頁面更新參數
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiPageParams {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

// ========== File Types ==========

/// 專案檔案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub id: u64,
    pub filename: String,
    pub filesize: u64,
    pub content_type: String,
    #[serde(default)]
    pub description: Option<String>,
    pub content_url: String,
    pub created_on: String,
}

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: u64,
    pub filename: String,
    pub filesize: u64,
    pub content_type: String,
    #[serde(default)]
    pub description: Option<String>,
    pub content_url: String,
    pub author: IdName,
    pub created_on: String,
}

/// 上傳結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upload {
    pub id: u64,
    pub token: String,
}

/// 下載結果
#[derive(Debug, Clone)]
pub struct DownloadResult {
    pub saved_to: String,
    pub filename: String,
}

// ========== Search Types ==========

/// 搜尋參數
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
}

/// 搜尋結果項目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u64,
    pub title: String,
    #[serde(rename = "type")]
    pub result_type: String,
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
    pub datetime: String,
}

// ========== Other Types ==========

/// 狀態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueStatus {
    pub id: u64,
    pub name: String,
    pub is_closed: bool,
}

/// 已存查詢
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedQuery {
    pub id: u64,
    pub name: String,
    pub is_public: bool,
    #[serde(default)]
    pub project_id: Option<u64>,
}

/// 新聞
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct News {
    pub id: u64,
    pub project: IdName,
    pub author: IdName,
    pub title: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub created_on: String,
}

/// 通用 API 回應
#[derive(Debug, Clone, Serialize)]
pub struct GenericResponse {
    pub status_code: u16,
    pub body: Option<serde_json::Value>,
    pub error: String,
}

// ========== Response Wrappers ==========

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueResponse {
    pub issue: Issue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueListResponse {
    pub issues: Vec<Issue>,
    pub total_count: u64,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectListResponse {
    pub projects: Vec<Project>,
    pub total_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipListResponse {
    pub memberships: Vec<Membership>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<User>,
    pub total_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerListResponse {
    pub trackers: Vec<IdName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusListResponse {
    pub issue_statuses: Vec<IssueStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityListResponse {
    pub issue_priorities: Vec<IdNameDefault>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryListResponse {
    pub time_entries: Vec<TimeEntry>,
    pub total_count: u64,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryCreatedResponse {
    pub time_entry: IssueRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntryActivityListResponse {
    pub time_entry_activities: Vec<IdNameDefault>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionListResponse {
    pub versions: Vec<Version>,
    pub total_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationListResponse {
    pub relations: Vec<IssueRelation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationResponse {
    pub relation: IssueRelation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueCategoryListResponse {
    pub issue_categories: Vec<IssueCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiPageListResponse {
    pub wiki_pages: Vec<WikiPageSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiPageResponse {
    pub wiki_page: WikiPage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileListResponse {
    pub files: Vec<ProjectFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentResponse {
    pub attachment: Attachment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub upload: Upload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_count: u64,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryListResponse {
    pub queries: Vec<SavedQuery>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListResponse {
    pub roles: Vec<IdName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupListResponse {
    pub groups: Vec<IdName>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsListResponse {
    pub news: Vec<News>,
}
