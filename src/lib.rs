/* ===== lib.rs ===== */

/* ===== Includes ===== */
use std::fmt;


/* ===== Global Data ===== */

const SIZE: usize = 5;
type Grid = [[char; SIZE]; SIZE];


/* ===== Structs ===== */

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize
}

/* Print the Coordinates for testing/debugging. */
impl fmt::Debug for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}


/* ===== Functions ===== */

/* Find all valid neighbors of a given tile. */
pub fn find_neighbors(grid: Grid, coords: Coordinates) -> Vec<Coordinates> {
    let mut neighbors = Vec::new();
    let width = grid.len();

    // Right
    if coords.x < (width - 1) {
        let (x, y) = (coords.x + 1, coords.y);
        if grid[x][y] != '\0' { neighbors.push(Coordinates{x: x, y: y}); }
    }
    // Down-Right
    if coords.y < (width - 1) && coords.x < (width - 1) {
        let (x, y) = (coords.x + 1, coords.y + 1);
        if grid[x][y] != '\0' { neighbors.push(Coordinates{x: x, y: y}); }
    }
    // Down
    if coords.y < (width - 1) {
        let (x, y) = (coords.x, coords.y + 1);
        if grid[x][y] != '\0' { neighbors.push(Coordinates{x: x, y: y}); }
    }
    // Left
    if coords.x > 0 {
        let (x, y) = (coords.x - 1, coords.y);
        if grid[x][y] != '\0' { neighbors.push(Coordinates{x: x, y: y}); }
    }
    // Up-Left
    if coords.x > 0 && coords.y > 0 {
        let (x, y) = (coords.x - 1, coords.y - 1);
        if grid[x][y] != '\0' { neighbors.push(Coordinates{x: x, y: y}); }
    }
    // Up
    if coords.y > 0 {
        let (x, y) = (coords.x, coords.y - 1);
        if grid[x][y] != '\0' { neighbors.push(Coordinates{x: x, y: y}); }
    }

    neighbors
}

/* Found the list of coordinates that correspond to the given sequence of characters. */
pub fn find_route(grid: Grid, characters: &[char], mut start: Coordinates) ->
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
        let neighbors = find_neighbors(grid, start);
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

/* Find the tile in the grid that corresponds to the given letter. */
pub fn find_tile(grid: Grid, start_char: char) -> Option<Coordinates> {
    for x in 0..grid.len() {
        for y in 0..grid.len() {
            if start_char == grid[x][y] {
                return Some(Coordinates{x: x, y: y});
            }
        }
    }
    None
}

/* Functions to move around the grid. */
fn move_right(c: &mut Coordinates) { c.x += 1; }
fn move_down_right(c: &mut Coordinates) { c.x += 1; c.y += 1; }
fn move_down(c: &mut Coordinates) { c.y += 1; }
fn move_left(c: &mut Coordinates) { c.x -= 1; }
fn move_up_left(c: &mut Coordinates) { c.x -= 1; c.y -= 1; }
fn move_up(c: &mut Coordinates) { c.y -= 1; }

/* Fill out the grid given the sequence of characters. */
pub fn construct_grid(grid: &mut Grid, letters: &[char]) -> Result<usize, &'static str> {

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
        move_right(&mut start);
        grid[start.x][start.y] = letters[n];
        n += 1; if n >= letters.len() { break; }

        // Move down-right by `layer-1` (why -1? because we already moved right 1 to start the new layer)
        for _ in 0..(layer-1) {
            move_down_right(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move down by `layer`
        for _ in 0..layer {
            move_down(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move left by `layer`
        for _ in 0..layer {
            move_left(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move up-left by `layer`
        for _ in 0..layer {
            move_up_left(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move up by `layer`
        for _ in 0..layer {
            move_up(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        // Move right by `layer`
        for _ in 0..layer {
            move_right(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1; if n >= letters.len() { break 'outer; }
        }

        layer += 1;
    }

    Ok(letters.len())
}
