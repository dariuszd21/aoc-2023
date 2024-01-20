use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq)]
enum Direction {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
    Ground,
}

//    L is a 90-degree bend connecting north and east.
//    J is a 90-degree bend connecting north and west.
//    7 is a 90-degree bend connecting south and west.
//    F is a 90-degree bend connecting south and east.
//    . is ground; there is no pipe in this tile.

fn recursive_map_pass(
    pos: (usize, usize),
    current_distance: u64,
    map: &HashMap<(usize, usize), Direction>,
    distance_map: &mut HashMap<(usize, usize), u64>,
) {
    if let Some(&cal_distance) = distance_map.get(&pos) {
        if current_distance < cal_distance {
            distance_map.insert(pos, current_distance);
        } else {
            return;
        }
    };
    distance_map.insert(pos, current_distance);
    let new_distance = current_distance + 1;

    if pos.0 > 0 {
        let upper_tile = (pos.0 - 1, pos.1);
        if let Some(&ref dir) = map.get(&upper_tile) {
            match dir {
                Direction::Vertical => {
                    recursive_map_pass(upper_tile, new_distance, map, distance_map)
                }
                Direction::SouthEast => {
                    recursive_map_pass(upper_tile, new_distance, map, distance_map)
                }
                Direction::SouthWest => {
                    recursive_map_pass(upper_tile, new_distance, map, distance_map)
                }
                _ => (),
            };
        }
    }
    let lower_tile = (pos.0 + 1, pos.1);
    if let Some(&ref dir) = map.get(&lower_tile) {
        match dir {
            Direction::Vertical => recursive_map_pass(lower_tile, new_distance, map, distance_map),
            Direction::NorthEast => recursive_map_pass(lower_tile, new_distance, map, distance_map),
            Direction::NorthWest => recursive_map_pass(lower_tile, new_distance, map, distance_map),
            _ => (),
        };
    }

    if pos.1 > 0 {
        let left_tile = (pos.0, pos.1 - 1);
        if let Some(&ref dir) = map.get(&left_tile) {
            match dir {
                Direction::Horizontal => {
                    recursive_map_pass(left_tile, new_distance, map, distance_map)
                }
                Direction::NorthEast => {
                    recursive_map_pass(left_tile, new_distance, map, distance_map)
                }
                Direction::SouthEast => {
                    recursive_map_pass(left_tile, new_distance, map, distance_map)
                }
                _ => (),
            };
        }
    }

    let right_tile = (pos.0, pos.1 + 1);
    if let Some(&ref dir) = map.get(&right_tile) {
        match dir {
            Direction::Horizontal => {
                recursive_map_pass(right_tile, new_distance, map, distance_map)
            }
            Direction::NorthWest => recursive_map_pass(right_tile, new_distance, map, distance_map),
            Direction::SouthWest => recursive_map_pass(right_tile, new_distance, map, distance_map),
            _ => (),
        };
    }
}

fn create_map(file_content: &str) -> HashMap<(usize, usize), Direction> {
    let mut map: HashMap<(usize, usize), Direction> = HashMap::new();

    for (row_idx, row) in file_content.split("\n").enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            let dir = match char {
                '|' => Direction::Vertical,
                '-' => Direction::Horizontal,
                'L' => Direction::NorthEast,
                'J' => Direction::NorthWest,
                '7' => Direction::SouthWest,
                'F' => Direction::SouthEast,
                'S' => Direction::Start,
                _ => Direction::Ground,
            };

            map.insert((row_idx, col_idx), dir);
        }
    }

    return map;
}

fn find_start(map: &HashMap<(usize, usize), Direction>) -> (usize, usize) {
    let mut start_idx: (usize, usize) = (0, 0);
    for (pos, val) in map.iter() {
        match val {
            Direction::Start => {
                start_idx = *pos;
            }
            _ => (),
        }
    }
    return start_idx;
}

pub fn day10_task1() {
    let input_filepath = match std::env::current_dir() {
        //Ok(filepath) => filepath.join("input_d10_test"),
        Ok(filepath) => filepath.join("input_d10_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut distance_map: HashMap<(usize, usize), u64> = HashMap::new();
    let map = create_map(&file_content);
    let start_idx = find_start(&map);

    recursive_map_pass(start_idx, 0, &map, &mut distance_map);

    println!("{} {}", start_idx.0, start_idx.1);

    if let Some((max_idx, v)) = distance_map
        .iter()
        .max_by(|(_, val1), (_, val2)| val1.cmp(val2))
    {
        println!("{},{} {}", max_idx.0, max_idx.1, v);
    }
}
