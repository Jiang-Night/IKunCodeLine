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
        // 优先使用缓存
        let (cached, _needs_refresh) = cache::get_cached_balance();
        if let Some(balance) = cached {
            return Some(SegmentData {
                primary: balance.format_display(),
                secondary: String::new(),
                metadata: HashMap::new(),
            });
        }

        // 从环境变量获取配置
        let api_key = std::env::var("BALANCE_API_KEY").ok()?;
        let api_url = std::env::var("BALANCE_API_URL")
            .unwrap_or_else(|_| "https://api.ikuncode.cc/api/user/self".to_string());

        let config = ApiConfig {
            enabled: true,
            api_key,
            api_url,
        };

        let client = ApiClient::new(config);
        let balance = client.get_balance().ok()?;
        let _ = cache::save_cached_balance(&balance);

        Some(SegmentData {
            primary: balance.format_display(),
            secondary: String::new(),
            metadata: HashMap::new(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Balance
    }
}
