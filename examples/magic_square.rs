/*

Fill an odd NxN square with numbers 1-(N*N) such that
the sum on each row and column and across diagonals
is the same everywhere.

A partial sum is when you add things together and some
of the numbers are unknown, but you know the sum must
be at least something because you do not add negative numbers.

Here I try an approach where the solver focuses on
a position that has the largest partial sum.

*/

extern crate quickbacktrack;

use quickbacktrack::{BackTrackSolver, Puzzle, SolveSettings};

#[derive(Clone)]
pub struct MagicSquare {
    pub slots: Vec<Vec<u16>>,
    pub exact: u16,
}

impl Puzzle for MagicSquare {
    type Pos = [usize; 2];
    type Val = u16;

    fn solve_simple(&mut self) {}

    fn set(&mut self, pos: [usize; 2], val: u16) {
        self.slots[pos[1]][pos[0]] = val;
    }

    fn remove(&mut self, other: &MagicSquare) {
        let n = self.slots.len();
        for y in 0..n {
            for x in 0..n {
                if other.slots[y][x] != 0 {
					self.slots[y][x] = 0;
				}
            }
        }
    }

    fn print(&self) {
        let n = self.slots.len();
        for y in 0..n {
            for x in 0..n {
                let val = self.slots[y][x];
                if val == 0 {
                    print!("  ,")
                } else {
                    if val < 10 {
                        print!(" {},", val);
                    } else {
                        print!("{},", val);
                    }
                }
            }
            println!("");
        }
    }

    fn is_solved(&self) -> bool {
        let n = self.slots.len();

        for y in 0..n {
            for x in 0..n {
                if self.slots[y][x] == 0 {return false;}
            }
        }

        let mut find_sum: Option<u16> = None;

        for y in 0..n {
            let mut sum = 0;
            for x in 0..n {
                sum += self.slots[y][x];
            }

            if let Some(find_sum) = find_sum {
                if sum != find_sum {return false;}
            } else {
                find_sum = Some(sum);
            }
        }

        let mut sum1 = 0;
        let mut sum2 = 0;
        for i in 0..n {
            sum1 += self.slots[i][i];
            sum2 += self.slots[i][n - 1 - i];
        }

        if let Some(find_sum) = find_sum {
            if sum1 != find_sum {return false;}
            if sum2 != find_sum {return false;}
        }

        true
    }
}

impl MagicSquare {
    pub fn new(n: u16) -> MagicSquare {
        MagicSquare {
            slots: vec![vec![0; n as usize]; n as usize],
            exact: (n * n + 1) / 2 * n,
        }
    }

    pub fn find_empty(&self) -> Option<[usize; 2]> {
        let n = self.slots.len();
        for y in 0..n {
            for x in 0..n {
                if self.slots[y][x] == 0 {return Some([x, y]);}
            }
        }
        None
    }

    // Seems to have no improvement over `find_empty`.
    pub fn find_min_empty(&self) -> Option<[usize; 2]> {
		let mut min = None;
		let mut min_pos = None;
        let n = self.slots.len();
		for y in 0..n {
			for x in 0..n {
				if self.slots[y][x] == 0 {
					let possible = self.possible([x, y]);
					if min.is_none() || min.unwrap() > possible.len() {
						min = Some(possible.len());
						min_pos = Some([x, y]);
					}
				}
			}
		}
		return min_pos;
	}

    pub fn find_max_partial_sum_empty(&self) -> Option<[usize; 2]> {
        use PartialSum::*;

        let mut max = None;
        let mut max_pos = None;
        let n = self.slots.len();
        for y in 0..n {
            for x in 0..n {
                if self.slots[y][x] == 0 {
                    let mut sum = 0;
                    let mut any_zero = false;
                    for i in 0..n {
                        let val = self.slots[i][x];
                        if val == 0 {
                            any_zero = true;
                            sum += 1;
                        } else {
                            sum += val;
                        }
                    }
                    match PartialSum::new(sum, any_zero) {
                        Exact(_) => unreachable!(),
                        AtLeast(v) => {
                            if max.is_none() || max.unwrap() < v {
                                max = Some(v);
                                max_pos = Some([x, y]);
                            }
                        }
                    }

                    let mut sum = 0;
                    let mut any_zero = false;
                    for i in 0..n {
                        let val = self.slots[y][i];
                        if val == 0 {
                            any_zero = true;
                            sum += 1;
                        } else {
                            sum += val;
                        }
                    }
                    match PartialSum::new(sum, any_zero) {
                        Exact(_) => unreachable!(),
                        AtLeast(v) => {
                            if max.is_none() || max.unwrap() < v {
                                max = Some(v);
                                max_pos = Some([x, y]);
                            }
                        }
                    }

                    if x == y {
                        let mut sum = 0;
                        let mut any_zero = false;
                        for i in 0..n {
                            let val = self.slots[i][i];
                            if val == 0 {
                                any_zero = true;
                                sum += 1;
                            } else {
                                sum += val;
                            }
                        }
                        match PartialSum::new(sum, any_zero) {
                            Exact(_) => unreachable!(),
                            AtLeast(v) => {
                                if max.is_none() || max.unwrap() < v {
                                    max = Some(v);
                                    max_pos = Some([x, y]);
                                }
                            }
                        }
                    }

                    if n - 1 - x == y {
                        let mut sum = 0;
                        let mut any_zero = false;
                        for i in 0..n {
                            let val = self.slots[i][n - 1 - i];
                            if val == 0 {
                                any_zero = true;
                                sum += 1;
                            } else {
                                sum += val;
                            }
                        }
                        match PartialSum::new(sum, any_zero) {
                            Exact(_) => unreachable!(),
                            AtLeast(v) => {
                                if max.is_none() || max.unwrap() < v {
                                    max = Some(v);
                                    max_pos = Some([x, y]);
                                }
                            }
                        }
                    }
                }
            }
        }
        return max_pos;
    }

    // Worse than `find_max_partial_sum_empty`.
    pub fn find_min_partial_sum_empty(&self) -> Option<[usize; 2]> {
        use PartialSum::*;

        let mut min = None;
        let mut min_pos = None;
        let n = self.slots.len();
        for y in 0..n {
            for x in 0..n {
                if self.slots[y][x] == 0 {
                    let mut sum = 0;
                    let mut any_zero = false;
                    for i in 0..n {
                        let val = self.slots[i][x];
                        if val == 0 {
                            any_zero = true;
                            sum += 1;
                        } else {
                            sum += val;
                        }
                    }
                    match PartialSum::new(sum, any_zero) {
                        Exact(_) => unreachable!(),
                        AtLeast(v) => {
                            if min.is_none() || min.unwrap() > v {
                                min = Some(v);
                                min_pos = Some([x, y]);
                            }
                        }
                    }

                    let mut sum = 0;
                    let mut any_zero = false;
                    for i in 0..n {
                        let val = self.slots[y][i];
                        if val == 0 {
                            any_zero = true;
                            sum += 1;
                        } else {
                            sum += val;
                        }
                    }
                    match PartialSum::new(sum, any_zero) {
                        Exact(_) => unreachable!(),
                        AtLeast(v) => {
                            if min.is_none() || min.unwrap() > v {
                                min = Some(v);
                                min_pos = Some([x, y]);
                            }
                        }
                    }

                    if x == y {
                        let mut sum = 0;
                        let mut any_zero = false;
                        for i in 0..n {
                            let val = self.slots[i][i];
                            if val == 0 {
                                any_zero = true;
                                sum += 1;
                            } else {
                                sum += val;
                            }
                        }
                        match PartialSum::new(sum, any_zero) {
                            Exact(_) => unreachable!(),
                            AtLeast(v) => {
                                if min.is_none() || min.unwrap() > v {
                                    min = Some(v);
                                    min_pos = Some([x, y]);
                                }
                            }
                        }
                    }

                    if n - 1 - x == y {
                        let mut sum = 0;
                        let mut any_zero = false;
                        for i in 0..n {
                            let val = self.slots[i][n - 1 - i];
                            if val == 0 {
                                any_zero = true;
                                sum += 1;
                            } else {
                                sum += val;
                            }
                        }
                        match PartialSum::new(sum, any_zero) {
                            Exact(_) => unreachable!(),
                            AtLeast(v) => {
                                if min.is_none() || min.unwrap() > v {
                                    min = Some(v);
                                    min_pos = Some([x, y]);
                                }
                            }
                        }
                    }
                }
            }
        }
        return min_pos;
    }

    pub fn possible(&self, pos: [usize; 2]) -> Vec<u16> {
        let mut res = vec![];
		if self.slots[pos[1]][pos[0]] != 0 {
			res.push(self.slots[pos[1]][pos[0]]);
			return res;
		}

        match self.find_partial_sum() {
            Some(PartialSum::Exact(sum)) => {
                if self.exact != sum {return vec![];}
            }
            Some(PartialSum::AtLeast(sum)) => {
                if self.exact < sum {return vec![];}
            }
            None => {
                // If partial sum is inconsistent, then there are no options.
                return vec![];
            }
        }

        let n = self.slots.len();
        'outer: for i in 0..n * n {
            let val = (i + 1) as u16;
            for y in 0..n {
                for x in 0..n {
                    if [x, y] == pos {continue;}
                    if self.slots[y][x] == val {continue 'outer;}
                }
            }
            res.push(val);
        }
        res
    }

    pub fn find_partial_sum(&self) -> Option<PartialSum> {
        let mut find_sum: Option<PartialSum> = None;
        let n = self.slots.len();

        for i in 0..n {
            let mut sum = 0;
            let mut any_zero = false;
            for x in 0..n {
                let val = self.slots[i][x];
                if val == 0 {
                    any_zero = true;
                    sum += 1;
                } else {
                    sum += val;
                }
            }
            let row = PartialSum::new(sum, any_zero);
            find_sum = if let Some(ref find_sum) = find_sum {
                if let Some(new_find_sum) = find_sum.improve(&row) {
                    Some(new_find_sum)
                } else {
                    return None;
                }
            } else {
                Some(row)
            };

            let mut sum = 0;
            let mut any_zero = false;
            for y in 0..n {
                let val = self.slots[y][i];
                if val == 0 {
                    any_zero = true;
                    sum += 1;
                } else {
                    sum += val;
                }
            }
            let col = PartialSum::new(sum, any_zero);
            find_sum = if let Some(ref find_sum) = find_sum {
                if let Some(new_find_sum) = find_sum.improve(&col) {
                    Some(new_find_sum)
                } else {
                    return None;
                }
            } else {
                Some(col)
            };
        }

        let mut sum = 0;
        let mut any_zero = false;
        for i in 0..n {
            let val = self.slots[i][i];
            if val == 0 {
                any_zero = true;
                sum += 1;
            } else {
                sum += val;
            }
        }
        let diag1 = PartialSum::new(sum, any_zero);
        find_sum = if let Some(ref find_sum) = find_sum {
            if let Some(new_find_sum) = find_sum.improve(&diag1) {
                Some(new_find_sum)
            } else {
                return None;
            }
        } else {
            Some(diag1)
        };

        let mut sum = 0;
        let mut any_zero = false;
        for i in 0..n {
            let val = self.slots[i][n - 1 - i];
            if val == 0 {
                any_zero = true;
                sum += 1;
            } else {
                sum += val;
            }
        }
        let diag2 = PartialSum::new(sum, any_zero);
        find_sum = if let Some(ref find_sum) = find_sum {
            if let Some(new_find_sum) = find_sum.improve(&diag2) {
                Some(new_find_sum)
            } else {
                return None;
            }
        } else {
            Some(diag2)
        };

        find_sum
    }
}

#[derive(Clone)]
pub enum PartialSum {
    /// The sum is known exactly.
    Exact(u16),
    /// The sum must be at least something.
    AtLeast(u16),
}

impl PartialSum {
    pub fn new(sum: u16, any_zero: bool) -> PartialSum {
        if any_zero {
            PartialSum::AtLeast(sum)
        } else {
            PartialSum::Exact(sum)
        }
    }

    /// Returns `Some(true)` is equal, `Some(false)` is not equal,
    /// and `None` if it is unknown whether they are equal.
    pub fn equal(&self, other: &PartialSum) -> Option<bool> {
        use PartialSum::*;

        match (self, other) {
            (&Exact(a), &Exact(b)) => Some(a == b),
            (&AtLeast(a), &Exact(b)) => {
                if b > a {None}
                else {Some(false)}
            }
            (&Exact(a), &AtLeast(b)) => {
                if a > b {None}
                else {Some(false)}
            }
            (&AtLeast(_), &AtLeast(_)) => None,
        }
    }

    /// Improves knowledge using new evidence.
    /// Returns `None` if new evidence is conflicting with exiting one.
    pub fn improve(&self, new: &PartialSum) -> Option<PartialSum> {
        use PartialSum::*;

        match (self, new) {
            (&Exact(a), &Exact(b)) => {
                if a == b {Some(self.clone())}
                else {None}
            }
            (&AtLeast(a), &Exact(b)) => {
                if b >= a {Some(new.clone())}
                else {None}
            }
            (&Exact(a), &AtLeast(b)) => {
                if a >= b {Some(self.clone())}
                else {None}
            }
            (&AtLeast(a), &AtLeast(b)) => {Some(AtLeast(
                if a > b {a} else {b}
            ))}
        }
    }
}

fn main() {
    let x = MagicSquare::new(5);
    let settings = SolveSettings::new()
        .debug(false)
        .sleep_ms(1);
    let solver = BackTrackSolver::new(x, settings);
    let answer = solver.solve(|s| s.find_max_partial_sum_empty(), |s, p| s.possible(p));
    if let Some(answer) = answer {
        answer.puzzle.print();
        println!("Iterations: {}", answer.iterations);
    } else {
        println!("Found no solution");
    }
}
