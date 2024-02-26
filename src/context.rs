use crate::cache::TokenCache;

pub struct Context {
    pub cache: TokenCache,
}

impl Context {
    pub fn new(cache: TokenCache) -> Self {
        Context { cache }
    }
}

impl juniper::Context for Context {}
