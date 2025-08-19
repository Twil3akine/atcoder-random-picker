use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RateLimiter {
    pub last_request: Arc<Mutex<HashMap<IpAddr, Instant>>>,
    pub ttl: Duration,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self { 
            last_request: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(600),
        }
    }

    pub async fn check(&self, ip: IpAddr) -> bool {
        let mut map = self.last_request.lock().await;
        let now = Instant::now();

        map.retain(|_, &mut last| now.duration_since(last) < self.ttl);

        match map.get(&ip) {
            Some(&last) if now.duration_since(last) < Duration::from_secs(1) => {
                false
            },
            _ => {
                map.insert(ip, now);
                true
            }
        }
    }
}