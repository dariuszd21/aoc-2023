use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Tile {
    ASH,
    ROCK,
}

#[derive(Debug)]
enum Solution {
    Vertical(u64),
    Horizontal(u64),
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

fn compare_tiles_rows(tiles_row: &Vec<Tile>, other_tiles_row: &Vec<Tile>) -> bool {
    let mut equal = true;
    for j in 0..tiles_row.len() {
        if tiles_row[j] != other_tiles_row[j] {
            equal = false;
            break;
        }
    }

    equal
}

fn find_solution(tiles: &Vec<Vec<Tile>>) -> Solution {
    let mut solution: Solution = Solution::Vertical(0);

    let tiles_num = tiles.len();
    for i in 1..tiles_num {
        let tiles_row = tiles.get(i).unwrap();
        let prev_row = tiles.get(i - 1).unwrap();
        let equal = compare_tiles_rows(&tiles_row, &prev_row);
        if equal {
            let mut check = true;
            for k in (0..i - 1).rev() {
                let difference = (i - k) + i - 1;

                if difference < tiles_num {
                    check &=
                        compare_tiles_rows(&tiles.get(k).unwrap(), &tiles.get(difference).unwrap());
                }

                if !check {
                    break;
                }
            }
            if check {
                solution = Solution::Horizontal(i.try_into().unwrap());
            }
            println!(
                "Found equal rows {}\n {:?}\n {:?}\n",
                i, prev_row, tiles_row
            );
            println!("Current solution: {:?}", solution,);
        }
    }

    solution
}

fn solve(file_path: &Path) -> u64 {
    let res = 0;
    let file_content = fs::read_to_string(file_path).expect("File could not be loaded");
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    for line in file_content.split("\n") {
        if line.trim().is_empty() {
            for t in &tiles {
                println!("Tilerow {:?}", t);
            }
            find_solution(&tiles);
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
