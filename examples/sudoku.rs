/*

Sudoku is a puzzle where you fill numbers 1-9 until every row, column and group contains 1-9.

For more information, see https://en.wikipedia.org/wiki/Sudoku

An interesting thing with this example is the varying performance with different algorithms
for picking the next empty slot.

*/

extern crate quickbacktrack;

use quickbacktrack::{combine, BackTrackSolver, MultiBackTrackSolver, Puzzle, SolveSettings};

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

	pub fn find_empty(&self) -> Option<[usize; 2]> {
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 {
					return Some([x, y]);
				}
			}
		}
		return None;
	}

	pub fn find_min_empty(&self) -> Option<[usize; 2]> {
		let mut min = None;
		let mut min_pos = None;
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 {
					let possible = self.possible([x, y]);
					if possible.len() == 0 {return None};
					if min.is_none() || min.unwrap() > possible.len() {
						min = Some(possible.len());
						min_pos = Some([x, y]);
					}
				}
			}
		}
		return min_pos;
	}

	pub fn find_min_potential(&self) -> Option<[usize; 2]> {
		let mut min = None;
		let mut min_pos = None;
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 {
					let possible = self.possible_max_future([x, y]);
					if possible.len() > 0 {
						let mut experiment = self.clone();
						experiment.set([x, y], possible[0]);
						let potential = experiment.potential();
						if potential == 0 {continue;}
						if min.is_none() || min.unwrap() > potential {
							min = Some(potential);
							min_pos = Some([x, y]);
						}
					}
				}
			}
		}
		return min_pos;
	}

	pub fn find_freq_empty(&self) -> Option<[usize; 2]> {
		// Find the frequency of each numbers.
		let mut freq = [0; 9];
		let mut mask: [[u16; 9]; 9] = [[0; 9]; 9];
		for y in 0..9 {
			for x in 0..9 {
				if self.slots[y][x] == 0 {
					let possible = self.possible([x, y]);
					for p in &possible {
						freq[(*p - 1) as usize] += 1;
						mask[y][x] |= 1 << (*p - 1);
					}
				}
			}
		}

		// Find the number with least frequency, but bigger than 0.
		let mut min_freq = None;
		for i in 0..9 {
			if freq[i] > 0 && (min_freq.is_none() || freq[i] < freq[min_freq.unwrap()]) {
				min_freq = Some(i);
			}
		}
		let min_freq = if let Some(i) = min_freq {
			i
		} else {
			return self.find_empty();
		};

		for y in 0..9 {
			for x in 0..9 {
				let bit = 1 << min_freq;
				if self.slots[y][x] == 0 && (mask[y][x] & bit == bit) {
					return Some([x, y]);
				}
			}
		}
		return self.find_empty();
	}

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

	pub fn potential(&self) -> usize {
		let mut sum_possible = 0;
		for y in 0..9 {
			for x in 0..9 {
				let n = self.possible([x, y]).len();
				if n == 0 {
					return 0;
				}
				sum_possible += n;
			}
		}
		sum_possible
	}

	pub fn possible_max_future(&self, pos: [usize; 2]) -> Vec<u8> {
		let choices = self.possible(pos);
		if choices.len() == 1 {return choices;}
		let mut potential = vec![];
		'choice: for &choice in &choices {
			let mut experiment = self.clone();
			experiment.set(pos, choice);
			let mut sum_possible = 0;
			for y in 0..9 {
				for x in 0..9 {
					let n = experiment.possible([x, y]).len();
					if n == 0 {
						potential.push(0);
						continue 'choice;
					}
					sum_possible += n;
				}
			}
			potential.push(sum_possible);
		}

		let mut inds: Vec<usize> = (0..choices.len()).collect();
		inds.sort_by_key(|&i| potential[i]);
		inds.reverse();
		let mut new_choices = Vec::with_capacity(choices.len());
		for &ind in &inds {
			new_choices.push(choices[ind]);
		}
		new_choices
	}

	pub fn possible_maxmin_future(&self, pos: [usize; 2]) -> Vec<u8> {
		let choices = self.possible(pos);
		if choices.len() == 1 {return choices;}
		let mut potential = vec![];
		'choice: for &choice in &choices {
			let mut experiment = self.clone();
			experiment.set(pos, choice);
			let mut maxmin = None;
			for y in 0..9 {
				for x in 0..9 {
					let n = experiment.possible([x, y]).len();
					if n == 0 {
						potential.push(0);
						continue 'choice;
					}
					if maxmin.is_none() || maxmin.unwrap() > n {
						maxmin = Some(n);
					}
				}
			}
			potential.push(maxmin.unwrap_or(0));
		}

		let mut inds: Vec<usize> = (0..choices.len()).collect();
		inds.sort_by_key(|&i| potential[i]);
		inds.reverse();
		let mut new_choices = Vec::with_capacity(choices.len());
		for &ind in &inds {
			new_choices.push(choices[ind]);
		}
		new_choices
	}

	pub fn possible_max_future2(&self, pos: [usize; 2]) -> Vec<u8> {
		let choices = self.possible(pos);
		if choices.len() == 1 {return choices;}
		let mut potential = vec![];
		'choice: for &choice in &choices {
			let mut experiment = self.clone();
			experiment.set(pos, choice);
			let mut sum_possible = 0;
			for y in 0..9 {
				for x in 0..9 {
					let possible = experiment.possible([x, y]);
					if possible.len() == 0 {
						potential.push(0);
						continue 'choice;
					}
					let weight = possible.len();
					for &val in &possible {
						let mut experiment2 = experiment.clone();
						experiment2.set([x, y], val);
						for y2 in 0..9 {
							for x2 in 0..9 {
								sum_possible += weight * experiment2.possible([x2, y2]).len();
							}
						}
					}

				}
			}
			potential.push(sum_possible);
		}

		let mut inds: Vec<usize> = (0..choices.len()).collect();
		inds.sort_by_key(|&i| potential[i]);
		// inds.reverse();
		let mut new_choices = Vec::with_capacity(choices.len());
		for &ind in &inds {
			new_choices.push(choices[ind]);
		}
		new_choices
	}
}

fn main() {
	let x = example10();
	x.print();

	let settings = SolveSettings::new()
		.solve_simple(true)
		.debug(true)
		.difference(true)
		.sleep_ms(500)
	;

	let use_multi = false;

	let solution = if use_multi {
		let solver = MultiBackTrackSolver::new(settings);
		let strategies: Vec<(fn(&_) -> _, fn(&_, _) -> _)> = vec![
			(Sudoku::find_min_empty, Sudoku::possible),
			(Sudoku::find_min_empty, Sudoku::possible_max_future),
			(Sudoku::find_min_empty, Sudoku::possible_maxmin_future),
			(Sudoku::find_min_empty, Sudoku::possible_max_future2),
			(Sudoku::find_min_potential, Sudoku::possible),
			(Sudoku::find_min_potential, Sudoku::possible_max_future),
			(Sudoku::find_min_potential, Sudoku::possible_maxmin_future),
			(Sudoku::find_min_potential, Sudoku::possible_max_future2),
			(Sudoku::find_empty, Sudoku::possible),
			(Sudoku::find_empty, Sudoku::possible_max_future),
			(Sudoku::find_empty, Sudoku::possible_maxmin_future),
			(Sudoku::find_empty, Sudoku::possible_max_future2),
			(Sudoku::find_min_empty, |s: &Sudoku, p: [usize; 2]| combine(vec![
					s.possible(p),
					s.possible_max_future(p),
					s.possible_maxmin_future(p),
					s.possible_max_future2(p),
				])),
			(Sudoku::find_min_potential, |s: &Sudoku, p: [usize; 2]| combine(vec![
					s.possible(p),
					s.possible_max_future(p),
					s.possible_maxmin_future(p),
					s.possible_max_future2(p),
				])),
			(Sudoku::find_empty, |s: &Sudoku, p: [usize; 2]| combine(vec![
					s.possible(p),
					s.possible_max_future(p),
					s.possible_maxmin_future(p),
					s.possible_max_future2(p),
				])),
		];
		solver.solve(x, &strategies)
	} else {
		let solver = BackTrackSolver::new(x, settings);
		solver.solve(Sudoku::find_min_empty, Sudoku::possible)
	};

	let solution = solution.expect("Expected solution");

	println!("Difference:");
	solution.puzzle.print();
	println!("Non-trivial moves: {}", solution.iterations);
	println!("Strategy: {}", solution.strategy.unwrap_or(0));
}

/*

example		best
1			1
2			4
3			5
4			7
5			29
6			4
7			13
8			3
9			5
10			23

*/

pub fn example1() -> Sudoku {
	Sudoku {
		slots: [
			[0, 4, 1, 0, 9, 0, 2, 0, 0],
			[9, 2, 6, 5, 0, 0, 1, 0, 0],
			[0, 0, 0, 1, 0, 0, 3, 0, 6],
			[6, 3, 0, 0, 4, 0, 0, 8, 9],
			[7, 0, 0, 0, 0, 0, 0, 0, 1],
			[1, 5, 0, 0, 8, 0, 0, 2, 7],
			[2, 0, 9, 0, 0, 7, 0, 0, 0],
			[0, 0, 5, 0, 0, 8, 9, 1, 2],
			[0, 0, 3, 0, 1, 0, 7, 5, 0],
		]
	}
}

pub fn example2() -> Sudoku {
	Sudoku {
		slots: [
			// [8, 3, 0, 0, 0, 0, 7, 0, 0],
			// [0, 0, 6, 0, 3, 4, 0, 2, 0],
			// [4, 7, 0, 9, 0, 0, 0, 6, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 3, 4, 0, 0, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],

			[9, 6, 0, 0, 5, 0, 0, 8, 7],
			[2, 0, 0, 0, 0, 0, 0, 0, 6],
			[7, 1, 0, 0, 2, 0, 0, 4, 5],

			[0, 2, 0, 0, 0, 9, 0, 7, 8],
			[0, 4, 0, 6, 1, 0, 5, 0, 0],
			[0, 0, 8, 0, 0, 0, 0, 1, 3],
		]
	}
}

pub fn example3() -> Sudoku {
	Sudoku {
		slots: [
			[0, 0, 3, 2, 0, 1, 0, 5, 0],
			[0, 0, 0, 8, 0, 0, 9, 0, 0],
			[0, 4, 5, 0, 0, 3, 0, 0, 1],

			[0, 0, 7, 0, 0, 0, 0, 0, 6],
			[4, 0, 0, 0, 0, 0, 0, 0, 8],
			[6, 0, 0, 0, 0, 0, 3, 0, 0],

			[5, 0, 0, 7, 0, 0, 8, 9, 0],
			[0, 0, 1, 0, 0, 9, 0, 0, 0],
			[0, 6, 0, 5, 0, 8, 2, 0, 0],
		]
	}
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

pub fn example5() -> Sudoku {
	Sudoku {
		slots: [
			[8, 0, 0, 6, 9, 0, 3, 0, 0],
			[0, 0, 1, 0, 0, 8, 0, 0, 0],
			[0, 3, 0, 5, 0, 0, 0, 9, 0],

			[1, 0, 0, 0, 0, 0, 4, 0, 0],
			[7, 0, 0, 0, 8, 0, 0, 0, 2],
			[0, 0, 6, 0, 0, 0, 0, 0, 1],

			[0, 1, 0, 0, 0, 2, 0, 3, 0],
			[0, 0, 0, 8, 0, 0, 2, 0, 0],
			[0, 0, 2, 0, 7, 6, 0, 0, 4],
		]
	}
}

pub fn example6() -> Sudoku {
	Sudoku {
		slots: [
			[0, 5, 0, 9, 7, 0, 2, 0, 0],
			[0, 0, 8, 0, 0, 4, 0, 0, 3],
			[1, 4, 0, 0, 0, 0, 0, 0, 0],

			[2, 0, 0, 0, 9, 0, 0, 0, 0],
			[0, 1, 0, 3, 0, 2, 0, 5, 0],
			[0, 0, 0, 0, 5, 0, 0, 0, 8],

			[0, 0, 0, 0, 0, 0, 0, 1, 2],
			[9, 0, 0, 8, 0, 0, 7, 0, 0],
			[0, 0, 1, 0, 4, 7, 0, 3, 0],
		]
	}
}

pub fn example7() -> Sudoku {
	Sudoku {
		slots: [
			[4, 0, 0, 7, 8, 0, 0, 0, 2],
			[8, 5, 0, 0, 0, 0, 0, 3, 0],
			[0, 0, 0, 0, 0, 2, 6, 0, 0],

			[0, 2, 0, 0, 0, 0, 0, 0, 0],
			[0, 3, 0, 5, 0, 6, 0, 1, 0],
			[0, 0, 0, 0, 0, 0, 0, 4, 0],

			[0, 0, 5, 6, 0, 0, 0, 0, 0],
			[0, 9, 0, 0, 0, 0, 0, 6, 1],
			[2, 0, 0, 0, 7, 9, 0, 0, 5],
		]
	}
}

pub fn example8() -> Sudoku {
	Sudoku {
		slots: [
			[0, 0, 0, 4, 0, 0, 0, 0, 7],
			[1, 0, 6, 0, 0, 0, 9, 0, 5],
			[0, 4, 0, 9, 0, 0, 0, 0, 8],

			[0, 0, 7, 0, 8, 0, 0, 6, 0],
			[0, 0, 0, 7, 0, 9, 0, 0, 0],
			[0, 8, 0, 0, 3, 0, 1, 0, 0],

			[9, 0, 0, 0, 0, 5, 0, 8, 0],
			[5, 0, 1, 0, 0, 0, 2, 0, 6],
			[8, 0, 0, 0, 0, 2, 0, 0, 0],
		]
	}
}

pub fn example9() -> Sudoku {
	Sudoku {
		slots: [
			[0, 0, 0, 0, 0, 0, 0, 0, 3],
			[9, 3, 0, 8, 0, 0, 0, 0, 5],
			[0, 0, 0, 0, 7, 0, 6, 1, 0],

			[0, 0, 0, 0, 2, 1, 5, 0, 0],
			[0, 6, 0, 4, 0, 8, 0, 7, 0],
			[0, 0, 4, 7, 9, 0, 0, 0, 0],

			[0, 8, 5, 0, 3, 0, 0, 0, 0],
			[1, 0, 0, 0, 0, 2, 0, 8, 9],
			[6, 0, 0, 0, 0, 0, 0, 0, 0],
		]
	}
}

pub fn example10() -> Sudoku {
	Sudoku {
		slots: [
			[0, 2, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 4, 5, 0, 0],
			[0, 6, 0, 0, 0, 0, 0, 0, 0],

			[0, 0, 4, 0, 0, 0, 0, 0, 0],
			[9, 0, 3, 0, 1, 0, 0, 7, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],

			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 8, 0, 0, 0, 0, 3, 0, 0],
			[0, 0, 0, 1, 0, 0, 0, 9, 0],
		]
	}
}
