pub mod cache;
pub mod client;

use serde::{Deserialize, Serialize};

/// API 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub enabled: bool,
    pub api_key: String,
    /// 完整的 API endpoint URL（不是 base URL）
    /// 例如: "https://api.ikuncode.cc/api/user/self"
    pub api_url: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_key: String::new(),
            api_url: "https://api.ikuncode.cc/api/user/self".to_string(),
        }
    }
}

/// new-api 格式的用户数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub success: bool,
    pub data: Option<UserData>,
    pub message: Option<String>,
}

/// 用户数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub quota: i64,
    pub used_quota: i64,
    pub request_count: i64,
}

/// 余额数据（用于显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceData {
    pub quota: i64,
    pub used: i64,
    pub remaining: f64,
    pub percentage: f64,
    pub is_unlimited: bool,
    pub is_overused: bool,
}

/// 无限额度阈值：quota >= 1000亿 视为无限
const UNLIMITED_THRESHOLD: i64 = 100_000_000_000;

impl BalanceData {
    /// 从 UserData 计算余额（带边界处理）
    pub fn from_user_data(data: &UserData) -> Self {
        // 检测无限额度
        let is_unlimited = data.quota >= UNLIMITED_THRESHOLD;

        // 检测超额使用
        let is_overused = data.used_quota > data.quota && !is_unlimited;

        // 计算剩余额度（clamp 到 0）
        let raw_remaining = data.quota - data.used_quota;
        let remaining = if is_unlimited {
            0.0 // 无限额度时设为 0，依赖 is_unlimited 标记判断
        } else {
            (raw_remaining.max(0) as f64) / 500000.0
        };

        // 计算百分比（clamp 到 0-100）
        let percentage = if is_unlimited {
            100.0
        } else if data.quota <= 0 {
            0.0
        } else {
            let pct = (raw_remaining as f64 / data.quota as f64) * 100.0;
            pct.clamp(0.0, 100.0)
        };

        Self {
            quota: data.quota,
            used: data.used_quota,
            remaining,
            percentage,
            is_unlimited,
            is_overused,
        }
    }

    /// 格式化显示
    pub fn format_display(&self) -> String {
        if self.is_unlimited {
            "∞".to_string()
        } else if self.is_overused {
            "超额".to_string()
        } else {
            format!("${:.2}", self.remaining)
        }
    }
}
