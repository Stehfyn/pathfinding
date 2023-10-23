use std::collections::HashMap;

use crate::ecs::entity::{get_entity_from_id, Entity, ENTITY_MANAGER};
use once_cell::sync::Lazy;
static mut ENTITY_HIERARCHY_OPEN: Lazy<HashMap<usize, bool>> = Lazy::new(|| HashMap::new());
static mut ENTITY_HIERARCHY_SELECTED: Lazy<HashMap<usize, bool>> = Lazy::new(|| HashMap::new());

pub enum SelectionEvent {
    Add,
    Change,
    ToggleCollapse,
    Hovered,
}

pub fn make_hierarchy_selectable(
    ui: &mut egui::Ui,
    id: egui::Id,
    ui_closure: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> Option<SelectionEvent> {
    let response = ui_closure(ui);

    if response.clicked_by(egui::PointerButton::Primary) && ui.ctx().input(|i| i.raw.modifiers.ctrl)
    {
        log::info!("Ctrl + Left Click");
        return Some(SelectionEvent::Add);
    }
    if response.double_clicked_by(egui::PointerButton::Primary) {
        log::info!("Double Click");
        return Some(SelectionEvent::ToggleCollapse);
    }
    if response.clicked_by(egui::PointerButton::Primary) {
        log::info!("Select");
        return Some(SelectionEvent::Change);
    }
    if response.hovered() {
        return Some(SelectionEvent::Hovered);
    }
    None
}

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Keep,
    Delete,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Tree {
    #[serde(skip)]
    nodes: Vec<Tree>,
    #[serde(skip)]
    pub open: bool,
    #[serde(skip)]
    pub id: usize,
    #[serde(skip)]
    pub name: String,
}

impl Default for Tree {
    fn default() -> Self {
        let e = Entity::default();
        let id = e.get_id();
        unsafe {
            ENTITY_HIERARCHY_OPEN.insert(id, true);
            ENTITY_HIERARCHY_SELECTED.insert(id, false);
            ENTITY_MANAGER.insert(id, e);
        }
        let name = if id == 0_usize {
            "Scene".to_string()
        } else {
            format!("Entity {id}")
        };
        Self {
            nodes: Vec::default(),
            open: true,
            id: id,
            name: name,
        }
    }
}

impl Tree {
    pub fn ui(&mut self, ui: &mut egui::Ui, font_size: f32, font_scale: f32) -> Action {
        self.ui_impl(ui, 0, "Hierarchy", font_size, font_scale)
    }

    fn ui_impl(
        &mut self,
        ui: &mut egui::Ui,
        depth: usize,
        name: &str,
        font_size: f32,
        font_scale: f32,
    ) -> Action {
        let cursor_pos = ui.cursor().left_top();

        let label_rect = egui::Rect {
            min: egui::Pos2 {
                x: cursor_pos.x + 20f32,
                y: cursor_pos.y,
            },
            max: egui::Pos2 {
                x: cursor_pos.x + 110f32 + 20f32,
                y: cursor_pos.y + 18f32,
            },
        };

        let mut name_str = self.name.clone();
        if self.id != 0_usize {
            unsafe {
                name_str = get_entity_from_id(self.id).data.name.clone();
            }
        }
        let eid = self.id;

        let act = egui::CollapsingHeader::new(egui::RichText::new("").size(font_size * font_scale))
            .id_source(self.id)
            .default_open(self.open)
            .open(Some(self.open))
            .icon(move |ui, openness, response| {
                egui::collapsing_header::paint_default_icon(ui, openness, &response);
                if response.clicked() {
                    unsafe {
                        if let Some(state) = ENTITY_HIERARCHY_OPEN.get_mut(&eid) {
                            *state = !*state;
                        }
                    }
                }
                let mut selected = false;
                // use move keyword
                unsafe {
                    selected = *ENTITY_HIERARCHY_SELECTED.get(&eid).unwrap();
                }

                let id = ui.next_auto_id();

                match make_hierarchy_selectable(ui, id, |ui| {
                    ui.put(
                        label_rect,
                        egui::SelectableLabel::new(
                            selected,
                            egui::RichText::new(&name_str).size(font_size * font_scale),
                        ),
                    )
                }) {
                    Some(event) => match event {
                        SelectionEvent::Add => unsafe {
                            select(eid);
                        },
                        SelectionEvent::Change => unsafe {
                            let state = query_select(eid);
                            unselect_all();
                            set_select(eid, !state)
                        },
                        SelectionEvent::ToggleCollapse => unsafe {
                            if let Some(state) = ENTITY_HIERARCHY_OPEN.get_mut(&eid) {
                                *state = !*state;
                            }
                        },
                        SelectionEvent::Hovered => ui.ctx().highlight_widget(id),
                    },
                    _ => {}
                }
            })
            .show(ui, |ui| {
                self.children_ui(ui, depth, cursor_pos, font_size, font_scale)
            })
            .body_returned
            .unwrap_or(Action::Keep);

        unsafe {
            if let Some(state) = ENTITY_HIERARCHY_OPEN.get_mut(&eid) {
                self.open = *state;
            }
        }

        act
    }

    fn children_ui(
        &mut self,
        ui: &mut egui::Ui,
        depth: usize,
        cursor_pos: egui::Pos2,
        font_size: f32,
        font_scale: f32,
    ) -> Action {
        let close_rect = egui::Rect {
            min: egui::Pos2 {
                x: cursor_pos.x + 140f32,
                y: cursor_pos.y + 4f32,
            },
            max: egui::Pos2 {
                x: cursor_pos.x + 140f32 + 16f32,
                y: cursor_pos.y + 16f32 + 4f32,
            },
        };
        let close_label = egui::Rect {
            min: egui::Pos2 {
                x: cursor_pos.x + 140f32,
                y: cursor_pos.y + 2f32 + 4f32,
            },
            max: egui::Pos2 {
                x: cursor_pos.x + 140f32 + 16f32,
                y: cursor_pos.y + 16f32 + 4f32,
            },
        };

        if depth > 0 && ui.put(close_rect, egui::Button::new("")).clicked() {
            return Action::Delete;
        }

        if depth > 0 {
            ui.put(
                close_label,
                egui::Label::new(egui::RichText::new("⊗").color(ui.visuals().error_fg_color)),
            );
        }

        self.nodes = std::mem::take(&mut self.nodes)
            .into_iter()
            .enumerate()
            .filter_map(|(i, mut tree)| {
                if tree.ui_impl(
                    ui,
                    depth + 1,
                    &format!("Entity #{i}"),
                    font_size,
                    font_scale,
                ) == Action::Keep
                {
                    Some(tree)
                } else {
                    None
                }
            })
            .collect();

        if ui.button("+").clicked() {
            self.nodes.push(Tree::default());
        }

        Action::Keep
    }
}

use super::Panel;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SceneHierarchyPanel {
    pub open: bool,
    #[serde(skip)]
    font_size: f32,
    #[serde(skip)]
    font_scale: f32,
    #[serde(skip)]
    dimensions: egui::Vec2,
    #[serde(skip)]
    scene_hierarchy: Tree,
    #[serde(skip)]
    label: String,
    #[serde(skip)]
    first_frame: bool,
}

impl Default for SceneHierarchyPanel {
    fn default() -> Self {
        Self {
            open: false,
            font_size: 20.,
            font_scale: 1.,
            dimensions: egui::vec2(200., 400.),
            scene_hierarchy: Tree::default(),
            label: "Hierarchy".to_string(),
            first_frame: true,
        }
    }
}

impl Panel for SceneHierarchyPanel {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.open {
            egui::Window::new(
                egui::RichText::new("Hierarchy").size(self.font_size * self.font_scale),
            )
            .fixed_pos((40., 40.))
            .fixed_size(self.dimensions)
            .collapsible(false)
            .show(ctx, |ui| {
                self.scene_hierarchy_ui(ui);
            });

            if self.first_frame {
                self.first_frame = false;
            }
        }
    }
    #[allow(unused)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}

impl SceneHierarchyPanel {
    fn scene_hierarchy_ui(&mut self, ui: &mut egui::Ui) {
        self.scene_hierarchy.ui(ui, self.font_size, self.font_scale);
    }
}

unsafe fn toggle_collapse(id: usize) {
    if let Some(state) = ENTITY_HIERARCHY_OPEN.get_mut(&id) {
        *state = !*state;
    }
}

pub unsafe fn toggle_select(id: usize) {
    if let Some(state) = ENTITY_HIERARCHY_SELECTED.get_mut(&id) {
        *state = !*state;
    }
}

unsafe fn set_select(id: usize, state: bool) {
    if let Some(state_) = ENTITY_HIERARCHY_SELECTED.get_mut(&id) {
        *state_ = state;
    }
}

pub unsafe fn select(id: usize) {
    set_select(id, true);
}

unsafe fn unselect(id: usize) {
    set_select(id, false);
}

unsafe fn select_all() {
    for (key, val) in ENTITY_HIERARCHY_SELECTED.iter_mut() {
        *val = true;
    }
}

pub unsafe fn unselect_all() {
    for (key, val) in ENTITY_HIERARCHY_SELECTED.iter_mut() {
        *val = false;
    }
}

pub unsafe fn get_selected() -> Vec<usize> {
    let mut selected = Vec::new();
    for (key, val) in ENTITY_HIERARCHY_SELECTED.iter_mut() {
        if *val {
            selected.push(*key);
        }
    }
    selected
}

pub unsafe fn is_selected(id: usize) -> bool {
    let selected = get_selected();
    selected.contains(&id)
}

unsafe fn query_select(id: usize) -> bool {
    *ENTITY_HIERARCHY_SELECTED.get(&id).unwrap()
}
