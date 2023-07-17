/* ===== test.rs ===== */

/* ===== Tests ===== */

// fn main() {

//     let input = "ABCD";

//     let mut grid = [['\0'; SIZE]; SIZE];
//     let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRS".chars().collect();
//     construct_grid(&mut grid, &letters).expect("Could not construct grid.");

//     println!("Finding neighbors of A...");
//     print_neighbors(grid, Coordinates{x: 2, y: 2});

//     println!("Finding neighbors of B...");
//     print_neighbors(grid, Coordinates{x: 3, y: 2});

//     println!("Finding neighbors of E...");
//     print_neighbors(grid, Coordinates{x: 1, y: 2});

//     println!("Finding neighbors of S...");
//     print_neighbors(grid, Coordinates{x: 2, y: 0});

//     println!("Finding route for ABCD");
//     let chars: Vec<char> = input.chars().collect();
//     match find_route(grid, &chars, Coordinates{x: 2, y: 2}) {
//         Ok(v) => println!("Found route: {:?}", v),
//         Err(e) => println!("Route not found: {:?}", e)
//     }

//     println!("Finding route for ABHSRQPE");
//     let chars: Vec<char> = "ABHSRQPE".chars().collect();
//     match find_route(grid, &chars, Coordinates{x: 2, y: 2}) {
//         Ok(v) => println!("Found route: {:?}", v),
//         Err(e) => println!("Route not found: {:?}", e)
//     }

//     println!("Finding route for AENDVML");
//     let chars: Vec<char> = "AENDVML".chars().collect();
//     match find_route(grid, &chars, Coordinates{x: 2, y: 2}) {
//         Ok(v) => println!("Found route: {:?}", v),
//         Err(e) => println!("{}", e)
//     }

//     println!("Finding start position for A");
//     match find_tile(grid, 'A') {
//         Some(coords) => println!("Found tile: {:?}", coords),
//         None => println!("Tile not found.")
//     }

//     println!("Finding start position for D");
//     match find_tile(grid, 'D') {
//         Some(coords) => println!("Found tile: {:?}", coords),
//         None => println!("Tile not found.")
//     }

//     println!("Finding start position for K");
//     match find_tile(grid, 'K') {
//         Some(coords) => println!("Found tile: {:?}", coords),
//         None => println!("Tile not found.")
//     }

//     println!("Finding start position for Q");
//     match find_tile(grid, 'Q') {
//         Some(coords) => println!("Found tile: {:?}", coords),
//         None => println!("Tile not found.")
//     }

//     println!("Finding start position for Z");
//     match find_tile(grid, 'Z') {
//         Some(coords) => println!("Found tile: {:?}", coords),
//         None => println!("Tile not found.")
//     }
// }
