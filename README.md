# quickbacktrack
Library for back tracking with customizable search for moves

[Back tracking](https://en.wikipedia.org/wiki/Backtracking) is a general algorithm for finding solutions to constraint satisfaction problems.

With other words, it solves puzzles like [Sudoku](https://en.wikipedia.org/wiki/Sudoku) or [Knapsack](https://en.wikipedia.org/wiki/Knapsack_problem)!

### Features

- Customizable search for moves
- Debug settings for watching the solving in real time
- Solve simple steps to reduce debug output
- Trait `Puzzle` for solving generic constraint satisfaction problems
- Can start with non-empty puzzle
- Get get difference from initial puzzle state

### Sudoku

```
 ___ ___ ___
|436| 8 |751|
|17 | 34|8 9|
|859|761|324|
 ---+---+---
|964|153|287|
|285|497|136|
|713|826|945|
 ---+---+---
|521|349|678|
|347|618|592|
|698| 7 |413|
 ---+---+---
Guess [5, 0], 2 depth 50 51
```

To run, open up Terminal and type:

```
cargo run --example sudoku
```

### Knapsack

```
Item { desc: "chocolate", weight: 0.2, value: 40 }
Item { desc: "book", weight: 0.5, value: 300 }
Item { desc: "hat", weight: 0.1, value: 1000 }
total weight: 0.7999999999999999
total value: 1340
```

To run, open up in Terminal and type:

```
cargo run --example knapsack
```

