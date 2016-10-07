/*

Rule 110 is a simple cellular automaton that is universal.

The problem is to find the reverse history from a final state,
up to a limited number of iterations.

*/

extern crate quickbacktrack;

use quickbacktrack::{BackTrackSolver, Puzzle, SolveSettings};

#[derive(Clone)]
pub struct Rule110 {
	pub cells: Vec<Vec<u8>>,
}

impl Rule110 {
    pub fn next(&mut self) -> Vec<u8> {
        let last = self.cells.last().unwrap();
        let n = last.len();
        let mut new_row = vec![0; n];
        for i in 0..n {
            let input = (
                if i > 0 { last[(i + n - 1)%n] } else { 1 },
                last[i],
                if i + 1 < n { last[i + 1] } else { 1 }
            );
            new_row[i] = rule(input);
        }
        new_row
    }

    /// Returns `false` when a contradition is found.
    /// This is checked by checking all affected cells in next step.
    pub fn is_satisfied(&self, pos: [usize; 2], val: u8) -> bool {
        let row = pos[0];
        let col = pos[1] as isize;

        if row + 1 < self.cells.len() {
            // Replace with new value if looking up cell at the location.
            let f = |ind: isize| {
                if ind < 0 { return 1; }
                if ind >= self.cells[row].len() as isize { 1 }
                else if ind == col { val }
                else { self.cells[row][ind as usize] }
            };
            for i in -1..2 {
                // Skip output cells at the edges.
                if col + i <= 0 ||
                   col + i >= self.cells[row].len() as isize {
                    continue;
                }

                let input = (
                    f(col + i - 1),
                    f(col + i),
                    f(col + i + 1),
                );

                let new_value = rule(input);
                let old_value = self.cells[row + 1][(col + i) as usize];
                match (new_value, old_value) {
                    (_, 0) => {}
                    (0, _) => {}
                    (a, b) if a == b => {}
                    (_, _) => return false,
                }
            }
        }

        // Check that previous row yields value.
        if row > 0 {
            let f = |ind: isize| {
                if ind < 0 { return 1; }
                if ind >= self.cells[row - 1].len() as isize { 1 }
                else { self.cells[row - 1][ind as usize] }
            };
            let input = (
                f(col - 1),
                f(col),
                f(col + 1),
            );
            match (rule(input), val) {
                (_, 0) => {}
                (0, _) => {}
                (a, b) if a == b => {}
                (_, _) => return false,
            }
        }

        true
    }

    pub fn possible(&self, pos: [usize; 2]) -> Vec<u8> {
        let mut res = vec![];
        for v in 1..3 {
            if self.is_satisfied(pos, v) {
                res.push(v);
            }
        }
        res
    }

    pub fn find_min_empty(&self) -> Option<[usize; 2]> {
        let mut min = None;
        let mut min_pos = None;
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j] == 0 {
                    let possible = self.possible([i, j]);
                    if min.is_none() || min.unwrap() >= possible.len() {
                        min = Some(possible.len());
                        min_pos = Some([i, j]);
                    }
                }
            }
        }
        return min_pos;
    }
}

/// Rule 110 extended with unknown inputs.
fn rule(state: (u8, u8, u8)) -> u8 {
    match state {
        (2, 2, 2) => 1,
        (2, 2, 1) => 2,
        (2, 1, 2) => 2,
        (2, 1, 1) => 1,
        (1, 2, 2) => 2,
        (1, 2, 1) => 2,
        (1, 1, 2) => 2,
        (1, 1, 1) => 1,

        // 1 unknown.
        (2, 2, 0) => 0,
        (2, 0, 2) => 0,
        (0, 2, 2) => 0,
        (2, 0, 1) => 0,
        (0, 2, 1) => 2,
        (2, 1, 0) => 0,
        (0, 1, 2) => 2,
        (0, 1, 1) => 1,
        (1, 2, 0) => 2,
        (1, 0, 2) => 2,
        (1, 0, 1) => 0,
        (1, 1, 0) => 0,

        // All with 2 unknowns or more has unknown result.
        (_, _, _) => 0,
    }
}

impl Puzzle for Rule110 {
    type Pos = [usize; 2];
    type Val = u8;

    fn solve_simple(&mut self) {}

    fn set(&mut self, pos: [usize; 2], val: u8) {
        self.cells[pos[0]][pos[1]] = val;
    }

    fn is_solved(&self) -> bool {
        for row in &self.cells {
            for col in row {
                if *col == 0 { return false; }
            }
        }
        true
    }

    fn remove(&mut self, other: &Rule110) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if other.cells[i][j] != 0 {
                    self.cells[i][j] = 0;
                }
            }
        }
    }

    fn print(&self) {
        println!("");
        for row in &self.cells {
            for cell in row {
                if *cell == 2 { print!("x"); }
                else if *cell == 1 { print!("_"); }
                else { print!(" "); }
            }
            println!("")
        }
        println!("");
    }
}

fn main() {
    /*
    let mut r = Rule110 {
        cells: vec![
            // _xx__xxxx_xx__x
            vec![1, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 2, 1, 1, 2]
        ]
    };
    let next = r.next();
    r.cells.push(next);
    r.print();
    return;
    */

    let x = Rule110 {
        cells: vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        ]
    };
    // println!("{}", x.is_satisfied([0, 1], 2));

    let settings = SolveSettings::new()
		.solve_simple(false)
		.debug(false)
		.difference(false)
		.sleep_ms(50)
	;
    let solver = BackTrackSolver::new(x, settings);
    let difference = solver.solve(|s| s.find_min_empty(), |s, p| s.possible(p))
		.expect("Expected solution").puzzle;
	println!("Solution:");
	difference.print();
    println!("{}\n{:?}", difference.cells.len(),
        difference.cells[2]);
}

pub fn example1() -> Rule110 {
    Rule110 {
        cells: vec![
            vec![1, 1, 1, 2, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![2, 2, 1, 2, 1],
        ]
    }
}

pub fn example2() -> Rule110 {
    Rule110 {
        cells: vec![
            vec![1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![2, 2, 1, 2, 1],
        ]
    }
}
