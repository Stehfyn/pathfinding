use super::component::{self, Component, ComponentTrait, Transform2};
use super::pos2;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use std::sync::atomic::AtomicUsize;
static ENTITY_THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub static mut ENTITY_MANAGER: Lazy<HashMap<usize, Entity>> = Lazy::new(|| HashMap::new());

pub struct EntityInternalData {
    id: usize,
}

impl Default for EntityInternalData {
    fn default() -> Self {
        Self {
            id: ENTITY_THREAD_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}

#[derive(panel_macros::GenerateUI, Clone)]
pub struct EntityData {
    pub name: String,
    pub tag: String,
}

impl Default for EntityData {
    fn default() -> Self {
        Self {
            name: format!("Entity"),
            tag: "".to_string(),
        }
    }
}

pub struct Entity {
    pub data: EntityData,
    internal_data: EntityInternalData,
    pub components: Vec<Component>,
}

impl Default for Entity {
    fn default() -> Self {
        let mut components: Vec<Component> = Vec::default();

        components.push(Component::Transform2(ComponentTrait::default()));
        let pos = pos2::Pos2::new(50i64, 50i64);
        match components.last_mut().unwrap() {
            Component::Transform2(tc) => {
                tc.get_mut().pos = pos;
            }
            _ => {}
        }
        components.push(Component::Color(ComponentTrait::default()));
        components.push(Component::Mesh(ComponentTrait::default()));
        match components.last_mut().unwrap() {
            Component::Mesh(mc) => mc.get_mut().mesh.push(pos),
            _ => {}
        }
        let internal_data = EntityInternalData::default();
        let mut data = EntityData::default();
        data.name = format!("Entity {}", internal_data.id);

        Self {
            data: data,
            internal_data: internal_data,
            components: components,
        }
    }
}

impl Entity {
    pub fn get_id(&self) -> usize {
        self.internal_data.id
    }
}

pub unsafe fn get_selected_entities() -> Vec<&'static mut Entity> {
    let selected = crate::panel::scene_hierarchy_panel::get_selected();
    let mut entities = Vec::new();
    for (key, val) in ENTITY_MANAGER.iter_mut() {
        if selected.contains(key) {
            entities.push(val);
        }
    }
    entities
}

pub unsafe fn get_entity_from_id(id: usize) -> &'static mut Entity {
    ENTITY_MANAGER.get_mut(&id).unwrap()
}

pub unsafe fn get_entities_from_xy(x: f64, y: f64) -> Vec<&'static Entity> {
    let mut entities: Vec<&Entity> = Vec::new();
    for (key, val) in ENTITY_MANAGER.iter_mut() {
        for c in val.components.iter() {
            match c {
                Component::Mesh(mc) => {
                    let pos = super::pos2::Pos2 {
                        x: x as i64,
                        y: y as i64,
                    };
                    if mc.get().mesh.contains(&pos) {
                        entities.push(val);
                    }
                }
                _ => {}
            }
        }
    }
    entities
}

pub unsafe fn propagate_entity_changes() {
    let mut pos = pos2::Pos2::default();

    for (_key, val) in ENTITY_MANAGER.iter_mut() {
        if let Some(tc) = val.components.iter().find_map(|c| {
            if let Component::Transform2(tc) = c {
                Some(tc.get().pos)
            } else {
                None
            }
        }) {
            pos = tc;
        }

        for c in val.components.iter_mut() {
            if let Component::Mesh(mc) = c {
                for p in mc.get_mut().mesh.iter_mut() {
                    *p = pos;
                }
            }
        }
    }
}
