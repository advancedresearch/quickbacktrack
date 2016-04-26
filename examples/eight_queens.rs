/*

Place 8 queens on a chess board such that they can not attack each other.

*/

extern crate quickbacktrack;

use quickbacktrack::{BackTrackSolver, Puzzle, SolveSettings};

#[derive(Clone)]
pub struct EightQueens {
    pub queens: [u8; 8],
}

impl Puzzle for EightQueens {
    type Pos = usize;
    type Val = u8;

    fn solve_simple(&mut self) {}

    fn set(&mut self, pos: usize, val: u8) {
        self.queens[pos] = val;
    }

    fn print(&self) {
        println!(" _ _ _ _ _ _ _ _ ");
        for y in 0..8 {
            print!("|");
            for x in 0..8 {
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

    fn possible(&self, pos: usize) -> Vec<u8> {
        let mut res = vec![];
        if self.queens[pos] > 0 {
            res.push(self.queens[pos]);
        } else {
            for v in 1..9 {
                if self.is_valid(pos, v) {
                    res.push(v);
                }
            }
        }
        return res;
    }

    fn is_solved(&self) -> bool {
        for q in &self.queens {
            if *q == 0 { return false; }
        }
        return true;
    }

    fn remove(&mut self, other: &EightQueens) {
        for i in 0..8 {
            if other.queens[i] > 0 {
                self.queens[i] = 0;
            }
        }
    }
}

impl EightQueens {
    pub fn new() -> EightQueens {
        EightQueens {
            queens: [0; 8]
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
        for i in 0..8 {
            if self.queens[i] == 0 { return Some(i); }
        }
        return None;
    }
}

fn can_take(a: [i8; 2], b: [i8; 2]) -> bool {
    let diff = [a[0] - b[0], a[1] - b[1]];
    return diff[0].abs() == diff[1].abs();
}

fn main() {
    let board = EightQueens::new();
    let settings = SolveSettings::new()
        .debug(true)
        .sleep_ms(100)
    ;
    let solver = BackTrackSolver::new(board, settings);
    let answer = solver.solve(|board| board.next_pos()).expect("Expected solution");
    answer.print();
}
