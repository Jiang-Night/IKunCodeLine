use super::{Segment, SegmentData};
use crate::api::{cache, client::ApiClient, ApiConfig};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct GroupSegment;

impl GroupSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for GroupSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        self.try_collect().ok().flatten()
    }

    fn id(&self) -> SegmentId {
        SegmentId::Group
    }
}

impl GroupSegment {
    fn try_collect(&self) -> Result<Option<SegmentData>, Box<dyn std::error::Error>> {
        // 优先使用缓存
        let (cached, _) = cache::get_cached_balance();
        if let Some(balance) = cached {
            if !balance.group.is_empty() {
                return Ok(Some(SegmentData {
                    primary: balance.group.clone(),
                    secondary: String::new(),
                    metadata: HashMap::new(),
                }));
            }
        }

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
        let balance = client.get_balance()?;
        let _ = cache::save_cached_balance(&balance);

        if balance.group.is_empty() {
            return Ok(None);
        }

        Ok(Some(SegmentData {
            primary: balance.group,
            secondary: String::new(),
            metadata: HashMap::new(),
        }))
    }
}
