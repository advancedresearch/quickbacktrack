/*

Traveling Salesman Problem

Find a closed route between cities with the shortest length.

This solver looks for the choice of roads that has the greatest
potential for reducing distance compared to the next best alternative.
Looks at pair of roads instead of considering each single road.
The choice of roads can not lead to cycle.

The algorithm is exact, and therefore not efficient.

*/

extern crate quickbacktrack;

use std::sync::Arc;
use quickbacktrack::{BackTrackSolver, Puzzle, SolveSettings};

#[derive(Clone, Debug)]
pub struct Tsp {
    /// Choose a pair of roads for each city.
    pub slots: Vec<Option<(usize, usize)>>,
    /// Distances between cities.
    /// Stores `from` in first index, `to` in second index.
    pub distances: Arc<Vec<Vec<f64>>>,
    /// Find a distance less than the target.
    pub target: Option<f64>,
}

impl Tsp {
    pub fn new_2d(map: &Vec<Vec<u8>>) -> Tsp {
        let mut cities: Vec<(usize, usize)> = vec![];
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] != 0 {
                    cities.push((i, j));
                }
            }
        }

        let mut distances: Vec<Vec<f64>> = vec![];
        for a in &cities {
            let mut a_distances = vec![];
            for b in &cities {
                let dx = b.0 as f64 - a.0 as f64;
                let dy = b.1 as f64 - a.1 as f64;
                a_distances.push((dx * dx + dy * dy).sqrt());
            }
            distances.push(a_distances);
        }

        Tsp {
            slots: vec![None; cities.len()],
            distances: Arc::new(distances),
            target: None,
        }
    }

    /// Get possible choices, sorted by local distance.
    pub fn possible(&self, pos: usize) -> Vec<Option<(usize, usize)>> {
        if self.slots[pos].is_some() { return vec![self.slots[pos]]; }
        if let Some(target) = self.target {
            if target <= self.distance() { return vec![]; }
        }

        let n = self.slots.len();
        let mut res: Vec<Option<(usize, usize)>> = vec![];
        let mut local_distances: Vec<(usize, f64)> = vec![];
        for i in 0..n {
            if i == pos { continue; }

            // Other cities must point to this edge.
            if let Some((i_a, i_b)) = self.slots[i] {
                if i_a != pos && i_b != pos { continue; }
            }

            'j: for j in i + 1..n {
                if j == pos { continue; }

                // Other cities must point to this edge.
                if let Some((j_a, j_b)) = self.slots[j] {
                    if j_a != pos && j_b != pos { continue; }
                }

                // Check that each city is only referenced twice.
                let count_i = self.slots.iter()
                    .filter(|&&x| {
                        if let Some((x_a, x_b)) = x {
                            x_a == i || x_b == i
                        } else { false }
                    }).count();
                if count_i >= 2 { continue; }

                let count_j = self.slots.iter()
                    .filter(|&&x| {
                        if let Some((x_a, x_b)) = x {
                            x_a == j || x_b == j
                        } else { false }
                    }).count();
                if count_j >= 2 { continue; }

                // Seems sufficient to point other slots to this.
                /*
                let mut visited: HashSet<usize> = HashSet::new();
                visited.insert(pos);
                visited.insert(i);
                visited.insert(j);
                if self.detect_loop(&mut visited, j) { continue 'j; }
                */

                // All other slots that point to this must
                // be pointed back to.
                for (ind, s) in self.slots.iter().enumerate() {
                    if let &Some((a, b)) = s {
                        if a == pos || b == pos {
                            if i != ind && j != ind {
                                continue 'j;
                            }
                        }
                    }
                }

                local_distances.push((res.len(),
                    self.distances[pos][i] + self.distances[pos][j]));
                res.push(Some((i, j)));
            }
        }

        if res.len() > 2 {
            use std::cmp::PartialOrd;

            // Try the pair by order of local distance.
            local_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            res = local_distances.iter().rev().map(|p| res[p.0]).collect();
        }

        res
    }

    /*
    fn detect_loop(&self, visited: &mut HashSet<usize>, pos: usize) -> bool {
        if let Some((a, b)) = self.slots[pos] {
            if !visited.contains(&a) {
                visited.insert(a);
                self.detect_loop(visited, a)
            } else if !visited.contains(&b) {
                visited.insert(b);
                self.detect_loop(visited, b)
            } else {
                true
            }
        } else {
            false
        }
    }
    */

    pub fn find_empty(&self) -> Option<usize> {
        for i in 0..self.slots.len() {
            if self.slots[i].is_none() { return Some(i); }
        }
        None
    }

    // Pick empty slot with maximum potential of reducing distance
    // between first and second choice.
    pub fn find_min_empty(&self) -> Option<usize> {
        let mut max_potential: Option<f64> = None;
        let mut min_i: Option<usize> = None;
        for i in 0..self.slots.len() {
            if self.slots[i].is_some() { continue; }

            let possible = self.possible(i);
            if possible.len() == 0 { return None; }
            if possible.len() == 1 { return Some(i); }
            if possible.len() >= 2 {
                let n = possible.len();
                let (aa, ab) = possible[n - 1].unwrap();
                let (ba, bb) = possible[n - 2].unwrap();
                let potential =
                    -self.distances[i][aa] +
                    -self.distances[i][ab] +
                    self.distances[i][ba] +
                    self.distances[i][bb];
                if max_potential.is_none() ||
                    max_potential.unwrap() < potential {
                    min_i = Some(i);
                    max_potential = Some(potential);
                }
            }
        }
        min_i
    }

    pub fn distance(&self) -> f64 {
        use std::collections::HashSet;
        use std::cmp::{max, min};

        let mut counted: HashSet<(usize, usize)> = HashSet::new();
        let mut dist = 0.0;
        for i in 0..self.slots.len() {
            if let Some((a, b)) = self.slots[i] {
                let i_a = (min(i, a), max(i, a));
                if !counted.contains(&i_a) {
                    counted.insert(i_a);
                    dist += self.distances[i][a];
                }

                let i_b = (min(i, b), max(i, b));
                if !counted.contains(&i_b) {
                    counted.insert(i_b);
                    dist += self.distances[i][b];
                }
            }
        }
        dist
    }

    /// Computes lower bound by summing the minimum pair of distances
    /// from each city and then divide by 2.
    /// When the optimal route equals the lower bound, each edge
    /// is counted twice, so therefore we divide by 2.
    pub fn lower_bound(&self) -> f64 {
        let mut sum = 0.0;
        for s in 0..self.slots.len() {
            let mut min_dist: Option<f64> = None;
            for i in 0..self.slots.len() {
                for j in i + 1..self.slots.len() {
                    let dist = self.distances[s][i] + self.distances[s][j];
                    if min_dist.is_none() || min_dist.unwrap() > dist {
                        min_dist = Some(dist);
                    }
                }
            }
            sum += min_dist.unwrap_or(0.0);
        }
        sum / 2.0
    }

    pub fn upper_bound(&self) -> f64 {
        let mut sum = 0.0;
        for s in 0..self.slots.len() {
            let mut max_dist: Option<f64> = None;
            for i in 0..self.slots.len() {
                for j in i + 1..self.slots.len() {
                    let dist = self.distances[s][i] + self.distances[s][j];
                    if max_dist.is_none() || max_dist.unwrap() < dist {
                        max_dist = Some(dist);
                    }
                }
            }
            sum += max_dist.unwrap_or(0.0);
        }
        sum / 2.0
    }
}

impl Puzzle for Tsp {
    type Pos = usize;
    type Val = Option<(usize, usize)>;

    fn solve_simple(&mut self) {}

    fn set(&mut self, pos: usize, val: Option<(usize, usize)>) {
        self.slots[pos] = val;
    }

    fn print(&self) {
        println!("{:?}", self.slots);
        println!("Distance {}", self.distance());
    }

    fn remove(&mut self, other: &Tsp) {
        for i in 0..self.slots.len() {
            if other.slots[i].is_some() {
                self.slots[i] = None;
            }
        }
    }

    fn is_solved(&self) -> bool {
        if let Some(target) = self.target {
            if target <= self.distance() {
                return false;
            }
        }

        self.slots.iter().all(|d| d.is_some())
    }
}

fn main() {
    let x = Tsp::new_2d(&vec![
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0],
        ]);

    // Compute lower bound.
    println!("Lower bound: {}", x.lower_bound());
    println!("Upper bound: {}", x.upper_bound());

    let settings = SolveSettings::new()
		.solve_simple(false)
		.debug(false)
		.difference(true)
		.sleep_ms(500)
	;

	let solver = BackTrackSolver::new(x, settings);
	let difference = solver.solve(|s| s.find_min_empty(), |s, p| s.possible(p))
		.expect("Expected solution").puzzle;
	println!("Difference:");
	difference.print();
}
