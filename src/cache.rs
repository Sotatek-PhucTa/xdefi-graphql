use lru::LruCache;
use tokio::sync::RwLock;
use std::time::{ SystemTime, Duration };
use crate::schemas::schema::Balance;
use crate::external_api::moralis::TokenData;
use crate::validates::env::get_cache_size;

pub struct TokenCacheValue {
    last_update_time: Option<SystemTime>,
    data: Vec<Balance>,
}

// Define a struct to represent the cache
pub struct TokenCache {
    cache: RwLock<LruCache<String, TokenCacheValue>>,
}

impl TokenCache {
    // Constructor to create a new TokenCache with a specified capacity
    pub fn new() -> Self {
        TokenCache {
            cache: RwLock::new(LruCache::new(get_cache_size().unwrap())),
        }
    }

    // Method to fetch tokens from Moralis API, respecting once-per-minute limit
    pub async fn fetch_tokens(&self, user: String) -> Vec<Balance> {
        let mut cache = self.cache.write().await;

        // Check if the cache was updated within the last minute
        if let Some(last_update_time) = cache.get(&user).unwrap().last_update_time {
            if let Ok(elapsed) = last_update_time.elapsed() {
                if elapsed < Duration::from_secs(60) {
                    // Return tokens from cache if updated within the last minute
                    if let Some(tokens) = cache.get(&user) {
                        let data = &tokens.data;
                        return data.clone();
                    }
                }
            }
        }

        // If cache is empty or last update was more than a minute ago, fetch tokens from API
        let tokens = self.fetch_tokens_from_api(user.clone()).await; // Call Moralis API to fetch tokens
        cache.put(user, { TokenCacheValue {
                last_update_time: Some(SystemTime::now()),
                data: tokens.clone(),
            } }); // Update cache
        tokens
    }

    // Method to simulate fetching tokens from Moralis API
    async fn fetch_tokens_from_api(&self, user: String) -> Vec<Balance> {
        let moralis_data = TokenData::get(&user).await.unwrap();
        let data: Vec<Balance> = moralis_data
            .iter()
            .map(|el| {
                Balance {
                    asset_address: el.token_address.clone(),
                    value: el.balance.clone(),
                    decimals: el.decimals as i32,
                }
            })
            .collect::<Vec<Balance>>();

        data
    }
}
