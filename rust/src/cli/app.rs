//! CLI 命令結構定義（clap derive）

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "redmine", version, about = "Redmine CLI & MCP Server")]
pub struct Cli {
    /// 啟動 MCP server 模式
    #[arg(long)]
    pub mcp: bool,

    /// JSON 輸出
    #[arg(long, global = true)]
    pub json: bool,

    /// Redmine URL
    #[arg(long, global = true)]
    pub url: Option<String>,

    /// API token
    #[arg(long, global = true)]
    pub token: Option<String>,

    /// 使用指定 profile
    #[arg(long, global = true)]
    pub profile: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// 儲存憑證（預設存本地 .redmine，-g 存全域）
    Login {
        /// Redmine URL
        #[arg(long)]
        url: String,
        /// API Token
        #[arg(long)]
        token: String,
        /// 存到全域 credentials.toml（預設存本地 .redmine）
        #[arg(short = 'g', long)]
        global: bool,
        /// Profile 名稱（預設自動產生）
        #[arg(long)]
        profile: Option<String>,
        /// 設為預設 profile
        #[arg(long)]
        set_default: bool,
    },
    /// 移除憑證
    Logout {
        /// 移除全域 profile（預設移除本地 .redmine）
        #[arg(short = 'g', long)]
        global: bool,
        /// 指定 profile 名稱
        #[arg(long)]
        profile: Option<String>,
    },
    /// 顯示連線資訊與 profile 來源
    Status {
        /// 顯示所有 profiles
        #[arg(short, long)]
        all: bool,
    },
    /// 當前使用者
    Me,
    /// 查看日誌 / 開啟 Web Log Viewer
    Log {
        /// 開啟 Web Log Viewer
        #[arg(long)]
        web: bool,
        /// 持續追蹤 (類似 tail -f)
        #[arg(short, long)]
        follow: bool,
    },

    /// Issues 操作
    Issues {
        #[command(subcommand)]
        action: IssuesAction,
    },
    /// 專案操作
    Projects {
        #[command(subcommand)]
        action: ProjectsAction,
    },
    /// 使用者操作
    Users {
        #[command(subcommand)]
        action: UsersAction,
    },
    /// 工時操作
    Time {
        #[command(subcommand)]
        action: TimeAction,
    },
    /// 版本操作
    Versions {
        #[command(subcommand)]
        action: VersionsAction,
    },
    /// Wiki 操作
    Wiki {
        #[command(subcommand)]
        action: WikiAction,
    },
    /// 檔案操作
    Files {
        #[command(subcommand)]
        action: FilesAction,
    },

    /// 全文搜尋
    Search {
        /// 搜尋關鍵字
        query: String,
        /// 專案 ID
        #[arg(long)]
        project_id: Option<String>,
        /// 筆數上限
        #[arg(long, default_value = "25")]
        limit: u64,
    },
    /// Tracker 列表
    Trackers,
    /// 狀態列表
    Statuses,
    /// 優先權列表
    Priorities,
    /// 分類列表
    Categories {
        /// 專案 ID
        project_id: String,
    },
    /// 已存查詢
    Queries,
    /// 角色列表
    Roles,
    /// 群組列表
    Groups,
    /// 新聞列表
    News {
        /// 專案 ID（可選）
        project_id: Option<String>,
    },
    /// 通用 API 請求
    Api {
        /// API 路徑（如 /projects.json）
        path: String,
        /// HTTP 方法
        #[arg(long, default_value = "GET")]
        method: String,
        /// JSON body
        #[arg(long)]
        data: Option<String>,
    },
}

// ========== Issues ==========

#[derive(Subcommand)]
pub enum IssuesAction {
    /// Issues 列表
    List {
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        assigned_to: Option<String>,
        #[arg(long)]
        tracker_id: Option<u64>,
        #[arg(long, default_value = "25")]
        limit: u64,
        #[arg(long)]
        offset: Option<u64>,
        #[arg(long)]
        sort: Option<String>,
    },
    /// Issue 詳情
    Show {
        /// Issue ID
        id: u64,
    },
    /// 更新 Issue
    Update {
        /// Issue ID
        id: u64,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        status_id: Option<u64>,
        #[arg(long)]
        assigned_to_id: Option<u64>,
        #[arg(long)]
        priority_id: Option<u64>,
        #[arg(long)]
        done_ratio: Option<u8>,
    },
    /// Issue 歷史
    Journals {
        /// Issue ID
        id: u64,
    },
}

// ========== Projects ==========

#[derive(Subcommand)]
pub enum ProjectsAction {
    /// 專案列表
    List,
    /// 專案成員
    Members {
        /// 專案 ID
        project_id: String,
    },
}

// ========== Users ==========

#[derive(Subcommand)]
pub enum UsersAction {
    /// 使用者列表
    List {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        group_id: Option<u64>,
        #[arg(long, default_value = "25")]
        limit: u64,
    },
    /// 使用者詳情
    Show {
        /// 使用者 ID
        id: u64,
    },
}

// ========== Time ==========

#[derive(Subcommand)]
pub enum TimeAction {
    /// 工時列表
    List {
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long)]
        user_id: Option<String>,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
        #[arg(long, default_value = "25")]
        limit: u64,
    },
    /// 建立工時
    Log {
        /// 工時（小時）
        #[arg(long)]
        hours: f64,
        /// Issue ID
        #[arg(long)]
        issue_id: Option<u64>,
        /// 專案 ID
        #[arg(long)]
        project_id: Option<String>,
        /// 活動類型 ID
        #[arg(long)]
        activity_id: Option<u64>,
        /// 備註
        #[arg(long)]
        comments: Option<String>,
        /// 日期 (YYYY-MM-DD)
        #[arg(long)]
        spent_on: Option<String>,
    },
    /// 活動類型
    Activities,
}

// ========== Versions ==========

#[derive(Subcommand)]
pub enum VersionsAction {
    /// 版本列表
    List {
        /// 專案 ID
        project_id: String,
    },
    /// 版本詳情
    Show {
        /// 版本 ID
        id: u64,
    },
}

// ========== Wiki ==========

#[derive(Subcommand)]
pub enum WikiAction {
    /// Wiki 頁面列表
    List {
        /// 專案 ID
        project_id: String,
    },
    /// Wiki 頁面內容
    Show {
        /// 專案 ID
        project_id: String,
        /// 頁面標題
        title: String,
    },
    /// 更新 Wiki 頁面
    Update {
        /// 專案 ID
        project_id: String,
        /// 頁面標題
        title: String,
        /// 內容（或用 --file 讀檔）
        #[arg(long)]
        text: Option<String>,
        /// 從檔案讀取內容
        #[arg(long)]
        file: Option<String>,
        /// 備註
        #[arg(long)]
        comments: Option<String>,
    },
}

// ========== Files ==========

#[derive(Subcommand)]
pub enum FilesAction {
    /// 檔案列表
    List {
        /// 專案 ID
        project_id: String,
    },
    /// 上傳檔案
    Upload {
        /// 檔案路徑
        file: String,
        /// 說明
        #[arg(long)]
        description: Option<String>,
    },
    /// 下載附件
    Download {
        /// 附件 ID
        id: u64,
        /// 儲存路徑
        #[arg(long, short)]
        output: Option<String>,
    },
}
