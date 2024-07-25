use async_trait::async_trait;
use std::collections::HashSet;
use super::models::{Key, Value, Tag};

#[async_trait]
pub trait AsyncKeyValueStorage {
    async fn add(&mut self, key: Key, value: Value, tags: HashSet<Tag>);
    async fn get(&self, key: &Key) -> Option<Value>;
    async fn get_tags(&self, key: &Key) -> Option<HashSet<Tag>>;
    async fn find_keys_by_tag(&self, tag: &Tag) -> Vec<Key>;
    async fn remove(&mut self, key: &Key) -> Option<(Value, HashSet<Tag>)>;
}