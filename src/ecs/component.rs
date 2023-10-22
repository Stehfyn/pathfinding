use super::pos2::Pos2;

pub struct ComponentStorage {
    name: String,
}

impl ComponentStorage {
    fn new(name: String) -> Self {
        Self { name: name }
    }
}

pub trait ComponentModifier {
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String);
}

impl ComponentModifier for ComponentStorage {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

pub struct Component(ComponentStorage);

pub struct TransformStorage {
    pos: Pos2,
}

pub trait TransformModifier {
    fn get_pos(&self) -> &Pos2;
    fn set_pos(&mut self, pos: Pos2);
}

impl TransformModifier for TransformStorage {
    fn get_pos(&self) -> &Pos2 {
        &self.pos
    }

    fn set_pos(&mut self, pos: Pos2) {
        self.pos = pos;
    }
}

pub struct Transform(Component, TransformStorage);

impl Default for Transform {
    fn default() -> Self {
        Self(
            Component(ComponentStorage {
                name: "Transform".to_string(),
            }),
            TransformStorage {
                pos: Pos2::default(),
            },
        )
    }
}

pub struct ColorStorage {
    col: egui::Color32,
}

pub trait ColorModifier {
    fn get_col(&self) -> &egui::Color32;
    fn set_col(&mut self, col: egui::Color32);
}

impl ColorModifier for ColorStorage {
    fn get_col(&self) -> &egui::Color32 {
        &self.col
    }

    fn set_col(&mut self, col: egui::Color32) {
        self.col = col;
    }
}
pub struct Color(Component, ColorStorage);

impl Default for Color {
    fn default() -> Self {
        Self(
            Component(ComponentStorage {
                name: "Transform".to_string(),
            }),
            ColorStorage {
                col: egui::Color32::RED,
            },
        )
    }
}

pub struct MeshStorage {
    mesh: Vec<Pos2>,
}

pub trait MeshModifier {
    fn get_mesh(&self) -> &Vec<Pos2>;
    fn set_mesh(&mut self, mesh: Vec<Pos2>);
}

impl MeshModifier for MeshStorage {
    fn get_mesh(&self) -> &Vec<Pos2> {
        &self.mesh
    }
    fn set_mesh(&mut self, mesh: Vec<Pos2>) {
        self.mesh = mesh
    }
}
pub struct Mesh(Component, MeshStorage);

impl Default for Mesh {
    fn default() -> Self {
        Self(
            Component(ComponentStorage {
                name: "Transform".to_string(),
            }),
            MeshStorage {
                mesh: Vec::default(),
            },
        )
    }
}

pub struct PotentialStorage {
    coeff: f32,
    base: f32,
    expon: f32,
}

pub trait PotentialModifier {
    fn get_coeff(&self) -> f32;
    fn set_coeff(&mut self, coeff: f32);
    fn get_base(&self) -> f32;
    fn set_base(&mut self, base: f32);
    fn get_expon(&self) -> f32;
    fn set_expon(&mut self, expon: f32);
}

impl PotentialModifier for PotentialStorage {
    fn get_coeff(&self) -> f32 {
        self.coeff
    }
    fn set_coeff(&mut self, coeff: f32) {
        self.coeff = coeff;
    }
    fn get_base(&self) -> f32 {
        self.base
    }
    fn set_base(&mut self, base: f32) {
        self.base = base;
    }
    fn get_expon(&self) -> f32 {
        self.expon
    }
    fn set_expon(&mut self, expon: f32) {
        self.expon = expon;
    }
}
pub struct Potential(Component, PotentialStorage);

impl Default for Potential {
    fn default() -> Self {
        Self(
            Component(ComponentStorage {
                name: "Potential Field".to_string(),
            }),
            PotentialStorage {
                coeff: 0.,
                base: 0.,
                expon: 0.,
            },
        )
    }
}
