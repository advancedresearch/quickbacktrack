//! Library for back tracking with customizable search for possible moves.
//!
//! [Back tracking](https://en.wikipedia.org/wiki/Backtracking) is a general algorithm to find
//! solution for constraint satisfaction problems.
//!
//! The performance of finding a solution can vary greatly with the algorithm used to look for
//! the best position to set a value next. For example, a sodoku puzzle with many missing numbers
//! can take 59 iterations when picking an empty slot with minimum number of options,
//! but it takes 295 992 iterations to solve when looking for the first empty slot.
//!
//! One can explain this difference in performance using probability theory.
//! If a slot can contain 2 correct out of N possible values, the odds are 2:N.
//! When this slot is tangled through constraints with another slot with odds 1:M,
//! the total odds become 2*1:N*M.
//! To maximize the chance of finding a correct solution, one must maximize the odds for the
//! remaining moves or fail to satisfy the constraints as early as possible.
//! Fewer choices reduces the chances of being wrong, increases the chance of failing constraints
//! early and therefore increases the chance of finding a solution more quickly.
//!
//! By making the search customizable, one can easier experiment with different algorithms
//! to pick the next best guess and see which has the best performance on a given problem.
//! This library is designed to assist with this kind of exploration.
//!
//! ### Solving simple moves
//!
//! In some constraint problems, there are lots of steps that are trivial once a choice is made.
//! For example, in Sudoku a lot of numbers can be filled in once a number is selected for a slot.
//!
//! By solving simple moves separately, one can improve performance and reduce the debugging
//! output significantly.
//!
//! ### Debugging
//!
//! The relationship between the structure of a puzzle and an efficient algorithm to pick the
//! next best guess can be non-trivial, so understanding what happens is essential for finding
//! an efficient algorithm.
//!
//! When the setting `SolveSettings::debug(true)` is enabled, the solver prints out the steps
//! to standard output while solving.
//!
//! The solver prints "Guess" when making a new move, and "Try" when changing an earlier move.
//! Number of iterations are printed at the end when the puzzle is solved.
//!
//! You can slow down the solving by setting `SolveSettings::sleep_ms(1000)`.
//! This makes the solver wait one second (1000 milliseconds) before continuing to the next step.

#![deny(missing_docs)]

use std::fmt::Debug;

/// Implemented by puzzles.
///
/// A puzzle stores the state of the problem, and can be modified by inserting a value at a
/// position within the puzzle. The solver does not understand the internal structure of the
/// puzzle, but is still able to find a solution (if any exists).
///
/// The initial state does not have to empty, and you can get the difference at the end
/// by setting `SolveSettings::difference(true)`.
pub trait Puzzle: Clone {
    /// The type of position.
    type Pos: Copy + Debug;
    /// The type of values stored in the puzzle.
    type Val: Copy + Debug + PartialEq;
    /// Solve simple stuff faster.
    /// This will reduce the number of steps in solution.
    /// If you do not know how to solve this, leave it empty.
    fn solve_simple(&mut self);
    /// Sets a value at position.
    fn set(&mut self, pos: Self::Pos, val: Self::Val);
    /// Print puzzle out to standard output.
    fn print(&self);
    /// Returns possible values at a given position.
    /// The last move in the list has highest priority, because the solver pops the values in turn.
    fn possible(&self, pos: Self::Pos) -> Vec<Self::Val>;
    /// Whether puzzle is solved.
    fn is_solved(&self) -> bool;
    /// Removes values from other puzzle to show changes.
    fn remove(&mut self, other: &Self);
}

/// Stores settings for solver.
///
/// Default settings:
///
/// - solve_simple: `true`
/// - debug: `false`
/// - difference: `false`
/// - sleep_ms: `None`
pub struct SolveSettings {
    solve_simple: bool,
    debug: bool,
    difference: bool,
    sleep_ms: Option<u64>,
}

impl SolveSettings {
    /// Creates new solve settings.
    pub fn new() -> SolveSettings {
        SolveSettings {
            solve_simple: true,
            debug: false,
            difference: false,
            sleep_ms: None,
        }
    }

    /// Sets wheter to solve simple moves between each step.
    pub fn set_solve_simple(&mut self, val: bool) {
        self.solve_simple = val;
    }

    /// Whether to solve simple moves between each step.
    pub fn solve_simple(mut self, val: bool) -> Self {
        self.set_solve_simple(val);
        self
    }

    /// Sets whether to debug by printing out to standard output.
    pub fn set_debug(&mut self, val: bool) {
        self.debug = val;
    }

    /// Whether to debug by printing out to standard output.
    pub fn debug(mut self, val: bool) -> Self {
        self.set_debug(val);
        self
    }

    /// Sets whether to return the difference from initial puzzle.
    pub fn set_difference(&mut self, val: bool) {
        self.difference = val;
    }

    /// Whether to return the difference from initial puzzle.
    pub fn difference(mut self, val: bool) -> Self {
        self.set_difference(val);
        self
    }

    /// Sets how many milliseconds to sleep between each step, if any.
    pub fn set_maybe_sleep_ms(&mut self, val: Option<u64>) {
        self.sleep_ms = val;
    }

    /// Sets how many milliseconds to sleep between each step, if any.
    pub fn maybe_sleep_ms(mut self, val: Option<u64>) -> Self {
        self.set_maybe_sleep_ms(val);
        self
    }

    /// Sets how many milliseconds to sleep between each step.
    pub fn set_sleep_ms(&mut self, val: u64) {
        self.sleep_ms = Some(val);
    }

    /// How many milliseconds to sleep between each step.
    pub fn sleep_ms(mut self, val: u64) -> Self {
        self.set_sleep_ms(val);
        self
    }
}

/// Solvees puzzles using back tracking.
pub struct BackTrackSolver<T>
    where T: Puzzle
{
	/// Stores the states.
	pub states: Vec<T>,
	/// Stores the choices for the states.
	pub choice: Vec<(T::Pos, Vec<T::Val>)>,
	/// Search for simple solutions.
	pub settings: SolveSettings,
}

impl<T> BackTrackSolver<T>
    where T: Puzzle
{
    /// Creates a new solver.
	pub fn new(puzzle: T, settings: SolveSettings) -> BackTrackSolver<T> {
		BackTrackSolver {
			states: vec![puzzle],
			choice: vec![],
			settings: settings,
		}
	}

    /// Solves puzzle, using a closure to look for best position to set a value next.
	pub fn solve<F>(mut self, mut f: F) -> Option<T>
		where F: FnMut(&T) -> Option<T::Pos>
	{
		use std::thread::sleep;
		use std::time::Duration;

		let mut iterations: usize = 0;
		loop {
            if self.settings.debug {
                if let Some(ms) = self.settings.sleep_ms {
        			sleep(Duration::from_millis(ms));
                }
            }
			let n = self.states.len() - 1;
			let mut new = self.states[n].clone();
			if self.settings.solve_simple {
				new.solve_simple();
			}
            if self.settings.debug {
    			new.print();
            }
			iterations += 1;
			if new.is_solved() {
                if self.settings.debug {
				    println!("Solved! Iterations: {}", iterations);
                }
                if self.settings.difference {
				    new.remove(&self.states[0]);
                }
				return Some(new);
			}

			let empty = match f(&new) {
                None => {
                    if self.settings.debug {
                        println!("No more possible choices");
                    }
                    return None;
                }
                Some(x) => x
            };
			let mut possible = new.possible(empty);
			if possible.len() == 0 {
				// println!("No possible at {:?}", empty);
				loop {
					if self.choice.len() == 0 {
                        if self.settings.debug {
    						// No more possible choices.
    						println!("No more possible choices");
                        }
						return None;
					}
					let (pos, mut possible) = self.choice.pop().unwrap();
					if let Some(new_val) = possible.pop() {
						// Try next choice.
						let n = self.states.len() - 1;
						self.states[n].set(pos, new_val);
						self.choice.push((pos, possible));
                        if self.settings.debug {
    						println!("Try   {:?}, {:?} depth {} {} (failed at {:?})",
    							pos, new_val, self.choice.len(), self.states.len(), empty);
                        }
						break;
					} else {
						if self.states.pop().is_none() {
							// No more possible choices.
							return None;
						}
					}
				}
			} else {
				// Put in the first guess.
				let v = possible.pop().unwrap();
				new.set(empty, v);
				self.choice.push((empty, possible));
				self.states.push(new);
                if self.settings.debug {
				    println!("Guess {:?}, {:?} depth {} {}",
                        empty, v, self.choice.len(), self.states.len());
                }
			}
		}
	}
}
