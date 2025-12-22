# Game of Life

## Introduction
* This is ___Conway's Game of Life___, by the British Mathematician __John Horton Conway__.
* It is a ___Cellular Automaton___
*  A cellular automaton (CA) is a __grid of cells__, like pixels on a screen, where each cell has a state (e.g. __on/off__ or __dead/alive__) and changes its __state__ over __discrete time steps__ based on simple, local rules applied to its __neighbors__, leading to complex, emergent patterns.

### Key Concepts
* __Grid__: A regular array of cells, which can be 1D, 2D, or higher dimensional.
* __States__: Each cell exists in one of a finite number of states (e.g., black/white, 0/1).
* __Neighborhood__: A defined set of surrounding cells for each cell.
* __Rules__: A fixed set of rules that determine a cell's next state based on its current state and its neighbors' states.
* __Time Steps__: The system evolves in discrete steps, with all cells updating simultaneously.

### Game of Life: The Rules
### The Setup
* The game is played on an infinite 2D grid of square cells.
* Each cell is either alive or dead.
* Time advances in discrete steps (generations).
* Each cell interacts with its 8 neighbors (horizontal, vertical, diagonal).

#### The Rules (applied simultaneously each generation)

1. Underpopulation
* A live cell with fewer than 2 live neighbors dies.

2. Survival
* A live cell with 2 or 3 live neighbors stays alive.

3. Overpopulation
* A live cell with more than 3 live neighbors dies.

4. Reproduction
* A dead cell with exactly 3 live neighbors becomes alive.

##### Key Notes
* No randomness, no players — the system evolves purely from the initial state.
* Simple rules lead to complex behavior: oscillators, spaceships, still lifes, and chaotic patterns.
  
### Requirements
* `Rust rustc 1.81.0`.
* `Cargo`.
* Text Editor for any updates/changes you want to make.

### Running the Program
* `$ cd GameOfLife`
* `$ cargo build`
* `$ cargo run`
