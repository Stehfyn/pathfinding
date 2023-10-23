//#[derive(panel_macros::GenerateUI)]
use super::pos2::Pos2;

#[derive(Clone, Copy)]
pub struct ComponentData<T> {
    pub name: &'static str,
    pub data: T,
}

impl<T: Default> Default for ComponentData<T> {
    fn default() -> Self {
        ComponentData {
            name: "",
            data: T::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ComponentWrapper<T> {
    pub component_data: ComponentData<T>,
}

impl<T: Default> Default for ComponentWrapper<T> {
    fn default() -> Self {
        ComponentWrapper {
            component_data: ComponentData::<T>::default(),
        }
    }
}

pub trait ComponentTrait<T>: Default {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
    fn default() -> Self;
}

impl<T> ComponentTrait<T> for ComponentWrapper<T>
where
    T: Clone + Default,
{
    fn get(&self) -> &T {
        &self.component_data.data
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.component_data.data
    }

    fn default() -> Self {
        ComponentWrapper::<T> {
            component_data: ComponentData::<T>::default(),
        }
    }
}

#[derive(panel_macros::GenerateUI, Clone, Copy)]
pub struct Transform2 {
    pub pos: Pos2,
}

impl Default for Transform2 {
    fn default() -> Self {
        Self {
            pos: Pos2::default(),
        }
    }
}
#[derive(panel_macros::GenerateUI, Clone, Copy)]
pub struct Color {
    pub col: egui::Color32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            col: egui::Color32::LIGHT_BLUE,
        }
    }
}

#[derive(panel_macros::GenerateUI, Clone)]
pub struct Mesh {
    pub mesh: Vec<Pos2>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self { mesh: Vec::new() }
    }
}

pub enum Component {
    Transform2(ComponentWrapper<Transform2>),
    Color(ComponentWrapper<Color>),
    Mesh(ComponentWrapper<Mesh>),
}
