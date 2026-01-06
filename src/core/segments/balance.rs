use super::{Segment, SegmentData};
use crate::api::{cache, client::ApiClient, ApiConfig};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct BalanceSegment;

impl BalanceSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for BalanceSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        // 静默处理所有错误，避免影响整个 statusline
        self.try_collect().ok().flatten()
    }

    fn id(&self) -> SegmentId {
        SegmentId::Balance
    }
}

impl BalanceSegment {
    fn try_collect(&self) -> Result<Option<SegmentData>, Box<dyn std::error::Error>> {
        // 优先使用缓存
        let (cached, _needs_refresh) = cache::get_cached_balance();
        if let Some(balance) = cached {
            return Ok(Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            }));
        }

        // 如果没有 BALANCE_API_KEY，直接返回 None（不显示）
        let balance_key = std::env::var("BALANCE_API_KEY").ok();
        let api_key = match balance_key {
            Some(key) => key,
            None => return Ok(None),
        };

        let api_url = std::env::var("BALANCE_API_URL")
            .unwrap_or_else(|_| "https://api.ikuncode.cc/api/user/self".to_string());

        let config = ApiConfig {
            enabled: true,
            api_key,
            api_url,
            user_id: std::env::var("BALANCE_API_USER").ok(),
        };

        let client = ApiClient::new(config);
        let balance = client.get_balance()?;
        let _ = cache::save_cached_balance(&balance);

        Ok(Some(SegmentData {
            primary: balance.format_display(),
            secondary: String::new(),
            metadata: HashMap::new(),
        }))
    }
}
