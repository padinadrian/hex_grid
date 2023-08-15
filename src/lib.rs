//  ============================================================
//  Filename:   lib.rs
//  Author:     Adrian Padin (padin.adrian@gmail.com)
//  Date:       August 15, 2023
//  ============================================================

//! Solution to Hex Grid challenge problem.

/* ========== Includes ========== */
use std::fmt;

/* ========== Types ========== */

/// 2D grid of letters
pub type Grid = Vec<Vec<char>>;

/* ========== Structs ========== */

/// 2D coordinates
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

/// Print the Coordinates for testing/debugging.
impl fmt::Debug for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

/* ========== Functions ========== */

/// Find all valid neighbors of a given tile.
pub fn find_neighbors(grid: &Grid, coords: Coordinates) -> Vec<Coordinates> {
    let mut neighbors = Vec::new();
    let width = grid.len();

    // Right
    if coords.x < (width - 1) {
        let (x, y) = (coords.x + 1, coords.y);
        if grid[x][y] != '\0' {
            neighbors.push(Coordinates { x, y });
        }
    }
    // Down-Right
    if coords.y < (width - 1) && coords.x < (width - 1) {
        let (x, y) = (coords.x + 1, coords.y + 1);
        if grid[x][y] != '\0' {
            neighbors.push(Coordinates { x, y });
        }
    }
    // Down
    if coords.y < (width - 1) {
        let (x, y) = (coords.x, coords.y + 1);
        if grid[x][y] != '\0' {
            neighbors.push(Coordinates { x, y });
        }
    }
    // Left
    if coords.x > 0 {
        let (x, y) = (coords.x - 1, coords.y);
        if grid[x][y] != '\0' {
            neighbors.push(Coordinates { x, y });
        }
    }
    // Up-Left
    if coords.x > 0 && coords.y > 0 {
        let (x, y) = (coords.x - 1, coords.y - 1);
        if grid[x][y] != '\0' {
            neighbors.push(Coordinates { x, y });
        }
    }
    // Up
    if coords.y > 0 {
        let (x, y) = (coords.x, coords.y - 1);
        if grid[x][y] != '\0' {
            neighbors.push(Coordinates { x, y });
        }
    }

    neighbors
}

/// Found the list of coordinates that correspond to the given sequence of
/// characters.
pub fn find_route(
    grid: &Grid,
    characters: &[char],
    mut start: Coordinates,
) -> Result<Vec<Coordinates>, &'static str> {
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

/// Find the tile in the grid that corresponds to the given letter.
pub fn find_tile(grid: &Grid, start_char: char) -> Option<Coordinates> {
    for x in 0..grid.len() {
        for y in 0..grid.len() {
            if start_char == grid[x][y] {
                return Some(Coordinates { x, y });
            }
        }
    }
    None
}

fn move_right(c: &mut Coordinates) {
    c.x += 1;
}
fn move_down_right(c: &mut Coordinates) {
    c.x += 1;
    c.y += 1;
}
fn move_down(c: &mut Coordinates) {
    c.y += 1;
}
fn move_left(c: &mut Coordinates) {
    c.x -= 1;
}
fn move_up_left(c: &mut Coordinates) {
    c.x -= 1;
    c.y -= 1;
}
fn move_up(c: &mut Coordinates) {
    c.y -= 1;
}

/// Find the inverse Fibonacci (kind of).
fn inv_fibish(mut f_n: usize) -> usize {
    let mut n = 0;
    while f_n > n {
        f_n -= n;
        n += 1
    }
    n
}

/// Given the length of the input string, return the number of layers required.
///
/// For example:
/// * 1 => 1 layer
/// * 2-7 => 2 layers
/// * 8-19 => 3 layers, etc.
///
fn layers_n(i: usize) -> usize {
    match i {
        0 => 0,
        1 => 1,
        _ => inv_fibish(((i - 2) / 6) + 1) + 1,
    }
}

/// Given the length of the input string, returns the required width of the grid.
fn grid_width(i: usize) -> usize {
    match i {
        0 => 0,
        1 => 1,
        _ => ((layers_n(i)) * 2) - 1,
    }
}

/// Fill out the grid given the sequence of characters.
pub fn construct_grid(letters: &[char]) -> Result<Grid, &'static str> {
    // Error checking
    if letters.len() < 1 {
        return Err("Input letters are empty. Requires at least 1 letter to work.");
    }

    // Find width of grid
    let width: usize = grid_width(letters.len());
    let mut grid: Grid = vec![vec!['\0'; width]; width];

    println!("Width: {}", width);

    // Find the center
    let mut start = {
        let center = width / 2;
        Coordinates {
            x: center,
            y: center,
        }
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
        // Move to the right to start a new layer.
        move_right(&mut start);
        grid[start.x][start.y] = letters[n];
        n += 1;
        if n >= letters.len() {
            break;
        }

        // Move down-right by `layer-1` (why -1? because we already moved
        // right 1 to start the new layer).
        for _ in 0..(layer - 1) {
            move_down_right(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1;
            if n >= letters.len() {
                break 'outer;
            }
        }

        // Move down by `layer`
        for _ in 0..layer {
            move_down(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1;
            if n >= letters.len() {
                break 'outer;
            }
        }

        // Move left by `layer`
        for _ in 0..layer {
            move_left(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1;
            if n >= letters.len() {
                break 'outer;
            }
        }

        // Move up-left by `layer`
        for _ in 0..layer {
            move_up_left(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1;
            if n >= letters.len() {
                break 'outer;
            }
        }

        // Move up by `layer`
        for _ in 0..layer {
            move_up(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1;
            if n >= letters.len() {
                break 'outer;
            }
        }

        // Move right by `layer`
        for _ in 0..layer {
            move_right(&mut start);
            grid[start.x][start.y] = letters[n];
            n += 1;
            if n >= letters.len() {
                break 'outer;
            }
        }

        layer += 1;
    }

    Ok(grid)
}

/// Print the grid
pub fn print_grid(grid: &Grid) {
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[x][y] > '\0' {
                print!("  {}", grid[x][y]);
            } else {
                print!("   ");
            }
        }
        println!("");
    }
}

/* ========== Tests ========== */

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Tests for [grid_width]
    ///
    #[test]
    fn grid_width_test() {
        assert_eq!(grid_width(0), 0);
        assert_eq!(grid_width(1), 1);
        for n in 2..=7 {
            assert_eq!(grid_width(n), 3);
        }
        for n in 8..=19 {
            assert_eq!(grid_width(n), 5);
        }
        for n in 20..=37 {
            assert_eq!(grid_width(n), 7);
        }
        for n in 38..=61 {
            assert_eq!(grid_width(n), 9);
        }
    }

    /// Tests for [layers_n]
    ///
    #[test]
    fn layers_n_test() {
        assert_eq!(layers_n(0), 0);
        assert_eq!(layers_n(1), 1);
        for n in 2..=7 {
            assert_eq!(layers_n(n), 2);
        }
        for n in 8..=19 {
            assert_eq!(layers_n(n), 3);
        }
        for n in 20..=37 {
            assert_eq!(layers_n(n), 4);
        }
        for n in 38..=61 {
            assert_eq!(layers_n(n), 5);
        }
    }

    /// Test for [construct_grid]: Error if input is empty
    ///
    #[test]
    fn construct_grid_test_empty() {
        assert!(construct_grid(&[]).is_err());
    }

    /// Test for [construct_grid]: Smallest possible grid
    ///
    #[test]
    fn construct_grid_test_smallest() {
        let expected: Grid = vec![vec!['A']];
        assert_eq!(construct_grid(&['A']).unwrap(), expected);
    }

    /// Test for [construct_grid]: Construct a small grid (7 letters)
    ///
    #[test]
    fn construct_grid_test_small() {
        let expected: Grid = vec![
            vec!['F', 'E', '\0'],
            vec!['G', 'A', 'D'],
            vec!['\0', 'B', 'C'],
        ];
        let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
        assert_eq!(construct_grid(&letters).unwrap(), expected);
    }

    /// Test for [construct_grid]: Construct a medium grid (19 letters)
    ///
    #[test]
    fn construct_grid_test_medium() {
        let expected: Grid = vec![
            vec!['Q', 'P', 'O', '\0', '\0'],
            vec!['R', 'F', 'E', 'N', '\0'],
            vec!['S', 'G', 'A', 'D', 'M'],
            vec!['\0', 'H', 'B', 'C', 'L'],
            vec!['\0', '\0', 'I', 'J', 'K'],
        ];
        let letters: Vec<char> = ('A'..='S').collect();
        assert_eq!(construct_grid(&letters).unwrap(), expected);
    }
}
