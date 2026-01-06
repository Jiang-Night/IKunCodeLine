use super::BalanceData;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

const CACHE_FRESH_SECS: u64 = 300; // 5分钟

fn get_cache_dir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let cache_dir = home.join(".claude").join("ikuncodeline").join("cache");
    fs::create_dir_all(&cache_dir).ok()?;
    Some(cache_dir)
}

fn get_cache_file() -> Option<PathBuf> {
    Some(get_cache_dir()?.join("balance.json"))
}

fn is_cache_fresh(path: &PathBuf) -> bool {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .map(|d| d.as_secs() < CACHE_FRESH_SECS)
        .unwrap_or(false)
}

/// 返回 (缓存数据, 是否需要刷新)
pub fn get_cached_balance() -> (Option<BalanceData>, bool) {
    let path = match get_cache_file() {
        Some(p) => p,
        None => return (None, false),
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return (None, false),
    };

    let data: Option<BalanceData> = serde_json::from_str(&content).ok();
    let needs_refresh = data.is_some() && !is_cache_fresh(&path);

    (data, needs_refresh)
}

pub fn save_cached_balance(data: &BalanceData) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = get_cache_file() {
        fs::write(path, serde_json::to_string(data)?)?;
    }
    Ok(())
}
