use poll_promise::Promise;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::atomic::AtomicUsize;

static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

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

    fn neighbors(&self) -> Vec<Pos2> {
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

#[derive(Eq, PartialEq)]
struct Reverse<T>(T);

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialOrd for Reverse<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct NavMesh {
    pub space_lut: HashMap<(i64, i64), bool>,
    pub min: Pos2,
    pub max: Pos2,
}

impl Default for NavMesh {
    fn default() -> Self {
        Self {
            space_lut: HashMap::default(),
            min: Pos2::default(),
            max: Pos2::default(),
        }
    }
}

impl NavMesh {
    pub fn set_grid_boundaries(&mut self, min: Pos2, max: Pos2) {
        self.min = min;
        self.max = max;
    }

    fn is_in_bounds(&self, pos: &Pos2) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x && pos.y >= self.min.y && pos.y <= self.max.y
    }

    pub fn set_space_lut(&mut self, space_lut: HashMap<(i64, i64), bool>) {
        self.space_lut = space_lut;
    }

    fn heuristic(&self, a: &Pos2, b: &Pos2) -> i64 {
        ((a.x - b.x).abs() + (a.y - b.y).abs()) as i64
    }

    pub fn async_a_star(&self, start: Pos2, end: Pos2) -> Option<Promise<Option<Vec<Pos2>>>> {
        let navmesh_clone = self.clone();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let thread_id = THREAD_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let thread_name = format!("a_star_{}", thread_id);
            return Some(Promise::spawn_thread(&thread_name, move || {
                navmesh_clone.a_star(start, end)
            }));
        }

        #[cfg(target_arch = "wasm32")]
        return Some(Promise::spawn_local(async move {
            navmesh_clone.a_star(start, end)
        }));
    }

    pub fn a_star(&self, start: Pos2, end: Pos2) -> Option<Vec<Pos2>> {
        if !self.space_lut.contains_key(&start.to_tuple())
            && !self.space_lut.contains_key(&end.to_tuple())
        {
            let mut open_set: BinaryHeap<Reverse<(i64, Pos2)>> = BinaryHeap::new();
            let mut came_from: HashMap<Pos2, Pos2> = HashMap::new();
            let mut g_score: HashMap<Pos2, i64> = HashMap::new();
            let mut f_score: HashMap<Pos2, i64> = HashMap::new();

            open_set.push(Reverse((0, start)));
            g_score.insert(start, 0);
            f_score.insert(start, self.heuristic(&start, &end));

            while let Some(Reverse((_, current))) = open_set.pop() {
                if current == end {
                    let mut path = vec![end];
                    while let Some(neighbor) = came_from.get(&path[path.len() - 1]) {
                        path.push(*neighbor);
                    }
                    path.reverse();
                    return Some(path);
                }

                for neighbor in current.neighbors() {
                    if !self.is_in_bounds(&neighbor)
                        || self.space_lut.contains_key(&neighbor.to_tuple())
                    {
                        continue;
                    }

                    let tentative_g_score = g_score.get(&current).unwrap_or(&i64::MAX) + 1;
                    if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i64::MAX) {
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);
                        f_score.insert(
                            neighbor,
                            tentative_g_score + self.heuristic(&neighbor, &end),
                        );
                        open_set.push(Reverse((f_score[&neighbor], neighbor)));
                    }
                }
            }
            None
        } else {
            None
        }
    }
}
