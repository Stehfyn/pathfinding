use super::component::{Component, ComponentModifier, Transform};

use std::sync::atomic::AtomicUsize;
static ENTITY_THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct EntityStorage {
    name: String,
    tag: String,
    components: Vec<Box<dyn ComponentModifier>>,
}

impl Default for EntityStorage {
    fn default() -> Self {
        let count = ENTITY_THREAD_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            name: format!("Entity {count}"),
            tag: "".to_string(),
            components: Vec::new(),
        }
    }
}

pub struct Entity(EntityStorage);

impl Default for Entity {
    fn default() -> Self {
        let storage = EntityStorage::default();
        //storage.components.push(Transform::default());
        Self(storage)
    }
}

