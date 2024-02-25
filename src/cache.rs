use lru::LruCache;
use std::time::{ SystemTime, Duration };
use std::num::NonZeroUsize;
use crate::schemas::schema::Balance;
use crate::external_api::moralis::TokenData;

// Define a struct to represent the cache
pub struct TokenCache {
    cache: LruCache<String, Vec<Balance>>,
    last_update_time: Option<SystemTime>,
}

impl TokenCache {
    // Constructor to create a new TokenCache with a specified capacity
    pub fn new(capacity: NonZeroUsize) -> Self {
        TokenCache {
            cache: LruCache::new(capacity),
            last_update_time: None,
        }
    }

    // Method to fetch tokens from Moralis API, respecting once-per-minute limit
    pub async fn fetch_tokens(&mut self, user: String) -> Vec<Balance> {
        // Check if the cache was updated within the last minute
        if let Some(last_update_time) = self.last_update_time {
            if let Ok(elapsed) = last_update_time.elapsed() {
                if elapsed < Duration::from_secs(60) {
                    // Return tokens from cache if updated within the last minute
                    if let Some(tokens) = self.cache.get(&user) {
                        return tokens.clone();
                    }
                }
            }
        }

        // If cache is empty or last update was more than a minute ago, fetch tokens from API
        let tokens = self.fetch_tokens_from_api(user.clone()).await; // Call Moralis API to fetch tokens
        self.cache.put(user, tokens.clone()); // Update cache
        self.last_update_time = Some(SystemTime::now()); // Update last update time
        tokens
    }

    // Method to simulate fetching tokens from Moralis API
    async fn fetch_tokens_from_api(&mut self, user: String) -> Vec<Balance> {
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
