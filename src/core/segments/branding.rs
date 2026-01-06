use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct BrandingSegment;

impl BrandingSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for BrandingSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        // 只在有 BALANCE_API_KEY 时显示（表示使用 ikuncode）
        if std::env::var("BALANCE_API_KEY").is_err() {
            return None;
        }

        Some(SegmentData {
            primary: "IKunCode".to_string(),
            secondary: String::new(),
            metadata: HashMap::new(),
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Branding
    }
}
