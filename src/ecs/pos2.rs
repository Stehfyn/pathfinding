#[derive(
    Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct Pos2 {
    pub x: i64,
    pub y: i64,
}

impl Default for Pos2 {
    fn default() -> Self {
        Self { x: 100, y: 100 }
    }
}

impl Pos2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x: x, y: y }
    }

    pub fn from_min(rect: &egui::Rect) -> Self {
        Self {
            x: rect.min.x as i64,
            y: rect.min.y as i64,
        }
    }
    pub fn from_max(rect: &egui::Rect) -> Self {
        Self {
            x: rect.max.x as i64,
            y: rect.max.y as i64,
        }
    }
}

impl Pos2 {
    pub fn to_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn neighbors(&self) -> Vec<Pos2> {
        vec![
            Pos2 {
                x: self.x + 1,
                y: self.y,
            },
            Pos2 {
                x: self.x - 1,
                y: self.y,
            },
            Pos2 {
                x: self.x,
                y: self.y + 1,
            },
            Pos2 {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}
