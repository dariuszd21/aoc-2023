use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    EmptySpace,
    RoundRock,
    SqureRock,
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

fn tilt_platform(tiles: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut tilted_tiles = Vec::new();

    for tile_row in tiles {
        tilted_tiles.push(tile_row.clone());
    }

    let num_rows = tilted_tiles.len();

    let num_cols = match num_rows > 0 {
        true => tilted_tiles[0].len(),
        false => 0,
    };

    println!("{} {}", num_rows, num_cols);

    for i in 0..num_rows {
        for j in 0..num_cols {
            let tile = tiles[i][j];
            match tile {
                Tile::EmptySpace => (),
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
                Tile::SqureRock => (),
            }
        }
    }

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

    let tilted = tilt_platform(&tiles);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solving_d13() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d14_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        assert_eq!(solve_part_01(&input_filepath), 136);
    }
}
