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
        self.try_collect().ok().flatten()
    }

    fn id(&self) -> SegmentId {
        SegmentId::Balance
    }
}

impl BalanceSegment {
    fn try_collect(&self) -> Result<Option<SegmentData>, Box<dyn std::error::Error>> {
        // 如果没有 BALANCE_API_KEY，直接返回 None
        let api_key = match std::env::var("BALANCE_API_KEY").ok() {
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

        // 先尝试调用 API 获取最新数据
        if let Ok(balance) = client.get_balance() {
            let _ = cache::save_cached_balance(&balance);
            return Ok(Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            }));
        }

        // API 失败时，使用缓存作为 fallback
        let (cached, _) = cache::get_cached_balance();
        if let Some(balance) = cached {
            return Ok(Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            }));
        }

        Ok(None)
    }
}
