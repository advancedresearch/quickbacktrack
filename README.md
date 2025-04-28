# QuickBacktrack
Library for backtracking with customizable search for moves

[Backtracking](https://en.wikipedia.org/wiki/Backtracking) is a general algorithm for finding solutions to constraint satisfaction problems.

With other words, it solves puzzles like [Sudoku](https://en.wikipedia.org/wiki/Sudoku) or [Knapsack](https://en.wikipedia.org/wiki/Knapsack_problem)!

### Features

- Customizable search for moves
- Debug settings for watching the solving in real time
- Solve simple steps to reduce debug output
- Trait `Puzzle` for solving generic constraint satisfaction problems
- Can start with non-empty puzzle
- Can get difference from initial puzzle state

### Sudoku

```text
 ___ ___ ___
|43 | 8 |7  |
|17 | 34|8  |
|85 | 6 |3  |
 ---+---+---
|964|153|287|
|285|497|136|
|713|826|945|
 ---+---+---
|521|349|678|
|347|61 |5  |
|698| 7 |413|
 ---+---+---
Guess [8, 1], 9 depth ch: 6 prev: 38 it: 7
```

To run, open up Terminal and type:

```text
cargo run --example sudoku
```

### Knapsack

```text
Item { desc: "chocolate", weight: 0.2, value: 40.0 }
Item { desc: "book", weight: 0.5, value: 300.0 }
Item { desc: "hat", weight: 0.1, value: 1000.0 }
total weight: 0.80
total value: 1340.00
```

To run, open up Terminal and type:

```text
cargo run --example knapsack
```

### 8 Queens

```text
 _ _ _ _ _ _ _ _
|_|_|_|_|_|_|_|x|
|_|_|_|x|_|_|_|_|
|x|_|_|_|_|_|_|_|
|_|_|x|_|_|_|_|_|
|_|_|_|_|_|_|_|_|
|_|_|_|_|_|_|_|_|
|_|_|_|_|_|_|_|_|
|_|_|_|_|_|_|_|_|

Guess 6, 7 depth ch: 5 prev: 5 it: 72
```

To run, open up Terminal and type:

```text
cargo run --example eight_queens
```

### Performance

The performance of finding a solution can vary greatly with the algorithm used to look for the best position to set a value next. For example, a Sudoku puzzle with many missing numbers can take 59 iterations when picking an empty slot with minimum number of options, but it takes 295 992 iterations to solve when looking for the first empty slot.

One can explain this difference in performance using probability theory. If a slot can contain 2 correct out of N possible values, the odds are `2:(N-2)`.
When this slot is tangled through constraints with another slot with odds `1:(M-1)`, the total odds become `2*1:(N-2)*(M-1)`. To maximize the chance of finding a correct solution, one must maximize the odds for the remaining moves or fail to satisfy the constraints as early as possible. Fewer choices reduces the chances of being wrong, increases the chance of failing constraints early and therefore increases the chance of finding a solution more quickly.

By making the search customizable, one can easier experiment with different algorithms to pick the next best guess and see which has the best performance on a given problem. This library is designed to assist with this kind of exploration.

### Solving simple moves

In some constraint problems, there are lots of steps that are trivial once a choice is made. For example, in Sudoku a lot of numbers can be filled in once a number is selected for a slot.

By solving simple moves separately, one can improve performance and reduce the debugging output significantly.

### Debugging

The relationship between the structure of a puzzle and an efficient algorithm to pick the next best guess can be non-trivial, so understanding what happens is essential for finding an efficient algorithm.

When the setting `SolveSettings::debug(true)` is enabled, the solver prints out the steps to standard output while solving.

The solver prints "Guess" when making a new move, and "Try" when changing an earlier move. Number of iterations are printed at the end when the puzzle is solved.

You can slow down the solving by setting `SolveSettings::sleep_ms(1000)`. This makes the solver wait one second (1000 milliseconds) before continuing to the next step.
