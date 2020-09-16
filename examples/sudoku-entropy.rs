/*

Sudoku example using entropy solver.

The entropy solver can learn from previous attempts.

*/

extern crate quickbacktrack;

use quickbacktrack::{EntropyBackTrackSolver, Puzzle, EntropySolveSettings, SolveSettings};

#[derive(Clone)]
pub struct Sudoku {
	pub slots: [[u8; 9]; 9],
}

impl Puzzle for Sudoku {
	type Pos = [usize; 2];
	type Val = u8;

	fn solve_simple<F: FnMut(&mut Self, Self::Pos, Self::Val)>(&mut self, mut f: F) {
		loop {
			let mut found_any = false;
			for y in 0..9 {
				for x in 0..9 {
					if self.slots[y][x] != 0 { continue; }
					let possible = self.possible([x, y]);
					if possible.len() == 1 {
						f(self, [x, y], possible[0]);
						found_any = true;
					}
				}
			}
			if !found_any { break; }
		}
	}

	fn set(&mut self, pos: [usize; 2], val: u8) {
		self.slots[pos[1]][pos[0]] = val;
	}

	fn get(&self, pos: [usize; 2]) -> u8 {
		self.slots[pos[1]][pos[0]]
	}

	fn remove(&mut self, other: &Sudoku) {
		for y in 0..9 {
			for x in 0..9 {
				if other.slots[y][x] != 0 {
					self.slots[y][x] = 0;
				}
			}
		}
	}

	fn print(&self) {
		println!(" ___ ___ ___");
		for y in 0..9 {
			print!("|");
			for x in 0..9 {
				let v = self.slots[y][x];
				if v == 0 {
					print!(" ");
				} else {
					print!("{}", self.slots[y][x]);
				}
				if x % 3 == 2 {
					print!("|");
				}
			}
			println!("");
			if y % 3 == 2 {
				println!(" ---+---+---");
			}
		}
	}

	fn is_solved(&self) -> bool {
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 { return false; }
			}
		}
		return true;
	}
}

impl Sudoku {
	pub fn possible(&self, pos: [usize; 2]) -> Vec<u8> {
		let mut res = vec![];
		if self.slots[pos[1]][pos[0]] != 0 {
			return res;
		}
		'next_val: for v in 1..10 {
			for x in 0..9 {
				if self.slots[pos[1]][x] == v {
					continue 'next_val;
				}
				if self.slots[x][pos[0]] == v {
					continue 'next_val;
				}
			}
			let block_x = 3 * (pos[0] / 3);
			let block_y = 3 * (pos[1] / 3);
			for y in block_y..block_y + 3 {
				for x in block_x..block_x + 3 {
					if self.slots[y][x] == v {
						continue 'next_val;
					}
				}
			}
			res.push(v);
		}
		return res;
	}
}

fn main() {
	let x = example4();
	x.print();

	let settings = SolveSettings::new()
		.solve_simple(true)
		.debug(false)
		.difference(true)
		.sleep_ms(500)
		.max_iterations(100)
	;
	let entropy_settings = EntropySolveSettings::new()
	 	.attempts(200)
		.noise(0.5)
		.final_attempt(Some(None));

	// Generate start choices.
	let mut start = vec![];
	for i in 0..9 {
		for j in 0..9 {
			start.push(([i, j], Sudoku::possible(&x, [i, j])));
		}
	}

	let mut solver = EntropyBackTrackSolver::new(x, start, entropy_settings, settings);

	let (i, solution) = solver.solve(Sudoku::possible);
	println!("Attempts: {}", i);
	let solution = solution.expect("Expected solution");

	println!("Difference:");
	solution.puzzle.print();
	println!("Non-trivial moves: {}", solution.iterations);
	println!("Strategy: {}", solution.strategy.unwrap_or(0));
}

pub fn example4() -> Sudoku {
	Sudoku {
		slots: [
			[0, 5, 0, 0, 9, 6, 0, 7, 0],
			[2, 0, 9, 8, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 6, 0, 1],

			[0, 1, 0, 0, 6, 2, 0, 0, 5],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[8, 0, 0, 5, 1, 0, 0, 6, 0],

			[4, 0, 1, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 7, 3, 0, 9],
			[0, 9, 0, 1, 8, 0, 0, 2, 0],
		]
	}
}
