/*

Rule 153 is a simple cellular automaton.

*/

extern crate quickbacktrack;

use quickbacktrack::*;

const BOUNDARY: bool = false;

#[derive(Clone)]
pub struct Rule153 {
	pub cells: Vec<Vec<u8>>,
}

impl Rule153 {
    pub fn next(&mut self) -> Vec<u8> {
        let last = self.cells.last().unwrap();
        let n = last.len();
        let mut new_row = vec![0; n];
        for i in 0..n {
            let input = (
                last[(i + n - 1) % n],
                last[i],
                last[(i + 1) % n]
            );
            new_row[i] = rule(input);
        }
        new_row
    }

    /// Returns `false` when a contradition is found.
    /// This is checked by checking all affected cells in next step.
    /// Also checks the previous step.
    pub fn is_satisfied(&self, pos: [usize; 2], val: u8) -> bool {
        let row = pos[0];
        let col = pos[1] as isize;

		if self.get(pos) != 0 {return self.get(pos) == val}

        let n = self.cells[row].len() as isize;
        if row + 1 < self.cells.len() {
            // Replace with new value if looking up cell at the location.
            let f = |ind: isize| {
				// println!("TEST col + ind {}", col + ind);
                let map_ind = ((col + ind + n) % n) as usize;
				if BOUNDARY && (col + ind < 0 || col + ind >= n) {1}
				else {
	                if map_ind == col as usize { val }
	                else {self.cells[row][map_ind]}
				}
            };
            // [o o x] [o x o] [x o o]
            for i in -1..2 {
                let input = (
                    f(i - 1),
                    f(i),
                    f(i + 1),
                );

                let col_next = ((col + n + i) % n) as usize;
                let new_value = rule(input);
                let old_value = if BOUNDARY && (col + i < 0 || col + i >= n) {1}
				                else {self.cells[row + 1][col_next]};
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
                let map_ind = ((col + ind + n) % n) as usize;
				if BOUNDARY && (col + ind < 0 || col + ind >= n) {1}
				else {self.cells[row - 1][map_ind]}
            };
            let input = (
                f(-1),
                f(0),
                f(1),
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
		if self.get(pos) != 0 {return res}
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

/// Rule 217 extended with unknown inputs.
fn rule(state: (u8, u8, u8)) -> u8 {
    match state {
        (2, 2, 2) => 2,
        (2, 2, 1) => 1,
        (2, 1, 2) => 1,
        (2, 1, 1) => 2,
        (1, 2, 2) => 2,
        (1, 2, 1) => 1,
        (1, 1, 2) => 1,
        (1, 1, 1) => 2,

        // 1 unknown.
        (2, 2, 0) => 0,
        (2, 0, 2) => 0,
		(0, 2, 2) => 2,
        (2, 0, 1) => 0,
        (0, 2, 1) => 1,
        (2, 1, 0) => 0,
        (0, 1, 2) => 1,
        (0, 1, 1) => 2,
        (1, 2, 0) => 0,
        (1, 0, 2) => 0,
        (1, 0, 1) => 0,
        (1, 1, 0) => 0,

        // All with 2 unknowns or more has unknown result.
        (_, _, _) => 0,
    }
}

impl Puzzle for Rule153 {
    type Pos = [usize; 2];
    type Val = u8;

    fn solve_simple<F: FnMut(&mut Self, Self::Pos, Self::Val)>(&mut self, mut f: F) {
        loop {
			let mut found_any = false;
			for i in 0..self.cells.len() {
				for j in 0..self.cells[i].len() {
					if self.cells[i][j] != 0 { continue; }
					let possible = self.possible([i, j]);
					if possible.len() == 1 {
						f(self, [i, j], possible[0]);
						found_any = true;
					}
				}
			}
			if !found_any { break; }
		}
    }

    fn set(&mut self, pos: [usize; 2], val: u8) {
        self.cells[pos[0]][pos[1]] = val;
    }

	fn get(&self, pos: [usize; 2]) -> u8 {
		self.cells[pos[0]][pos[1]]
	}

    fn is_solved(&self) -> bool {
        // All cells must be non-empty.
        for row in &self.cells {
            for col in row {
                if *col == 0 { return false; }
            }
        }

        // All cells must satisfy the constraints.
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if !self.is_satisfied([i, j], self.cells[i][j]) {
                    return false;
                }
            }
        }

        true
    }

    fn remove(&mut self, other: &Rule153) {
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
                if *cell == 2 { print!("o"); }
                else if *cell == 1 { print!("-"); }
                else { print!(" "); }
            }
            println!("")
        }
        println!("");
    }
}

fn main() {
    let x = example4();

	// Generate start choices.
	let mut start = vec![];
	for i in 0..x.cells.len() {
		for j in 0..x.cells[i].len() {
			start.push(([i, j], Rule153::possible(&x, [i, j])));
		}
	}

	let entropy_settings = EntropySolveSettings::new()
	 	.attempts(20000)
		.noise(0.5)
		.final_attempt(Some(Some(1000)));
	let settings = SolveSettings::new()
		.solve_simple(true)
		.debug(false)
		.difference(false)
		.sleep_ms(5)
		.max_iterations(100)
	;
    let mut solver = EntropyBackTrackSolver::new(x, start, entropy_settings, settings);
    let (i, solution) = solver.solve(Rule153::possible);
	println!("Attempts: {}", i);
	let solution = solution.expect("Expected solution");

	println!("Solution:");
	solution.puzzle.print();
	println!("Non-trivial moves: {}", solution.iterations);

}

pub fn example1() -> Rule153 {
	Rule153 {
        cells: vec![
			vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ]
    }
}

pub fn example2() -> Rule153 {
	Rule153 {
        cells: vec![
            vec![0, 0, 0, 0, 0, 0, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 1, 1, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 1, 2, 1, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    }
}

pub fn example3() -> Rule153 {
	Rule153 {
        cells: vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    }
}


pub fn example4() -> Rule153 {
	Rule153 {
        cells: vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2],
        ]
    }
}
