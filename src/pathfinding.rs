use crate::ecs::pos2::Pos2;
use poll_promise::Promise;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
static THREAD_COUNTER: AtomicUsize = AtomicUsize::new(0);

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

    pub fn async_waypointed_a_star(
        &self,
        start: Pos2,
        waypoints: Vec<Pos2>,
    ) -> Option<Promise<Option<Vec<Pos2>>>> {
        let navmesh_clone = self.clone();

        // Spawn a new thread or async task depending on the target architecture
        #[cfg(not(target_arch = "wasm32"))]
        {
            let thread_id = THREAD_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let thread_name = format!("waypointed_a_star_{}", thread_id);
            Some(Promise::spawn_thread(&thread_name, move || {
                // Call the waypointed_a_star function with the cloned navmesh, start, and waypoints
                navmesh_clone.waypointed_a_star(start, waypoints)
            }))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Some(Promise::spawn_local(async move {
                // Since spawn_local expects a Future, we have to use async block here.
                // The waypointed_a_star call is still synchronous, but we are in an async block.
                navmesh_clone.waypointed_a_star(start, waypoints)
            }))
        }
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
                    // Check if the movement was diagonal
                    let is_diagonal = (neighbor.x != current.x) && (neighbor.y != current.y);
                    let movement_cost = if is_diagonal { 14 } else { 10 }; // Use 14 and 10 as approximations for 1.4 and 1

                    let tentative_g_score =
                        g_score.get(&current).unwrap_or(&i64::MAX) + movement_cost;

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
    pub fn waypointed_a_star(&self, start: Pos2, waypoints: Vec<Pos2>) -> Option<Vec<Pos2>> {
        let mut total_path = Vec::new();
        let mut current_start = start;

        for end in waypoints.into_iter() {
            if self.space_lut.contains_key(&current_start.to_tuple())
                || self.space_lut.contains_key(&end.to_tuple())
            {
                // If any point is not traversable, return None
                return None;
            }

            let mut open_set: BinaryHeap<Reverse<(i64, Pos2)>> = BinaryHeap::new();
            let mut came_from: HashMap<Pos2, Pos2> = HashMap::new();
            let mut g_score: HashMap<Pos2, i64> = HashMap::new();
            let mut f_score: HashMap<Pos2, i64> = HashMap::new();

            open_set.push(Reverse((0, current_start)));
            g_score.insert(current_start, 0);
            f_score.insert(current_start, self.heuristic(&current_start, &end));

            while let Some(Reverse((_, current))) = open_set.pop() {
                if current == end {
                    let mut path = Vec::new();
                    let mut current_pos = current;
                    while let Some(&came_from_pos) = came_from.get(&current_pos) {
                        path.push(current_pos);
                        current_pos = came_from_pos;
                    }
                    path.push(current_start); // Add the start position to the path
                    path.reverse(); // Reverse the path to start->end order

                    if !total_path.is_empty() {
                        path.remove(0); // Remove the first point to avoid duplication
                    }

                    total_path.extend(path);
                    break;
                }

                for neighbor in current.neighbors() {
                    if !self.is_in_bounds(&neighbor)
                        || self.space_lut.contains_key(&neighbor.to_tuple())
                    {
                        continue;
                    }
                    let is_diagonal = (neighbor.x != current.x) && (neighbor.y != current.y);
                    let movement_cost = if is_diagonal { 14 } else { 10 };

                    let tentative_g_score =
                        g_score.get(&current).unwrap_or(&i64::MAX) + movement_cost;

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

            current_start = end; // The next start point is the current end point
        }

        if total_path.is_empty() {
            None // No path found
        } else {
            Some(total_path) // Return the concatenated path
        }
    }
}
