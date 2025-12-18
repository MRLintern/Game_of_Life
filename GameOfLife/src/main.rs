// --------------------------------------------------------- //
//
// Conway's Game of Life
//
// this program simulates Conways Game of Life
// on a fixed 20 x 20 grid and displays it in the terminal
//
// --------------------------------------------------------- //
//
// ------------------------------------- //
//
// External imports
//
// ------------------------------------- //

use rand::random;
use std::{
    env, //for command-line arguments
    fs::File, // opening files
    io::{BufRead, BufReader}, // efficient line-by-line reading
    thread,
    time::Duration,
};

use termion::{clear, color}; // terminal control

// ----------------------------------------------- //
// constants and type aliases
// ----------------------------------------------- //

// size of the square grid (75 x 75)
const N: usize = 20;

// a type alias to make signatures readable
//
// World is a 2D array of booleans:
//  * true = alive cell
//  * false = dead cell

type World = [[bool; N]; N];

fn main() {
    use std::thread;
    use std::time::Duration;

    // ----------------------------------
    // Create and initialize the world
    // ----------------------------------
    let mut world: World = [[false; N]; N];

    // Random initial state
    for i in 0..N {
        for j in 0..N {
            world[i][j] = rand::random::<bool>();
        }
    }

    // Track generation count
    let mut generation: usize = 0;

    // ----------------------------------
    // Main simulation loop
    // ----------------------------------
    loop {
        // Compute the next generation
        let next = next_generation(&world);

        // Render the world:
        // - world = previous generation
        // - next  = current generation
        display_world(&world, &next, generation);

        // Advance simulation state
        world = next;
        generation += 1;

        // Slow down the animation
        thread::sleep(Duration::from_millis(400));
    }
}


// ------------------------------------------------ //
//
// NOTE: Optional - if you want to load a text file containing the coordinates of cells in the grid
//
// load a world from a text file
//
// ------------------------------------------------ //
//
// each line in the file represents one live cell
// E.g.
//
//  10  20
//  11  20
//  12  20
//  13  20
//

fn populate_from_file(filename: &str) -> World {
    // start with an empty world
    let mut world: World = [[false; N]; N];

    // open the file (panic if it fails)
    let file = File::open(filename)
        .expect("Failed to open pattern file");

    // wrap the file in a buffered reader
    let reader = BufReader::new(file);

    // read each line
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        // split the line into 2 numbers
        let mut parts = line.split_whitespace();

        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();

        // mark the cell as alive
        world[x][y] = true;
    }

    world
}

// ----------------------------------------------------- //
// display the world in the terminal
// ----------------------------------------------------- //
//
// alive cells are shown as green +; dead cells are red X
// dead cells are shown as spaces

fn display_world(prev: &World, current: &World, generation: usize) {
    use termion::color;

    // Clear screen and move cursor to top-left
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    // -------- STATUS LINE --------
    let pop = population(current);
    println!(
        "{}Generation {} | Population {}{}",
        color::Fg(color::LightBlue),
        generation,
        pop,
        color::Fg(color::Reset)
    );
    println!();

    // -------- TOP BORDER --------
    print!("+");
    for _ in 0..N {
        print!("---+");
    }
    println!();

    // -------- GRID --------
    for i in 0..N {
        print!("|");
        for j in 0..N {
            let was_alive = prev[i][j];
            let is_alive = current[i][j];

            match (was_alive, is_alive) {
                (true, true) => print!(
                    " {}*{} |",
                    color::Fg(color::LightGreen),
                    color::Fg(color::Reset)
                ),
                (false, true) => print!(
                    " {}+{} |",
                    color::Fg(color::Green),
                    color::Fg(color::Reset)
                ),
                (true, false) => print!(
                    " {}x{} |",
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                ),
                (false, false) => print!("   |"),
            }
        }
        println!();

        // Row separator
        print!("+");
        for _ in 0..N {
            print!("---+");
        }
        println!();
    }
}

// function to show Generation and Population number
fn population(world: &World) -> usize {
    world.iter()
         .flatten()
         .filter(|&&cell| cell)
         .count()
}


// ----------------------------------------------------//
// Compute the next generation
// ----------------------------------------------------//
//
// This implements Conway’s Game of Life rules:
//
// 1. Any live cell with fewer than 2 neighbors dies
// 2. Any live cell with 2 or 3 neighbors survives
// 3. Any live cell with more than 3 neighbors dies
// 4. Any dead cell with exactly 3 neighbors becomes alive

fn next_generation(world: &World) -> World {
    // Create a new empty world
    let mut next: World = [[false; N]; N];

    for x in 0..N {
        for y in 0..N {
            let alive = world[x][y];
            let neighbors = count_neighbors(world, x, y);

            // Apply the rules
            next[x][y] = match (alive, neighbors) {
                (true, 2 | 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    next
}


// ------------------------------------------------------- //
// Count live neighbors around a cell
// ------------------------------------------------------- //
//
// We check the 8 surrounding cells carefully
// without going out of bounds.

fn count_neighbors(world: &World, x: usize, y: usize) -> usize {
    let mut count = 0;

    // Offsets to move around the cell
    for dx in [-1isize, 0, 1] {
        for dy in [-1isize, 0, 1] {
            // Skip the cell itself
            if dx == 0 && dy == 0 {
                continue;
            }

            // Convert to signed coordinates
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // Bounds check
            if nx >= 0 && ny >= 0 && nx < N as isize && ny < N as isize {
                if world[nx as usize][ny as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}


// ------------------------------------------------------- //
// Count total live cells (population)
// ------------------------------------------------------- //
//
// This function demonstrates iterator-based Rust.

fn census(world: &World) -> usize {
    world
        .iter()          // iterate over rows
        .flatten()       // flatten into cells
        .filter(|&&c| c) // keep only live cells
        .count()
}
