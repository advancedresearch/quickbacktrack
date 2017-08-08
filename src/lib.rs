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

extern crate fnv;

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
    max_iterations: Option<u64>,
}

impl SolveSettings {
    /// Creates new solve settings.
    pub fn new() -> SolveSettings {
        SolveSettings {
            solve_simple: true,
            debug: false,
            difference: false,
            sleep_ms: None,
            max_iterations: None,
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

    /// Sets the maximum number of iterations before giving up.
    pub fn set_maybe_max_iterations(&mut self, val: Option<u64>) {
        self.max_iterations = val;
    }

    /// The maximum number of iterations before giving up.
    pub fn maybe_max_iterations(mut self, val: Option<u64>) -> Self {
        self.set_maybe_max_iterations(val);
        self
    }

    /// Sets the maximum number of iterations before giving up.
    pub fn set_max_iterations(&mut self, val: u64) {
        self.max_iterations = Some(val);
    }

    /// The maximum number of iterations before giving up.
    pub fn max_iterations(mut self, val: u64) -> Self {
        self.set_max_iterations(val);
        self
    }
}

/// Contains solution.
pub struct Solution<T> {
    /// The solved puzzle.
    pub puzzle: T,
    /// The number of iterations used to solve the puzzle.
    pub iterations: u64,
    /// The strategy that found the solution.
    pub strategy: Option<usize>,
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

    /// Solves puzzle, using a closure to look for best position to set a value next,
    /// and a closure for picking options in preferred order.
    ///
    /// The second closure returns possible values at a given position.
    /// The last move in the list has highest priority, because the solver pops the values in turn.
    pub fn solve<F, G>(mut self, mut f: F, mut g: G) -> Option<Solution<T>>
        where F: FnMut(&T) -> Option<T::Pos>,
              G: FnMut(&T, T::Pos) -> Vec<T::Val>
    {
        use std::thread::sleep;
        use std::time::Duration;

        let mut iterations: u64 = 0;
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
            if let Some(max_iterations) = self.settings.max_iterations {
                if iterations > max_iterations {
                    return None;
                }
            }
            if new.is_solved() {
                if self.settings.debug {
                    println!("Solved! Iterations: {}", iterations);
                }
                if self.settings.difference {
                    new.remove(&self.states[0]);
                }
                return Some(Solution { puzzle: new, iterations: iterations, strategy: None });
            }

            let empty = f(&new);
            let mut possible = match empty {
                None => vec![],
                Some(x) => g(&new, x)
            };
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
                let empty = empty.unwrap();
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

/// Solves puzzle using multiple strategies at the same time.
/// Each strategy is evaluated one step by turn until a solution is found.
pub struct MultiBackTrackSolver<T>
    where T: Puzzle
{
    /// Stores the states.
    pub states: Vec<Vec<T>>,
    /// Stores the choices for the states.
    pub choice: Vec<Vec<(T::Pos, Vec<T::Val>)>>,
    /// Search for simple solutions.
    pub settings: SolveSettings,
}

impl<T> MultiBackTrackSolver<T>
    where T: Puzzle
{
    /// Creates a new solver.
    pub fn new(settings: SolveSettings) -> MultiBackTrackSolver<T> {
        MultiBackTrackSolver {
            states: vec![],
            choice: vec![],
            settings: settings,
        }
    }

    /// Solves puzzle, using a closure to look for best position to set a value next,
    /// and a closure for picking options in preferred order.
    ///
    /// The second closure returns possible values at a given position.
    /// The last move in the list has highest priority, because the solver pops the values in turn.
    ///
    /// If you have problems compiling, annotate type `(fn(&_) -> _, fn(&_, _) -> _)` to
    /// the list of strategies, e.g. `Vec<(fn(&_) -> _, fn(&_, _) -> _)>` or
    /// `&[(fn(&_) -> _, fn(&_, _) -> _)]`.
    pub fn solve(
        mut self,
        puzzle: T,
        strategies: &[(fn(&T) -> Option<T::Pos>, fn(&T, T::Pos) -> Vec<T::Val>)]
    ) -> Option<Solution<T>> {
        use std::thread::sleep;
        use std::time::Duration;

        self.states = vec![vec![puzzle]; strategies.len()];
        self.choice = vec![vec![]; strategies.len()];
        let mut iterations: u64 = 0;
        loop {
            if self.settings.debug {
                if let Some(ms) = self.settings.sleep_ms {
                    sleep(Duration::from_millis(ms));
                }
            }

            iterations += 1;
            if let Some(max_iterations) = self.settings.max_iterations {
                if iterations > max_iterations {
                    return None;
                }
            }

            for i in 0..strategies.len() {
                let ref mut states = self.states[i];
                let ref mut choice = self.choice[i];
                let (f, g) = strategies[i];

                let n = states.len() - 1;
                let mut new = states[n].clone();
                if self.settings.solve_simple {
                    new.solve_simple();
                }
                if self.settings.debug {
                    println!("Strategy {}", i);
                    new.print();
                }
                if new.is_solved() {
                    if self.settings.debug {
                        println!("Solved! Iterations: {}", iterations);
                    }
                    if self.settings.difference {
                        new.remove(&states[0]);
                    }
                    return Some(Solution { puzzle: new, iterations: iterations, strategy: Some(i) });
                }

                let empty = f(&new);
                let mut possible = match empty {
                    None => vec![],
                    Some(x) => g(&new, x)
                };
                if possible.len() == 0 {
                    // println!("No possible at {:?}", empty);
                    loop {
                        if choice.len() == 0 {
                            if self.settings.debug {
                                // No more possible choices.
                                println!("No more possible choices");
                            }
                            return None;
                        }
                        let (pos, mut possible) = choice.pop().unwrap();
                        if let Some(new_val) = possible.pop() {
                            // Try next choice.
                            let n = states.len() - 1;
                            states[n].set(pos, new_val);
                            choice.push((pos, possible));
                            if self.settings.debug {
                                println!("Try   {:?}, {:?} depth {} {} (failed at {:?})",
                                    pos, new_val, choice.len(), states.len(), empty);
                            }
                            break;
                        } else {
                            if states.pop().is_none() {
                                // No more possible choices.
                                return None;
                            }
                        }
                    }
                } else {
                    let empty = empty.unwrap();
                    // Put in the first guess.
                    let v = possible.pop().unwrap();
                    new.set(empty, v);
                    choice.push((empty, possible));
                    states.push(new);
                    if self.settings.debug {
                        println!("Guess {:?}, {:?} depth {} {}",
                            empty, v, choice.len(), states.len());
                    }
                }
            }
        }
    }
}

/// Combines multiple priority lists together.
///
/// This is used to combine strategies into a new one.
/// Sometimes this is better than using either strategy.
pub fn combine<T>(lists: Vec<Vec<T>>) -> Vec<T>
	where T: Clone + ::std::hash::Hash + Eq
{
	let mut priority: fnv::FnvHashMap<T, usize> = fnv::FnvHashMap::default();
	for list in &lists {
		for (i, ch) in list.iter().enumerate() {
			if priority.contains_key(ch) {
				let old = priority[ch];
				priority.insert(ch.clone(), old + i);
			} else {
				priority.insert(ch.clone(), i);
			}
		}
	}

	let keys: Vec<&T> = priority.keys().collect();
	let mut inds: Vec<usize> = (0..keys.len()).collect();
	inds.sort_by_key(|&ind| priority[keys[ind]]);
	let mut res = Vec::with_capacity(keys.len());
	for &ind in &inds {
		res.push(keys[ind].clone());
	}
	res
}
