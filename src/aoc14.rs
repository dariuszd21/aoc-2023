use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    EmptySpace,
    RoundRock,
    SqureRock,
}

enum Direction {
    North,
    South,
    West,
    East,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::EmptySpace => write!(f, "."),
            Tile::RoundRock => write!(f, "O"),
            Tile::SqureRock => write!(f, "#"),
        }
    }
}

fn vec_from_str(s: &str) -> Vec<Tile> {
    let mut tiles_row = Vec::new();

    for c in s.chars() {
        match c {
            '.' => tiles_row.push(Tile::EmptySpace),
            '#' => tiles_row.push(Tile::SqureRock),
            'O' => tiles_row.push(Tile::RoundRock),
            _ => (),
        }
    }
    tiles_row
}

fn load_platform(file_path: &Path) -> Vec<Vec<Tile>> {
    let mut tiles = Vec::new();
    let file_content = fs::read_to_string(file_path).expect("File could not be loaded");
    for line in file_content.split("\n") {
        let tiles_row = vec_from_str(line);
        if !tiles_row.is_empty() {
            tiles.push(tiles_row);
        }
    }

    tiles
}

fn update_tile(tiles: &mut Vec<Vec<Tile>>, pos_i: usize, pos_j: usize, val: Tile) {
    tiles[pos_i][pos_j] = val;
}

fn tilt_platform(tiles: &Vec<Vec<Tile>>, direction: Direction) -> Vec<Vec<Tile>> {
    let mut tilted_tiles = Vec::new();

    for tile_row in tiles {
        tilted_tiles.push(tile_row.clone());
    }

    let num_rows = tilted_tiles.len();

    let num_cols = match num_rows > 0 {
        true => tilted_tiles[0].len(),
        false => 0,
    };

    match direction {
        Direction::North => {
            for i in 0..num_rows {
                for j in 0..num_cols {
                    let tile = tiles[i][j];
                    match tile {
                        Tile::RoundRock => {
                            let mut new_row = i;
                            for new_i in (0..i).rev() {
                                if tilted_tiles[new_i][j] == Tile::EmptySpace {
                                    new_row = new_i;
                                    continue;
                                }
                                break;
                            }
                            if new_row != i {
                                update_tile(&mut tilted_tiles, new_row, j, Tile::RoundRock);
                                update_tile(&mut tilted_tiles, i, j, Tile::EmptySpace);
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
        Direction::South => {
            for i in (0..num_rows).rev() {
                for j in (0..num_cols).rev() {
                    let tile = tiles[i][j];
                    match tile {
                        Tile::RoundRock => {
                            let mut new_row = i;
                            for new_i in i + 1..num_cols {
                                if tilted_tiles[new_i][j] == Tile::EmptySpace {
                                    new_row = new_i;
                                    continue;
                                }
                                break;
                            }
                            if new_row != i {
                                update_tile(&mut tilted_tiles, new_row, j, Tile::RoundRock);
                                update_tile(&mut tilted_tiles, i, j, Tile::EmptySpace);
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
        Direction::West => {
            for i in 0..num_rows {
                for j in 0..num_cols {
                    let tile = tiles[i][j];
                    match tile {
                        Tile::RoundRock => {
                            let mut new_col = j;
                            for new_j in (0..j).rev() {
                                if tilted_tiles[i][new_j] == Tile::EmptySpace {
                                    new_col = new_j;
                                    continue;
                                }
                                break;
                            }
                            if new_col != j {
                                update_tile(&mut tilted_tiles, i, new_col, Tile::RoundRock);
                                update_tile(&mut tilted_tiles, i, j, Tile::EmptySpace);
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
        Direction::East => {
            for i in (0..num_rows).rev() {
                for j in (0..num_cols).rev() {
                    let tile = tiles[i][j];
                    match tile {
                        Tile::RoundRock => {
                            let mut new_col = j;
                            for new_j in j + 1..num_cols {
                                if tilted_tiles[i][new_j] == Tile::EmptySpace {
                                    new_col = new_j;
                                    continue;
                                }
                                break;
                            }
                            if new_col != j {
                                update_tile(&mut tilted_tiles, i, new_col, Tile::RoundRock);
                                update_tile(&mut tilted_tiles, i, j, Tile::EmptySpace);
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
    };

    tilted_tiles
}

fn print_tiles(tiles: &Vec<Vec<Tile>>) {
    for row in tiles {
        println!(
            "{}",
            row.iter()
                .map(|&v| v.to_string())
                .collect::<Vec<_>>()
                .join("")
        );
    }
}

pub fn solve_part_01(file_path: &Path) -> u64 {
    let mut res = 0;
    let tiles = load_platform(file_path);
    print_tiles(&tiles);

    println!("\n\n----------\n\n");

    let tilted = tilt_platform(&tiles, Direction::North);
    print_tiles(&tilted);

    let num_rows = tiles.len();
    for (i, tile_row) in tilted.iter().enumerate() {
        for tile in tile_row {
            res += match tile {
                Tile::EmptySpace => 0,
                Tile::RoundRock => (num_rows - i) as u64,
                Tile::SqureRock => 0,
            };
        }
        println!("Current load {}", res);
    }

    res
}

pub fn solve_part_02(file_path: &Path) -> u64 {
    let mut res = 0;
    let tiles = load_platform(file_path);
    print_tiles(&tiles);

    println!("\n\n----------\n\n");

    let mut state = Vec::new();
    for row in &tiles {
        state.push(row.clone());
    }

    // Calculate when cycle starts and how long cycle is
    // so we can calculated what's the state after billion cycles

    let mut cycle = Vec::new();
    cycle.push(tiles.clone());

    loop {
        state = tilt_platform(&state, Direction::North);
        state = tilt_platform(&state, Direction::West);
        state = tilt_platform(&state, Direction::South);
        state = tilt_platform(&state, Direction::East);
        if cycle.contains(&state) {
            println!("Found cycle!");
            break;
        }
        cycle.push(state.clone());
    }

    let first_cycle_el_pos = cycle.iter().position(|x| *x == state).unwrap();
    let cycle_len = cycle.len() - first_cycle_el_pos;
    println!(
        "Cycle starts at {} (len {})!",
        first_cycle_el_pos, cycle_len,
    );
    let state_after_multiple_cycles = (1_000_000_000 - first_cycle_el_pos) % cycle_len;

    state = cycle[state_after_multiple_cycles + first_cycle_el_pos].clone();
    print_tiles(&state);
    let num_rows = tiles.len();
    for (i, tile_row) in state.iter().enumerate() {
        for tile in tile_row {
            res += match tile {
                Tile::EmptySpace => 0,
                Tile::RoundRock => (num_rows - i) as u64,
                Tile::SqureRock => 0,
            };
        }
        println!("Current load {}", res);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solving_d14_part1() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d14_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        assert_eq!(solve_part_01(&input_filepath), 136);
    }

    #[test]
    fn test_solving_d14_part2() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d14_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        assert_eq!(solve_part_02(&input_filepath), 64);
    }
}
