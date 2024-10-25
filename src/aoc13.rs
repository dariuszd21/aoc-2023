use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    ASH,
    ROCK,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn find_horizontal_solution(tiles: &Vec<Vec<Tile>>) -> Option<Solution> {
    let mut solution = Solution::Vertical(0);
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
                solution = Solution::Horizontal(i.try_into().unwrap());
            }
        }
    }

    match solution {
        Solution::Vertical(_) => None,
        Solution::Horizontal(_) => Some(solution),
    }
}

fn find_vertical_solution(tiles: &Vec<Vec<Tile>>) -> Option<Solution> {
    let mut solution = Solution::Horizontal(0);
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
                solution = Solution::Vertical(i.try_into().unwrap());
            }
        }
    }

    match solution {
        Solution::Vertical(_) => Some(solution),
        _ => None,
    }
}

fn find_new_horizontal_solution(
    tiles: &Vec<Vec<Tile>>,
    prev_solution: &Solution,
) -> Option<Solution> {
    let mut solution = Solution::Vertical(0);
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
                let tmp_solution = Solution::Horizontal(i.try_into().unwrap());

                if tmp_solution != *prev_solution {
                    solution = tmp_solution;
                }
            }
        }
    }

    match solution {
        Solution::Vertical(_) => None,
        Solution::Horizontal(_) => Some(solution),
    }
}

fn find_new_vertical_solution(
    tiles: &Vec<Vec<Tile>>,
    prev_solution: &Solution,
) -> Option<Solution> {
    let mut solution = Solution::Horizontal(0);
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
                let tmp_solution = Solution::Vertical(i.try_into().unwrap());

                if tmp_solution != *prev_solution {
                    solution = tmp_solution;
                }
            }
        }
    }

    match solution {
        Solution::Vertical(_) => Some(solution),
        _ => None,
    }
}

fn find_solution(tiles: &Vec<Vec<Tile>>) -> Solution {
    if let Some(solution) = find_horizontal_solution(tiles) {
        println!("Found horizontal solution {:?}", solution);
        return solution;
    }

    if let Some(solution) = find_vertical_solution(tiles) {
        println!("Found vertical solution {:?}", solution);
        return solution;
    }

    return Solution::Vertical(0);
}

fn solve(file_path: &Path) -> u64 {
    let mut res = 0;
    let file_content = fs::read_to_string(file_path).expect("File could not be loaded");
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    for line in file_content.split("\n") {
        if line.trim().is_empty() {
            let solution = find_solution(&tiles);
            println!("Current solution: {:?}", solution,);

            res += match solution {
                Solution::Vertical(col) => col,
                Solution::Horizontal(row) => 100 * row,
            };

            tiles.clear();
            continue;
        }
        let tiles_row = vec_from_str(line);

        tiles.push(tiles_row);
    }
    res
}

fn find_other_solution(tiles: &Vec<Vec<Tile>>, solution: &Solution) -> Option<Solution> {
    println!("\n#############################");
    println!("Old solution: {:?}", solution);
    for tile_row in tiles {
        for i in 0..tiles.len() {
            for j in 0..tile_row.len() {
                let mut new_tiles = Vec::new();
                for tile_row in tiles {
                    new_tiles.push(tile_row.clone());
                }
                new_tiles[i][j] = match new_tiles.get(i).unwrap().get(j).unwrap() {
                    Tile::ASH => Tile::ROCK,
                    Tile::ROCK => Tile::ASH,
                };

                if let Some(new_solution) = find_new_horizontal_solution(&new_tiles, solution) {
                    return Some(new_solution);
                }
                if let Some(new_solution) = find_new_vertical_solution(&new_tiles, solution) {
                    return Some(new_solution);
                }
            }
        }
    }
    println!("#############################\n");
    None
}

fn solve_part_2(file_path: &Path) -> u64 {
    let mut res = 0;
    let file_content = fs::read_to_string(file_path).expect("File could not be loaded");
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    for line in file_content.split("\n") {
        if line.trim().is_empty() {
            let solution = find_solution(&tiles);
            let other_solution = find_other_solution(&tiles, &solution);
            println!("New vertical solution {:?}", other_solution);

            res += match other_solution {
                Some(Solution::Vertical(col)) => col,
                Some(Solution::Horizontal(row)) => 100 * row,
                None => {
                    for tile in &tiles {
                        println!("Unsolved {:?}", tile);
                    }
                    println!("No solution for {:?}", solution);
                    match solution {
                        Solution::Vertical(col) => col,
                        Solution::Horizontal(row) => 100 * row,
                    }
                }
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

pub fn day13_task02() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d13_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };
    let result = solve_part_2(&input_filepath);
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

    #[test]
    fn test_solving_d13_part2() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d13_t02_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        let result = solve_part_2(&input_filepath);
        assert_eq!(result, 400);
    }

    #[test]
    fn test_enum_equality() {
        let s = Solution::Horizontal(5);
        let s2 = Solution::Horizontal(2);

        assert_eq!(s, s2);
    }
}
