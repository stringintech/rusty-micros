use std::collections::{HashMap, HashSet};
use tokio::sync::Mutex;
use async_trait::async_trait;
use super::kv_traits::models::{Key, Value, Tag};
use super::kv_traits::async_kv_storage::AsyncKeyValueStorage;

#[derive(Default)]
pub struct InMemoryKeyValueStore {
    store: Mutex<HashMap<Key, (Value, HashSet<Tag>)>>,
    tag_index: Mutex<HashMap<Tag, HashSet<Key>>>,
}

#[async_trait]
impl AsyncKeyValueStorage for InMemoryKeyValueStore {
    async fn add(&mut self, key: Key, value: Value, tags: HashSet<Tag>) {
        let mut store = self.store.lock().await;
        let mut tag_index = self.tag_index.lock().await;
        
        for tag in &tags {
            tag_index.entry(tag.clone()).or_default().insert(key.clone());
        }
        store.insert(key, (value, tags));
    }

    async fn get(&self, key: &Key) -> Option<Value> {
        let store = self.store.lock().await;
        store.get(key).map(|(value, _)| value.clone())
    }

    async fn get_tags(&self, key: &Key) -> Option<HashSet<Tag>> {
        let store = self.store.lock().await;
        store.get(key).map(|(_, tags)| tags.clone())
    }

    async fn find_keys_by_tag(&self, tag: &Tag) -> Vec<Key> {
        let tag_index = self.tag_index.lock().await;
        match tag_index.get(tag) {
            Some(keys) => keys.iter().cloned().collect(),
            None => Vec::new(),
        }
    }

    async fn remove(&mut self, key: &Key) -> Option<(Value, HashSet<Tag>)> {
        let mut store = self.store.lock().await;
        let mut tag_index = self.tag_index.lock().await;

        if let Some((value, tags)) = store.remove(key) {
            for tag in &tags {
                if let Some(keys) = tag_index.get_mut(tag) {
                    keys.remove(key);
                    if keys.is_empty() {
                        tag_index.remove(tag);
                    }
                }
            }
            Some((value, tags))
        } else {
            None
        }
    }
}