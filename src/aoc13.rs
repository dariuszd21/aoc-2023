use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    ASH,
    ROCK,
}

#[derive(Debug)]
enum Solution {
    Vertical { col: u64 },
    Horizontal { row: u64 },
}

fn vec_from_str(s: &str) -> Vec<Tile> {
    let mut tiles_row = Vec::new();

    for c in s.chars() {
        match c {
            '.' => tiles_row.push(Tile::ASH),
            '#' => tiles_row.push(Tile::ROCK),
            _ => (),
        }
    }
    tiles_row
}

fn compare_tiles(tiles: &Vec<Tile>, cmp_tiles: &Vec<Tile>) -> bool {
    let mut equal = true;
    for j in 0..tiles.len() {
        if tiles[j] != cmp_tiles[j] {
            equal = false;
            break;
        }
    }

    equal
}

fn extract_column(tiles: &Vec<Vec<Tile>>, col: usize) -> Vec<Tile> {
    let mut extracted_column = Vec::new();
    for tile_row in tiles {
        match tile_row.get(col) {
            Some(tile) => extracted_column.push(tile.clone()),
            None => (),
        }
    }
    extracted_column
}

fn find_solution(tiles: &Vec<Vec<Tile>>) -> Solution {
    let mut solution: Solution = Solution::Vertical { col: 0 };

    let tiles_rows = tiles.len();
    for i in 1..tiles_rows {
        let tiles_row = tiles.get(i).unwrap();
        let prev_row = tiles.get(i - 1).unwrap();
        let equal = compare_tiles(&tiles_row, &prev_row);
        if equal {
            let mut check = true;
            // check if rest of the rows are also mirrored
            for k in (0..i - 1).rev() {
                let difference = (i - k) + i - 1;

                if difference < tiles_rows {
                    check &= compare_tiles(&tiles.get(k).unwrap(), &tiles.get(difference).unwrap());
                }

                if !check {
                    break;
                }
            }
            if check {
                solution = Solution::Horizontal {
                    row: i.try_into().unwrap(),
                };
            }
            println!(
                "Found equal rows {}\n {:?}\n {:?}\n",
                i, prev_row, tiles_row
            );
        }
    }

    let tiles_columns = tiles.get(0).unwrap().len();
    for i in 1..tiles_columns {
        let tiles_column = extract_column(tiles, i);
        let prev_col = extract_column(tiles, i - 1);
        let equal = compare_tiles(&tiles_column, &prev_col);
        if equal {
            let mut check = true;
            // check if rest of the rows are also mirrored
            for k in (0..i - 1).rev() {
                let difference = (i - k) + i - 1;
                let left_hand_column = extract_column(tiles, k);

                if difference < tiles_columns {
                    let mirror_column = extract_column(tiles, difference);
                    check &= compare_tiles(&left_hand_column, &mirror_column);
                }

                if !check {
                    break;
                }
            }
            if check {
                solution = Solution::Vertical {
                    col: i.try_into().unwrap(),
                };
            }
            println!(
                "Found equal columns {}\n {:?}\n {:?}\n",
                i, prev_col, tiles_column
            );
        }
    }

    solution
}

fn solve(file_path: &Path) -> u64 {
    let mut res = 0;
    let file_content = fs::read_to_string(file_path).expect("File could not be loaded");
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    for line in file_content.split("\n") {
        if line.trim().is_empty() {
            for t in &tiles {
                println!("Tilerow {:?}", t);
            }
            let solution = find_solution(&tiles);
            println!("Current solution: {:?}", solution,);

            res += match solution {
                Solution::Vertical { col } => col,
                Solution::Horizontal { row } => 100 * row,
            };

            tiles.clear();
            continue;
        }
        let tiles_row = vec_from_str(line);

        tiles.push(tiles_row);
    }
    println!("Tiles {:?}", tiles);
    res
}

pub fn day13_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d13_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };
    let result = solve(&input_filepath);
    println!("And the result is {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_row() {}

    #[test]
    fn test_solving_d13() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d13_t01_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        let result = solve(&input_filepath);
        assert_eq!(result, 405);
    }
}
