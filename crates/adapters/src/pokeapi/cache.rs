use std::{collections::HashMap, time::{SystemTime, Duration}};
use serde::{Serialize, Deserialize};


pub trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<V>;
    fn set(&mut self, key: K, value: V);
    fn is_stale(&self, key: &K) -> bool;
}


const TTL: Duration = Duration::from_secs(60 * 60 * 24); // 24h

#[derive(Serialize, Deserialize, Clone)]
pub struct CacheEntry<T> {
    pub value: T,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DiskCache<T> {
    map: HashMap<String, CacheEntry<T>>,
}

impl<T: Clone> DiskCache<T> {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        self.map.get(key).map(|v| v.value.clone())
    }

    pub fn set(&mut self, key: String, value: T) {
        let now = now();
        self.map.insert(key, CacheEntry { value, timestamp: now });
    }

    pub fn is_stale(&self, key: &str) -> bool {
        self.map.get(key)
            .map(|e| now() - e.timestamp > TTL.as_secs())
            .unwrap_or(true)
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> DiskCache<T> {
    pub fn load(path: &str) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(Self::new)
    }

    pub fn save(&self, path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(path, json);
        }
    }
}