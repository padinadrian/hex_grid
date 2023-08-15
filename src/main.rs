//  ============================================================
//  Filename:   main.rs
//  Author:     Adrian Padin (padin.adrian@gmail.com)
//  Date:       August 15, 2023
//  ============================================================

/* ===== Includes ===== */
use hex_grid::{construct_grid, find_route, find_tile, print_grid, Grid};
use std::env;

/* ===== MAIN ===== */

fn main() {
    // Get args from stdin
    let args: Vec<String> = env::args().collect();
    match args.len() {
        x if x < 3 => {
            println!("Too few arguments: {:?}", args);
            return;
        }
        x if x > 3 => {
            println!("Too few arguments: {:?}", args);
            return;
        }
        _ => {}
    }
    let grid_input = &args[1];
    let expected_route = &args[2];

    // Construct grid
    println!("Building grid from {}", grid_input);
    let letters: Vec<char> = grid_input.chars().collect();
    let grid: Grid = construct_grid(&letters).expect("Could not construct grid.");

    println!("Grid:");
    print_grid(&grid);

    // Find route
    let start_char = expected_route.chars().nth(0).unwrap();
    let start = find_tile(&grid, start_char);
    if start.is_none() {
        println!("Could not find starting point for {}", start_char);
        return;
    } else {
        println!("Starting point: {:?}", start.unwrap());
    }

    println!("Finding route for {}", expected_route);
    let chars: Vec<char> = expected_route.chars().collect();
    match find_route(&grid, &chars, start.unwrap()) {
        Ok(v) => println!("Found route: {:?}", v),
        Err(_) => println!("Route not found."),
    }
}
