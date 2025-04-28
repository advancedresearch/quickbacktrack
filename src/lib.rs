#![doc = include_str!("../README.md")]

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
    ///
    /// Call the closure when making a simple choice.
    fn solve_simple<F: FnMut(&mut Self, Self::Pos, Self::Val)>(&mut self, _f: F) {}
    /// Sets a value at position.
    fn set(&mut self, pos: Self::Pos, val: Self::Val);
    /// Gets value at position.
    fn get(&self, pos: Self::Pos) -> Self::Val;
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
    /// Whether to solve simple steps.
    pub solve_simple: bool,
    /// Whether to output debug prints.
    pub debug: bool,
    /// Show difference from original puzzle.
    pub difference: bool,
    /// The number of milliseconds to sleep before each debug print.
    pub sleep_ms: Option<u64>,
    /// The number of maximum iterations.
    pub max_iterations: Option<u64>,
    /// Whether to print every million iteration.
    pub print_millions: bool,
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
            print_millions: false,
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

    /// Sets printing of every million iteration to standard error output.
    pub fn set_print_millions(&mut self, val: bool) {
        self.print_millions = val;
    }

    /// Prints every million iteration to standard error output.
    pub fn print_millions(mut self, val: bool) -> Self {
        self.set_print_millions(val);
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

/// Solves puzzles using back tracking.
pub struct BackTrackSolver<T>
    where T: Puzzle
{
    /// Stores the original state.
    pub original: T,
    /// Stores the state.
    pub state: T,
    /// Stores the previous values of a position before making a choice.
    /// If the flag is true, the value was inserted due to a simple choice.
    pub prevs: Vec<(T::Pos, T::Val, bool)>,
    /// Stores the choices for the states.
    pub choice: Vec<(T::Pos, Vec<T::Val>)>,
    /// Stores solve settings.
    pub settings: SolveSettings,
}

impl<T> BackTrackSolver<T>
    where T: Puzzle
{
    /// Creates a new solver.
    pub fn new(puzzle: T, settings: SolveSettings) -> BackTrackSolver<T> {
        BackTrackSolver {
            original: puzzle.clone(),
            state: puzzle,
            prevs: vec![],
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
            if self.settings.solve_simple {
                let ref mut prevs = self.prevs;
                self.state.solve_simple(|state, pos, val| {
                    prevs.push((pos, state.get(pos), true));
                    state.set(pos, val);
                });
            }
            if self.settings.debug {
                self.state.print();
            }
            iterations += 1;
            if let Some(max_iterations) = self.settings.max_iterations {
                if iterations > max_iterations {
                    return None;
                }
            }
            if self.state.is_solved() {
                if self.settings.debug {
                    eprintln!("Solved! Iterations: {}", iterations);
                }
                if self.settings.difference {
                    self.state.remove(&self.original);
                }
                return Some(Solution { puzzle: self.state, iterations: iterations, strategy: None });
            }

            let empty = f(&self.state);
            let mut possible = match empty {
                None => vec![],
                Some(x) => g(&self.state, x)
            };
            if possible.len() == 0 {
                loop {
                    if self.choice.len() == 0 {
                        if self.settings.debug {
                            // No more possible choices.
                            eprintln!("No more possible choices");
                        }
                        return None;
                    }
                    let (pos, mut possible) = self.choice.pop().unwrap();
                    if let Some(new_val) = possible.pop() {
                        // Try next choice.
                        while let Some((old_pos, old_val, simple)) = self.prevs.pop() {
                            self.state.set(old_pos, old_val);
                            if !simple {break}
                        }
                        self.prevs.push((pos, self.state.get(pos), false));
                        self.state.set(pos, new_val);
                        self.choice.push((pos, possible));
                        if self.settings.debug {
                            eprintln!("Try   {:?}, {:?} depth ch: {} prev: {} (failed at {:?}) it: {}",
                                pos, new_val, self.choice.len(), self.prevs.len(), empty, iterations);
                        } else if self.settings.print_millions && (iterations % 1_000_000 == 0) {
                            eprintln!("Iteration: {}mill", iterations / 1_000_000);
                        }
                        break;
                    } else {
                        let mut undo = false;
                        while let Some((old_pos, old_val, simple)) = self.prevs.pop() {
                            self.state.set(old_pos, old_val);
                            undo = true;
                            if !simple {break}
                        }
                        if !undo {
                            // No more possible choices.
                            return None;
                        }
                    }
                }
            } else {
                let empty = empty.unwrap();
                // Put in the first guess.
                let v = possible.pop().unwrap();
                self.prevs.push((empty, self.state.get(empty), false));
                self.state.set(empty, v);
                self.choice.push((empty, possible));
                if self.settings.debug {
                    eprintln!("Guess {:?}, {:?} depth ch: {} prev: {} it: {}",
                        empty, v, self.choice.len(), self.prevs.len(), iterations);
                } else if self.settings.print_millions && (iterations % 1_000_000 == 0) {
                    eprintln!("Iteration: {}mill", iterations / 1_000_000);
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
    pub states: Vec<T>,
    /// Stores previous values before making choices.
    /// The flags is true when made a simple choice.
    pub prevs: Vec<Vec<(T::Pos, T::Val, bool)>>,
    /// Stores the choices for the states.
    pub choice: Vec<Vec<(T::Pos, Vec<T::Val>)>>,
    /// Stores solve settings.
    pub settings: SolveSettings,
}

impl<T> MultiBackTrackSolver<T>
    where T: Puzzle
{
    /// Creates a new solver.
    pub fn new(settings: SolveSettings) -> MultiBackTrackSolver<T> {
        MultiBackTrackSolver {
            states: vec![],
            prevs: vec![],
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

        let origin = puzzle.clone();
        self.states = vec![puzzle; strategies.len()];
        self.prevs = vec![vec![]; strategies.len()];
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
                let ref mut state = self.states[i];
                let ref mut prevs = self.prevs[i];
                let ref mut choice = self.choice[i];
                let (f, g) = strategies[i];

                if self.settings.solve_simple {
                    state.solve_simple(|state, pos, val| {
                        prevs.push((pos, state.get(pos), true));
                        state.set(pos, val)
                    });
                }
                if self.settings.debug {
                    println!("Strategy {}", i);
                    state.print();
                }
                if state.is_solved() {
                    if self.settings.debug {
                        println!("Solved! Iterations: {}", iterations);
                    }
                    if self.settings.difference {
                        state.remove(&origin);
                    }
                    return Some(Solution { puzzle: state.clone(), iterations: iterations, strategy: Some(i) });
                }

                let empty = f(&state);
                let mut possible = match empty {
                    None => vec![],
                    Some(x) => g(&state, x)
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
                            while let Some((old_pos, old_val, simple)) = prevs.pop() {
                                state.set(old_pos, old_val);
                                if !simple {break}
                            }
                            prevs.push((pos, state.get(pos), false));
                            state.set(pos, new_val);
                            choice.push((pos, possible));
                            if self.settings.debug {
                                eprintln!("Try   {:?}, {:?} depth ch: {} prev: {} (failed at {:?}) it: {}",
                                    pos, new_val, self.choice.len(), self.prevs.len(), empty, iterations);
                            } else if self.settings.print_millions && (iterations % 1_000_000 == 0) {
                                eprintln!("Iteration: {}mill", iterations / 1_000_000);
                            }
                            break;
                        } else {
                            let mut undo = false;
                            while let Some((old_pos, old_val, simple)) = prevs.pop() {
                                state.set(old_pos, old_val);
                                undo = true;
                                if !simple {break}
                            }
                            if !undo {
                                // No more possible choices.
                                return None;
                            }
                        }
                    }
                } else {
                    let empty = empty.unwrap();
                    // Put in the first guess.
                    let v = possible.pop().unwrap();
                    prevs.push((empty, state.get(empty), false));
                    state.set(empty, v);
                    choice.push((empty, possible));
                    if self.settings.debug {
                        eprintln!("Guess {:?}, {:?} depth ch: {} prev: {} it: {}",
                            empty, v, self.choice.len(), self.prevs.len(), iterations);
                    } else if self.settings.print_millions && (iterations % 1_000_000 == 0) {
                        eprintln!("Iteration: {}mill", iterations / 1_000_000);
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

/// Stores settings for entropy solver.
pub struct EntropySolveSettings {
    /// The number of solve attempts.
    pub attempts: u64,
    /// Whether to sample randomly (1) or converge (0).
    pub noise: f64,
    /// Make one final attempt with maximum iterations setting.
    pub final_attempt: Option<Option<u64>>,
}

impl EntropySolveSettings {
    /// Creates new entropy settings.
    pub fn new() -> EntropySolveSettings {
        EntropySolveSettings {
            attempts: 1,
            noise: 0.0,
            final_attempt: None,
        }
    }

    /// Sets number of attempts.
    pub fn set_attempts(&mut self, val: u64) {
        self.attempts = val;
    }

    /// The number of attempts.
    pub fn attempts(mut self, val: u64) -> Self {
        self.set_attempts(val);
        self
    }

    /// Sets the noise (0 = converge, 1 = random sampling).
    pub fn set_noise(&mut self, val: f64) {
        self.noise = val;
    }

    /// The noise (0 = converge, 1 = random sampling).
    pub fn noise(mut self, val: f64) -> Self {
        self.set_noise(val);
        self
    }

    /// Sets one final attempt with maximum iterations setting.
    pub fn set_final_attempt(&mut self, val: Option<Option<u64>>) {
        self.final_attempt = val;
    }

    /// The final attempt with maximum iterations setting.
    pub fn final_attempt(mut self, val: Option<Option<u64>>) -> Self {
        self.set_final_attempt(val);
        self
    }
}

/// Solves puzzles using minimum entropy search.
///
/// This solver learns from repeatedly attempting to solve the puzzle.
/// The algorithm is inspired by [WaveFunctionCollapse](https://github.com/mxgmn/WaveFunctionCollapse).
///
/// This solver is general and guaranteed to find a solution, if any.
/// It also uses custom priority of choices in the initial attempts.
///
/// The search works by attempting normal backtrack solving,
/// but increasing weights to choices each time they are made.
/// When the algorithm is stuck, it minimizes entropy of common choices.
/// At later attempts, the algorithm will try these common choices first.
///
/// When `EntropySettings::noise` is non-zero, the choices will occationally be shuffled.
/// For more information, see `EntropySolveSettings`.
pub struct EntropyBackTrackSolver<T> where T: Puzzle {
    /// Stores the original state.
    pub original: T,
    /// Stores the state.
    pub state: T,
    /// Stores the previous values of a position before making a choice.
    /// If the flag is true, the value was inserted due to a simple choice.
    pub prevs: Vec<(T::Pos, T::Val, bool)>,
    /// Stores the choices for the states.
    pub choice: Vec<(T::Pos, Vec<T::Val>)>,
    /// The initial choices.
    pub start_choice: Vec<(T::Pos, Vec<T::Val>)>,
    /// Stores weights of choices.
    pub weights: Vec<Vec<f64>>,
    /// Stores solve settings.
    pub settings: SolveSettings,
    /// Stores entropy solve settings.
    pub entropy_settings: EntropySolveSettings,
}

impl<T> EntropyBackTrackSolver<T> where T: Puzzle {
    /// Creates a new collapse solver.
    pub fn new(
        puzzle: T,
        start_choice: Vec<(T::Pos, Vec<T::Val>)>,
        entropy_settings: EntropySolveSettings,
        settings: SolveSettings
    ) -> Self {
        let weights = start_choice.iter().map(|n| vec![1.0; n.1.len()]).collect();
        EntropyBackTrackSolver {
            original: puzzle.clone(),
            prevs: vec![],
            state: puzzle,
            choice: vec![],
            start_choice,
            weights,
            entropy_settings,
            settings,
        }
    }

    /// Calculates the entropy of a choice.
    pub fn entropy(&self, i: usize) -> f64 {
        let sum: f64 = self.weights[i].iter().sum();
        self.weights[i].iter().map(|&w| {
                let p: f64 = w / sum;
                -(p * p.ln())
            }).sum()
    }

    /// Finds the position with least entropy.
    pub fn min_entropy<G>(&self, g: &mut G) -> Option<(usize, T::Pos)>
        where G: FnMut(&T, T::Pos) -> Vec<T::Val>
    {
        let mut min: Option<(usize, f64)> = None;
        for i in 0..self.weights.len() {
            if self.weights.len() == 0 {continue};
            if g(&self.state, self.start_choice[i].0).len() == 0 {continue};
            let e = self.entropy(i);
            if min.is_none() || min.unwrap().1 > e {
                min = Some((i, e));
            }
        }
        min.map(|(i, _)| (i, self.start_choice[i].0))
    }

    /// Increase weight of observed state.
    pub fn observe(&mut self, pos: T::Pos, new_val: T::Val)
        where T::Pos: PartialEq,
    {
        for (i, ch) in self.start_choice.iter().enumerate() {
            if ch.0 == pos {
                for (j, val) in self.start_choice[i].1.iter().enumerate() {
                    if *val == new_val {
                        self.weights[i][j] += 1.0;
                        return;
                    }
                }
            }
        }
    }

    /// Attempts to solve puzzle repeatedly, using `SolveSettings::max_iterations`.
    ///
    /// The solver learns by reusing weights from previous attempts.
    pub fn solve<G>(&mut self, g: G) -> (u64, Option<Solution<T>>)
        where G: Copy + FnMut(&T, T::Pos) -> Vec<T::Val>,
              T::Pos: PartialEq
    {
        let mut solution = None;
        let mut i = 0;
        if self.settings.max_iterations.is_some() {
            loop {
                if i >= self.entropy_settings.attempts {break};

                if solution.is_none() {
                    solution = self.solve_single_attempt(g);
                } else {
                    break;
                }

                i += 1;
            }
        }
        if solution.is_none() {
            if let Some(new_max_iter) = self.entropy_settings.final_attempt {
                let max_iter = self.settings.max_iterations;
                let noise = self.entropy_settings.noise;
                self.entropy_settings.noise = 0.0;
                self.settings.max_iterations = new_max_iter;
                solution = self.solve_single_attempt(g);
                // Reset old settings.
                self.settings.max_iterations = max_iter;
                self.entropy_settings.noise = noise;
            }
        }
        (i, solution)
    }

    /// Solves puzzle, using a closure for picking options in preferred order.
    ///
    /// This can be called repeated times, limited by `SolveSettings::max_iterations`
    /// to reuse weights from previous attempts.
    pub fn solve_single_attempt<G>(&mut self, mut g: G) -> Option<Solution<T>>
        where G: FnMut(&T, T::Pos) -> Vec<T::Val>,
              T::Pos: PartialEq
    {
        use std::thread::sleep;
        use std::time::Duration;

        let mut rng = rand::thread_rng();
        let mut iterations: u64 = 0;
        loop {
            if self.settings.debug {
                if let Some(ms) = self.settings.sleep_ms {
                    sleep(Duration::from_millis(ms));
                }
            }
            if self.settings.solve_simple {
                let ref mut prevs = self.prevs;
                self.state.solve_simple(|state, pos, val| {
                    prevs.push((pos, state.get(pos), true));
                    state.set(pos, val);
                });
            }
            if self.settings.debug {
                self.state.print();
            }
            iterations += 1;
            if let Some(max_iterations) = self.settings.max_iterations {
                if iterations > max_iterations {
                    return None;
                }
            }
            if self.state.is_solved() {
                if self.settings.debug {
                    eprintln!("Solved! Iterations: {}", iterations);
                }
                if self.settings.difference {
                    self.state.remove(&self.original);
                }
                return Some(Solution { puzzle: self.state.clone(), iterations: iterations, strategy: None });
            }

            let empty = self.min_entropy(&mut g);
            let mut possible = match empty {
                None => vec![],
                Some((ind, x)) => {
                    use rand::Rng;

                    let mut possible = g(&self.state, x);
                    if rng.gen::<f64>() < self.entropy_settings.noise {
                        use rand::seq::SliceRandom;
                        possible.shuffle(&mut rng);
                        possible
                    } else {
                        let mut keys = vec![];
                        for (j, p) in possible.iter().enumerate() {
                            for i in 0..self.start_choice[ind].1.len() {
                                if self.start_choice[ind].1[i] == *p {
                                    keys.push((j, self.weights[ind][i]));
                                    break;
                                }
                            }
                        }
                        keys.sort_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap());
                        let new_possible = keys.iter().map(|&(i, _)| possible[i]).collect::<Vec<T::Val>>();
                        new_possible
                    }
                }
            };
            if possible.len() == 0 {
                loop {
                    if self.choice.len() == 0 {
                        if self.settings.debug {
                            // No more possible choices.
                            eprintln!("No more possible choices");
                        }
                        return None;
                    }
                    let (pos, mut possible) = self.choice.pop().unwrap();
                    if let Some(new_val) = possible.pop() {
                        // Try next choice.
                        while let Some((old_pos, old_val, simple)) = self.prevs.pop() {
                            self.state.set(old_pos, old_val);
                            if !simple {break}
                        }
                        self.prevs.push((pos, self.state.get(pos), false));
                        self.state.set(pos, new_val);
                        self.observe(pos, new_val);
                        self.choice.push((pos, possible));
                        if self.settings.debug {
                            eprintln!("Try   {:?}, {:?} depth ch: {} prev: {} (failed at {:?}) it: {}",
                                pos, new_val, self.choice.len(), self.prevs.len(), empty, iterations);
                        } else if self.settings.print_millions && (iterations % 1_000_000 == 0) {
                            eprintln!("Iteration: {}mill", iterations / 1_000_000);
                        }
                        break;
                    } else {
                        let mut undo = false;
                        while let Some((old_pos, old_val, simple)) = self.prevs.pop() {
                            self.state.set(old_pos, old_val);
                            undo = true;
                            if !simple {break}
                        }
                        if !undo {
                            // No more possible choices.
                            return None;
                        }
                    }
                }
            } else {
                let empty = empty.unwrap().1;
                // Put in the first guess.
                let v = possible.pop().unwrap();
                self.prevs.push((empty, self.state.get(empty), false));
                self.state.set(empty, v);
                self.observe(empty, v);
                self.choice.push((empty, possible));
                if self.settings.debug {
                    eprintln!("Guess {:?}, {:?} depth ch: {} prev: {} it: {}",
                        empty, v, self.choice.len(), self.prevs.len(), iterations);
                } else if self.settings.print_millions && (iterations % 1_000_000 == 0) {
                    eprintln!("Iteration: {}mill", iterations / 1_000_000);
                }
            }
        }
    }
}
