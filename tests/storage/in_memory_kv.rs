use std::collections::HashSet;
use rusty_micros::storage::in_memory_kv::InMemoryKeyValueStore;
use rusty_micros::storage::kv_traits::async_kv_storage::AsyncKeyValueStorage;
use rusty_micros::storage::kv_traits::models::*;

#[tokio::test]
async fn test_add_and_get() {
    let mut store = InMemoryKeyValueStore::default();

    let key = Key("key1".to_string());
    let value = Value("value1".to_string());
    let tags: HashSet<Tag> = HashSet::new();

    store.add(key.clone(), value.clone(), tags.clone()).await;
    let result = store.get(&key).await;

    assert_eq!(result, Some(value));
}

#[tokio::test]
async fn test_add_and_get_tags() {
    let mut store = InMemoryKeyValueStore::default();

    let key = Key("key2".to_string());
    let value = Value("value2".to_string());
    let mut tags = HashSet::new();
    tags.insert(Tag("tag1".to_string()));

    store.add(key.clone(), value.clone(), tags.clone()).await;
    let result = store.get_tags(&key).await;

    assert_eq!(result, Some(tags));
}

#[tokio::test]
async fn test_find_keys_by_tag() {
    let mut store = InMemoryKeyValueStore::default();

    let key1 = Key("key3".to_string());
    let key2 = Key("key4".to_string());
    let value = Value("value3".to_string());
    let mut tags = HashSet::new();
    tags.insert(Tag("tag2".to_string()));

    store.add(key1.clone(), value.clone(), tags.clone()).await;
    store.add(key2.clone(), value.clone(), tags.clone()).await;

    let result = store.find_keys_by_tag(&Tag("tag2".to_string())).await;
    assert!(result.contains(&key1));
    assert!(result.contains(&key2));
}

#[tokio::test]
async fn test_remove() {
    let mut store = InMemoryKeyValueStore::default();

    let key = Key("key5".to_string());
    let value = Value("value5".to_string());
    let mut tags = HashSet::new();
    tags.insert(Tag("tag3".to_string()));

    store.add(key.clone(), value.clone(), tags.clone()).await;
    let removed = store.remove(&key).await;

    assert_eq!(removed, Some((value, tags)));
    let result = store.get(&key).await;
    assert_eq!(result, None);
}
