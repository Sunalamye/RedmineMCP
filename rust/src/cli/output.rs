//! CLI 輸出格式化

use comfy_table::{presets::UTF8_FULL_CONDENSED, Table, ContentArrangement};
use serde_json::Value;

pub struct Output {
    pub json: bool,
}

impl Output {
    pub fn new(json: bool) -> Self {
        Self { json }
    }

    /// 輸出 JSON 或表格
    pub fn print_table(&self, headers: &[&str], rows: Vec<Vec<String>>, json_value: &Value) {
        if self.json {
            self.print_json(json_value);
            return;
        }
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL_CONDENSED)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(headers);
        for row in rows {
            table.add_row(row);
        }
        println!("{table}");
    }

    /// 輸出 key-value 詳情
    pub fn print_detail(&self, pairs: &[(&str, String)], json_value: &Value) {
        if self.json {
            self.print_json(json_value);
            return;
        }
        let max_key = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
        for (k, v) in pairs {
            println!("{:>width$}: {}", k, v, width = max_key);
        }
    }

    /// 輸出純 JSON
    pub fn print_json(&self, value: &Value) {
        match serde_json::to_string_pretty(value) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("JSON 序列化錯誤: {e}"),
        }
    }

    /// 輸出成功訊息
    pub fn print_ok(&self, msg: &str) {
        if self.json {
            println!(r#"{{"ok":true,"message":"{msg}"}}"#);
        } else {
            println!("{msg}");
        }
    }

    /// 輸出 ID + Name 列表
    pub fn print_id_name_list(&self, title: &str, items: &[(u64, String)], json_value: &Value) {
        if self.json {
            self.print_json(json_value);
            return;
        }
        println!("{title}");
        for (id, name) in items {
            println!("  {id:>5}  {name}");
        }
    }
}
