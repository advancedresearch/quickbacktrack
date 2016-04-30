/*

We have a bag with maximum capacity and some valuable items we want to pack into it.
Find the items required to reach maximum value without exceeding the capacity.

For more information about the Knapsack problem, see https://en.wikipedia.org/wiki/Knapsack_problem

*/

extern crate quickbacktrack;

use quickbacktrack::{BackTrackSolver, Puzzle, SolveSettings};

#[derive(Debug)]
pub struct Item {
    pub desc: &'static str,
    pub weight: f64,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct Bag {
    pub items: u32,
    pub max_weight: f64,
    pub target_value: f64,
}

impl Puzzle for Bag {
    type Pos = usize;
    type Val = bool;

    fn solve_simple(&mut self) {}

    fn set(&mut self, ind: usize, val: bool) {
        if val {
            self.items |= 1 << ind;
        } else {
            self.items &= !(1 << ind);
        }
    }

    fn print(&self) {
        for i in 0..self.item_count() {
            if self.get(i) {
                let info = self.item_info(i);
                println!("{:?}", info);
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.total_value() > self.target_value
    }

    fn remove(&mut self, other: &Bag) {
        for i in 0..self.item_count() {
            if other.get(i) {
                self.set(i, false);
            }
        }
    }

    fn possible(&self, ind: usize) -> Vec<bool> {
        let mut res = vec![];
        if self.get(ind) {
            res.push(true);
        } else {
            let item = self.item_info(ind);
            if self.total_weight() + item.weight
                <= self.max_weight {
                res.push(true);
            }
        }
        return res;
    }
}

impl Bag {
    pub fn new(max_weight: f64, target_value: f64) -> Bag {
        Bag {
            items: 0,
            max_weight: max_weight,
            target_value: target_value,
        }
    }

    pub fn item_count(&self) -> usize { 6 }

    pub fn item_info(&self, ind: usize) -> Item {
        match ind {
            0 => Item { desc: "chocolate", weight: 0.2, value: 40.0 },
            1 => Item { desc: "book", weight: 0.5, value: 300.0 },
            2 => Item { desc: "hat", weight: 0.1, value: 1000.0 },
            3 => Item { desc: "sleeping bag", weight: 0.8, value: 1400.0 },
            4 => Item { desc: "socks", weight: 0.1, value: 200.0 },
            5 => Item { desc: "banana", weight: 0.2, value: 50.0 },
            _ => panic!("Not an item {}", ind),
        }
    }

    pub fn get(&self, ind: usize) -> bool {
        self.items & (1 << ind) == (1 << ind)
    }

    pub fn total_weight(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.item_count() {
            if self.get(i) {
                let info = self.item_info(i);
                sum += info.weight;
            }
        }
        return sum;
    }

    pub fn total_value(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.item_count() {
            if self.get(i) {
                let info = self.item_info(i);
                sum += info.value;
            }
        }
        return sum;
    }
}

fn main() {
    let max_weight = 1.2;
    let mut target_value = 0.0;

    // Search for solutions, increasing target value until there are no solution found.
    loop {
        let bag = Bag::new(max_weight, target_value);

        let settings = SolveSettings::new()
            .debug(false)
            .sleep_ms(100)
        ;
        let solver = BackTrackSolver::new(bag, settings);
        let answer = match solver.solve(|bag| {
            for i in 0..bag.item_count() {
                if !bag.get(i) { return Some(i); }
            }
            return None;
        }) {
            None => break,
            Some(x) => x.puzzle
        };
        answer.print();
        println!("total weight: {}", answer.total_weight());
        println!("total value: {}", answer.total_value());
        println!("~~~");
        target_value = answer.total_value();
    }
}
