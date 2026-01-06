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
        // 优先使用 BALANCE_API_KEY，否则回退到 ANTHROPIC_AUTH_TOKEN
        let balance_key = std::env::var("BALANCE_API_KEY").ok();
        let use_dedicated_key = balance_key.is_some();
        let api_key = balance_key
            .or_else(|| std::env::var("ANTHROPIC_AUTH_TOKEN").ok())?;

        // 如果使用专用 BALANCE_API_KEY，默认用 ikuncode URL
        // 否则从 ANTHROPIC_BASE_URL 推导
        let api_url = std::env::var("BALANCE_API_URL").unwrap_or_else(|_| {
            if use_dedicated_key {
                "https://api.ikuncode.cc/api/user/self".to_string()
            } else if let Ok(base_url) = std::env::var("ANTHROPIC_BASE_URL") {
                if let Some(pos) = base_url.find("://") {
                    let after_scheme = &base_url[pos + 3..];
                    if let Some(slash_pos) = after_scheme.find('/') {
                        let domain = &base_url[..pos + 3 + slash_pos];
                        return format!("{}/api/user/self", domain);
                    }
                }
                format!("{}/api/user/self", base_url.trim_end_matches('/'))
            } else {
                "https://api.ikuncode.cc/api/user/self".to_string()
            }
        });

        let config = ApiConfig {
            enabled: true,
            api_key,
            api_url,
            user_id: std::env::var("BALANCE_API_USER").ok(),
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
