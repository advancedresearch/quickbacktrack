/*

Place 8 queens on a chess board such that they can not attack each other.

*/

extern crate quickbacktrack;

use quickbacktrack::{BackTrackSolver, Puzzle, SolveSettings};

#[derive(Clone)]
pub struct EightQueens {
    pub queens: Vec<u8>,
}

impl Puzzle for EightQueens {
    type Pos = usize;
    type Val = u8;

    fn solve_simple(&mut self) {}

    fn set(&mut self, pos: usize, val: u8) {
        self.queens[pos] = val;
    }

    fn print(&self) {
        for _ in 0..self.queens.len() {
            print!(" _");
        }
        println!("");
        for y in 0..self.queens.len() {
            print!("|");
            for x in 0..self.queens.len() as u8 {
                let q = self.queens[y];
                if q > 0 && q - 1 == x {
                    print!("x|");
                } else {
                    print!("_|");
                }
            }
            println!("")
        }
        println!("")
    }

    fn is_solved(&self) -> bool {
        for q in &self.queens {
            if *q == 0 { return false; }
        }
        return true;
    }

    fn remove(&mut self, other: &EightQueens) {
        for i in 0..self.queens.len() {
            if other.queens[i] > 0 {
                self.queens[i] = 0;
            }
        }
    }
}

impl EightQueens {
    pub fn new(size: usize) -> EightQueens {
        EightQueens {
            queens: vec![0; size]
        }
    }

    pub fn is_valid(&self, pos: usize, val: u8) -> bool {
        for q in &self.queens {
            if *q == val { return false; }
        }
        let this_pos = [val as i8 - 1, pos as i8];
        for (i, q) in self.queens.iter().enumerate() {
            if *q == 0 { continue; }
            let that_pos = [*q as i8 - 1, i as i8];
            if can_take(that_pos, this_pos) {
                return false;
            }
        }
        return true
    }

    pub fn next_pos(&self) -> Option<usize> {
        for i in 0..self.queens.len() {
            if self.queens[i] == 0 { return Some(i); }
        }
        return None;
    }

    pub fn next_imp_pos(&self) -> Option<usize> {
        // If it is impossible to place a queen in any row,
        // then the puzzle is impossible to solve.
        for i in 0..self.queens.len() {
            if self.queens[i] > 0 { continue; }
            if self.possible(i).len() == 0 { return None; };
        }
        return self.next_pos();
    }

    pub fn find_min_pos(&self) -> Option<usize> {
        let mut min: Option<usize> = None;
        let mut min_possible: Option<usize> = None;
        for i in 0..self.queens.len() {
            if self.queens[i] > 0 { continue; }
            let possible = self.possible(i);
            if min.is_none() || possible.len() < min_possible.unwrap() {
                min_possible = Some(possible.len());
                min = Some(i);
            }
        }
        return min;
    }

    pub fn find_imp_pos(&self) -> Option<usize> {
        // If it is impossible to place a queen in any row,
        // then the puzzle is impossible to solve.
        for i in 0..self.queens.len() {
            if self.queens[i] > 0 { continue; }
            if self.possible(i).len() == 0 { return None; };
        }
        return self.find_min_pos();
    }

    pub fn possible(&self, pos: usize) -> Vec<u8> {
        let mut res = vec![];
        if self.queens[pos] > 0 {
            res.push(self.queens[pos]);
        } else {
            for v in 1..(self.queens.len() + 1) as u8 {
                if self.is_valid(pos, v) {
                    res.push(v);
                }
            }
        }
        return res;
    }
}

fn can_take(a: [i8; 2], b: [i8; 2]) -> bool {
    let diff = [a[0] - b[0], a[1] - b[1]];
    return diff[0].abs() == diff[1].abs();
}

fn main() {
    for i in 8..9 {
        let max_iterations = 300_000;
        let board = EightQueens::new(i);
        let settings = SolveSettings::new()
            .debug(true)
            .sleep_ms(100)
            .max_iterations(max_iterations)
        ;
        let solver = BackTrackSolver::new(board, settings);
        match solver.solve(|board| board.find_min_pos(),
                           |board, p| board.possible(p)) {
            None => {
                println!("{} >{}", i, max_iterations);
            }
            Some(x) => {
                // answer.puzzle.print();
                println!("{} {}", i, x.iterations);
            }
        }
    }
}
