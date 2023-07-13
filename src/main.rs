
/* ===== Includes ===== */
// use std::str::FromStr;
// use std::env;
use std::fmt;

/* ===== Functions ===== */
// fn gcd(mut n: u64, mut m: u64) -> u64 {
//     assert!(n != 0 && m != 0);
//     while m != 0 {
//         if m < n {
//             (m, n) = (n, m);
//         }
//         m %= n;
//     }
//     n
// }

#[derive(Copy, Clone)]
// #[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Coordinates {
    x: usize,
    y: usize
}


impl fmt::Debug for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

fn find_neighbors(coords: Coordinates, size: usize) -> Vec<Coordinates> {
    let mut neighbors = Vec::new();
    if coords.x < (size - 1) {
        neighbors.push(Coordinates{x: coords.x + 1, y: coords.y});      // Right
    }
    if coords.y < (size - 1) && coords.x < (size - 1) {
        neighbors.push(Coordinates{x: coords.x + 1, y: coords.y + 1});  // Down-Right
    }
    if coords.y < (size - 1) {
        neighbors.push(Coordinates{x: coords.x, y: coords.y + 1});      // Down
    }
    if coords.x > 0 {
        neighbors.push(Coordinates{x: coords.x - 1, y: coords.y});      // Left
    }
    if coords.x > 0 && coords.y > 0 {
        neighbors.push(Coordinates{x: coords.x - 1, y: coords.y - 1});  // Up-Left
    }
    if coords.y > 0 {
        neighbors.push(Coordinates{x: coords.x, y: coords.y - 1});      // Up
    }
    neighbors
}

fn is_valid_coordinates(coords: Coordinates, size: usize) -> bool {
    let result = (coords.x < size) && (coords.y < size);
    
    // println!("Is ({}, {}) valid? {}", coords.x, coords.y, result);
    
    result
}

const SIZE: usize = 5;
type Grid = [[char; SIZE]; SIZE];

fn print_neighbors(grid: Grid, coords: Coordinates) {
    let neighbors = find_neighbors(coords, SIZE);
    println!("Length of neighbors: {}", neighbors.len());
    for it in &neighbors {
        println!("Found neighbor: {}", grid[it.x][it.y]);
        // if is_valid_coordinates(*it, SIZE) {
        // }
    }
}

fn find_route(grid: Grid, characters: &[char], mut start: Coordinates) ->
    Result<Vec<Coordinates>, &'static str>
{
    // First character must match starting point
    if characters[0] != grid[start.x][start.y] {
        return Err("Invalid start");
    }

    // Start with empty route 
    let mut route = Vec::new();
    route.push(start);

    for c in &characters[1..] {
        // Check if any of the immediate neighbors matches the next letter.
        let neighbors = find_neighbors(start, grid.len());
        let prev = start;
        for n in neighbors {
            if *c == grid[n.x][n.y] {
                // Found next step in route - add to route and break immediately.
                route.push(n);
                start = n;
                break;
            }
        }
        // If the starting point hasn't changed, we couldn't find the next step.
        if start == prev {
            return Err("Path not found.");
        }
    }
    
    Ok(route)
}


fn find_tile(grid: Grid, start_char: char) -> Option<Coordinates> {
    for x in 0..grid.len() {
        for y in 0..grid.len() {
            if start_char == grid[x][y] {
                return Some(Coordinates{x: x, y: y});
            }
        }
    }
    None
}

fn move_right(c: &mut Coordinates) { c.x += 1; }
fn move_down_right(c: &mut Coordinates) { c.x += 1; c.y += 1; }
fn move_down(c: &mut Coordinates) { c.y += 1; }
fn move_left(c: &mut Coordinates) { c.x -= 1; }
fn move_up_left(c: &mut Coordinates) { c.x -= 1; c.y -= 1; }
fn move_up(c: &mut Coordinates) { c.y -= 1; }

fn construct_grid(grid: &mut Grid, letters: &[char]) -> Result<usize, &'static str> {

    // Find the center
    let mut start = {
        let center = grid.len() / 2;
        Coordinates{x: center, y: center}
    };

    println!("Center: {:?}", start);

    // Start with center
    grid[start.x][start.y] = letters[0];

    // Place letters around center in an outward spiral
    // Move in a pattern: DR, D, L, UL, U, R
    // TODO: Check for if grid is too small for number of letters
    let mut layer = 1;
    let mut n = 1;
    'outer: while n < letters.len() {
        // Move to the right to start a new layer
        println!("Moving right");
        move_right(&mut start);
        println!("at: {:?}", start);
        println!("placing n={}", n);
        grid[start.x][start.y] = letters[n];
        n += 1; if n >= letters.len() { break; }

        // Move down-right by `layer-1` (why -1? because we already moved right 1 to start the new layer)
        for _ in 0..(layer-1) {
            println!("Moving down-right");
            move_down_right(&mut start);
            println!("at: {:?}", start);
            println!("placing n={}", n);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move down by `layer`
        for _ in 0..layer {
            println!("Moving down");
            move_down(&mut start);
            println!("at: {:?}", start);
            println!("placing n={}", n);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move left by `layer`
        for _ in 0..layer {
            println!("Moving left");
            move_left(&mut start);
            println!("at: {:?}", start);
            println!("placing n={}", n);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move up-left by `layer`
        for _ in 0..layer {
            println!("Moving up-left");
            move_up_left(&mut start);
            println!("at: {:?}", start);
            println!("placing n={}", n);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move up by `layer`
        for _ in 0..layer {
            println!("Moving up");
            move_up(&mut start);
            println!("at: {:?}", start);
            println!("placing n={}", n);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move right by `layer`
        for _ in 0..layer {
            println!("Moving right");
            move_right(&mut start);
            println!("at: {:?}", start);
            println!("placing n={}", n);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        layer += 1;
    }

    Ok(letters.len())
}

/* ===== MAIN ===== */

fn main() {
    
    let input = "ABCD";

    let mut grid = [['0'; SIZE]; SIZE];
    let letters: Vec<char> = "ABCDEFGHIJK".chars().collect();
    construct_grid(&mut grid, &letters).expect("Could not construct grid.");
    // grid[0][0] = 'Q';
    // grid[0][1] = 'P';
    // grid[0][2] = 'O';
    
    // grid[1][0] = 'R';
    // grid[1][1] = 'F';
    // grid[1][2] = 'E';
    // grid[1][3] = 'N';
    
    // grid[2][0] = 'S';
    // grid[2][1] = 'G';
    // grid[2][2] = 'A';
    // grid[2][3] = 'D';
    // grid[2][4] = 'M';
    
    // grid[3][1] = 'H';
    // grid[3][2] = 'B';
    // grid[3][3] = 'C';
    // grid[3][4] = 'L';
    
    // grid[4][2] = 'I';
    // grid[4][3] = 'J';
    // grid[4][4] = 'K';

    println!("Finding neighbors of A...");
    print_neighbors(grid, Coordinates{x: 2, y: 2});

    println!("Finding neighbors of B...");
    print_neighbors(grid, Coordinates{x: 3, y: 2});

    println!("Finding neighbors of E...");
    print_neighbors(grid, Coordinates{x: 1, y: 2});
    
    println!("Finding neighbors of S...");
    print_neighbors(grid, Coordinates{x: 2, y: 0});

    println!("Finding route for ABCD");
    let chars: Vec<char> = input.chars().collect();
    match find_route(grid, &chars, Coordinates{x: 2, y: 2}) {
        Ok(v) => println!("Found route: {:?}", v),
        Err(e) => println!("Route not found: {:?}", e)
    }

    println!("Finding route for ABHSRQPE");
    let chars: Vec<char> = "ABHSRQPE".chars().collect();
    match find_route(grid, &chars, Coordinates{x: 2, y: 2}) {
        Ok(v) => println!("Found route: {:?}", v),
        Err(e) => println!("Route not found: {:?}", e)
    }

    println!("Finding route for AENDVML");
    let chars: Vec<char> = "AENDVML".chars().collect();
    match find_route(grid, &chars, Coordinates{x: 2, y: 2}) {
        Ok(v) => println!("Found route: {:?}", v),
        Err(e) => println!("{}", e)
    }
    
    println!("Finding start position for A");
    match find_tile(grid, 'A') {
        Some(coords) => println!("Found tile: {:?}", coords),
        None => println!("Tile not found.")
    }
    
    println!("Finding start position for D");
    match find_tile(grid, 'D') {
        Some(coords) => println!("Found tile: {:?}", coords),
        None => println!("Tile not found.")
    }
    
    println!("Finding start position for K");
    match find_tile(grid, 'K') {
        Some(coords) => println!("Found tile: {:?}", coords),
        None => println!("Tile not found.")
    }
    
    println!("Finding start position for Q");
    match find_tile(grid, 'Q') {
        Some(coords) => println!("Found tile: {:?}", coords),
        None => println!("Tile not found.")
    }
    
    println!("Finding start position for Z");
    match find_tile(grid, 'Z') {
        Some(coords) => println!("Found tile: {:?}", coords),
        None => println!("Tile not found.")
    }
}

