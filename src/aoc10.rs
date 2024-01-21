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

#[derive(PartialEq, Eq)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
enum TileMarker {
    Pipe,
    Inside,
    Outside,
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
    let current_direction = match map.get(&pos) {
        Some(dir) => dir,
        None => &Direction::Start,
    };

    let north = match current_direction {
        Direction::Start => true,
        Direction::Vertical => true,
        Direction::NorthWest => true,
        Direction::NorthEast => true,
        _ => false,
    };
    let south = match current_direction {
        Direction::Start => true,
        Direction::Vertical => true,
        Direction::SouthWest => true,
        Direction::SouthEast => true,
        _ => false,
    };

    let west = match current_direction {
        Direction::Start => true,
        Direction::Horizontal => true,
        Direction::NorthWest => true,
        Direction::SouthWest => true,
        _ => false,
    };

    let east = match current_direction {
        Direction::Start => true,
        Direction::Horizontal => true,
        Direction::NorthEast => true,
        Direction::SouthEast => true,
        _ => false,
    };

    if pos.0 > 0 && north {
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
    if south {
        if let Some(&ref dir) = map.get(&lower_tile) {
            match dir {
                Direction::Vertical => {
                    recursive_map_pass(lower_tile, new_distance, map, distance_map)
                }
                Direction::NorthEast => {
                    recursive_map_pass(lower_tile, new_distance, map, distance_map)
                }
                Direction::NorthWest => {
                    recursive_map_pass(lower_tile, new_distance, map, distance_map)
                }
                _ => (),
            };
        }
    }

    if pos.1 > 0 && west {
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
    if east {
        if let Some(&ref dir) = map.get(&right_tile) {
            match dir {
                Direction::Horizontal => {
                    recursive_map_pass(right_tile, new_distance, map, distance_map)
                }
                Direction::NorthWest => {
                    recursive_map_pass(right_tile, new_distance, map, distance_map)
                }
                Direction::SouthWest => {
                    recursive_map_pass(right_tile, new_distance, map, distance_map)
                }
                _ => (),
            };
        }
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

fn print_map(max_coordinate: (usize, usize), tile_map: &HashMap<(usize, usize), TileMarker>) {
    for i in 0..max_coordinate.0 {
        for j in 0..max_coordinate.1 {
            if let Some(&ref tile_marker) = tile_map.get(&(i, j)) {
                let tile_char = match tile_marker {
                    TileMarker::Pipe => '*',
                    TileMarker::Outside => 'O',
                    TileMarker::Inside => 'I',
                };

                print!("{}", tile_char);
            } else {
                print!("U");
            }
        }
        print!("\n");
    }
}

fn mark_tile_outside(
    tile_idx: (usize, usize),
    tile_map: &mut HashMap<(usize, usize), TileMarker>,
) -> TileMarker {
    match tile_map.get(&tile_idx) {
        Some(tile_marker) => match tile_marker {
            TileMarker::Outside => TileMarker::Outside,
            TileMarker::Pipe => TileMarker::Pipe,
            TileMarker::Inside => {
                tile_map.insert(tile_idx, TileMarker::Outside);
                TileMarker::Outside
            }
        },
        None => {
            tile_map.insert(tile_idx, TileMarker::Outside);
            TileMarker::Outside
        }
    }
}

fn mark_tile_inside(
    tile_idx: (usize, usize),
    tile_map: &mut HashMap<(usize, usize), TileMarker>,
) -> TileMarker {
    match tile_map.get(&tile_idx) {
        Some(tile_marker) => match tile_marker {
            TileMarker::Inside => TileMarker::Inside,
            TileMarker::Pipe => TileMarker::Pipe,
            TileMarker::Outside => {
                tile_map.insert(tile_idx, TileMarker::Inside);
                TileMarker::Inside
            }
        },
        None => {
            tile_map.insert(tile_idx, TileMarker::Inside);
            TileMarker::Inside
        }
    }
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

    if let Some((max_idx, v)) = distance_map
        .iter()
        .max_by(|(_, val1), (_, val2)| val1.cmp(val2))
    {
        println!("{},{} {}", max_idx.0, max_idx.1, v);
    }
}

pub fn day10_task2() {
    let input_filepath = match std::env::current_dir() {
        //Ok(filepath) => filepath.join("input_d10_test"),
        Ok(filepath) => filepath.join("input_d10_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let map = create_map(&file_content);
    let start_idx = find_start(&map);
    let mut distance_map: HashMap<(usize, usize), u64> = HashMap::new();

    recursive_map_pass(start_idx, 0, &map, &mut distance_map);

    if let Some((max_idx, v)) = distance_map
        .iter()
        .max_by(|(_, val1), (_, val2)| val1.cmp(val2))
    {
        println!("{},{} {}", max_idx.0, max_idx.1, v);
    }

    let mut tile_map: HashMap<(usize, usize), TileMarker> = HashMap::new();

    let mut movement_direction = Movement::Down;
    let mut loop_item = start_idx;
    for (item_idx, distance) in distance_map.iter() {
        if *distance == (1 as u64) {
            loop_item = item_idx.clone();
            break;
        }
    }
    println!("Starting point {} {}", loop_item.0, loop_item.1);

    if let Some(&max_coordinate) = map.keys().max() {
        loop {
            if loop_item == start_idx {
                break;
            }
            let (loop_item_row_idx, loop_item_col_idx) = loop_item;
            if let Some(&ref direction) = map.get(&(loop_item_row_idx, loop_item_col_idx)) {
                tile_map.insert((loop_item_row_idx, loop_item_col_idx), TileMarker::Pipe);
                movement_direction = match movement_direction {
                    Movement::Up => {
                        for row in (0..loop_item_row_idx).rev() {
                            let tile_coords = (row, loop_item_col_idx);
                            match mark_tile_outside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for col in loop_item_col_idx + 1..=max_coordinate.1 {
                            let tile_coords = (loop_item_row_idx, col);
                            match mark_tile_outside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for col in (0..loop_item_col_idx).rev() {
                            let tile_coords = (loop_item_row_idx, col);
                            match mark_tile_inside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        loop_item = match direction {
                            Direction::SouthWest => (loop_item_row_idx, loop_item_col_idx - 1),
                            Direction::SouthEast => (loop_item_row_idx, loop_item_col_idx + 1),
                            _ => (loop_item_row_idx - 1, loop_item_col_idx),
                        };
                        match direction {
                            Direction::SouthWest => Movement::Left,
                            Direction::SouthEast => Movement::Right,
                            _ => movement_direction,
                        }
                    }
                    Movement::Down => {
                        for row in loop_item_row_idx + 1..=max_coordinate.0 {
                            let tile_coords = (row, loop_item_col_idx);
                            match mark_tile_outside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for col in loop_item_col_idx + 1..=max_coordinate.1 {
                            let tile_coords = (loop_item_row_idx, col);
                            match mark_tile_inside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for col in (0..loop_item_col_idx).rev() {
                            let tile_coords = (loop_item_row_idx, col);
                            match mark_tile_outside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }

                        loop_item = match direction {
                            Direction::NorthEast => (loop_item_row_idx, loop_item_col_idx + 1),
                            Direction::NorthWest => (loop_item_row_idx, loop_item_col_idx - 1),
                            _ => (loop_item_row_idx + 1, loop_item_col_idx),
                        };

                        match direction {
                            Direction::NorthWest => Movement::Left,
                            Direction::NorthEast => Movement::Right,
                            _ => movement_direction,
                        }
                    }
                    Movement::Left => {
                        for col in (0..loop_item_col_idx).rev() {
                            let item_idx = (loop_item_row_idx, col);
                            match mark_tile_outside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for row in loop_item_row_idx + 1..=max_coordinate.0 {
                            let item_idx = (row, loop_item_col_idx);
                            match mark_tile_inside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for row in (0..loop_item_row_idx).rev() {
                            let item_idx = (row, loop_item_col_idx);
                            match mark_tile_outside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }

                        loop_item = match direction {
                            Direction::NorthEast => (loop_item_row_idx - 1, loop_item_col_idx),
                            Direction::SouthEast => (loop_item_row_idx + 1, loop_item_col_idx),
                            _ => (loop_item_row_idx, loop_item_col_idx - 1),
                        };
                        match direction {
                            Direction::SouthEast => Movement::Down,
                            Direction::NorthEast => Movement::Up,
                            _ => movement_direction,
                        }
                    }
                    Movement::Right => {
                        for col in loop_item_col_idx + 1..=max_coordinate.1 {
                            let item_idx = (loop_item_row_idx, col);
                            match mark_tile_outside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for row in loop_item_row_idx + 1..=max_coordinate.0 {
                            let item_idx = (row, loop_item_col_idx);
                            match mark_tile_outside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for row in (0..loop_item_row_idx).rev() {
                            let item_idx = (row, loop_item_col_idx);
                            match mark_tile_inside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        loop_item = match direction {
                            Direction::NorthWest => (loop_item_row_idx - 1, loop_item_col_idx),
                            Direction::SouthWest => (loop_item_row_idx + 1, loop_item_col_idx),
                            _ => (loop_item_row_idx, loop_item_col_idx + 1),
                        };
                        match direction {
                            Direction::SouthWest => Movement::Down,
                            Direction::NorthWest => Movement::Up,
                            _ => movement_direction,
                        }
                    }
                };
                match movement_direction {
                    Movement::Up => {
                        for row in (0..loop_item_row_idx).rev() {
                            let tile_coords = (row, loop_item_col_idx);
                            match mark_tile_outside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for col in loop_item_col_idx + 1..=max_coordinate.1 {
                            let tile_coords = (loop_item_row_idx, col);
                            match mark_tile_outside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for col in (0..loop_item_col_idx).rev() {
                            let tile_coords = (loop_item_row_idx, col);
                            match mark_tile_inside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                    }
                    Movement::Down => {
                        for row in loop_item_row_idx + 1..=max_coordinate.0 {
                            let tile_coords = (row, loop_item_col_idx);
                            match mark_tile_outside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for col in loop_item_col_idx + 1..=max_coordinate.1 {
                            let tile_coords = (loop_item_row_idx, col);
                            match mark_tile_inside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for col in (0..loop_item_col_idx).rev() {
                            let tile_coords = (loop_item_row_idx, col);
                            match mark_tile_outside(tile_coords, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                    }
                    Movement::Left => {
                        for col in (0..loop_item_col_idx).rev() {
                            let item_idx = (loop_item_row_idx, col);
                            match mark_tile_outside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for row in loop_item_row_idx + 1..=max_coordinate.0 {
                            let item_idx = (row, loop_item_col_idx);
                            match mark_tile_inside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for row in (0..loop_item_row_idx).rev() {
                            let item_idx = (row, loop_item_col_idx);
                            match mark_tile_outside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                    }
                    Movement::Right => {
                        for col in loop_item_col_idx + 1..=max_coordinate.1 {
                            let item_idx = (loop_item_row_idx, col);
                            match mark_tile_outside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for row in loop_item_row_idx + 1..=max_coordinate.0 {
                            let item_idx = (row, loop_item_col_idx);
                            match mark_tile_outside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                        for row in (0..loop_item_row_idx).rev() {
                            let item_idx = (row, loop_item_col_idx);
                            match mark_tile_inside(item_idx, &mut tile_map) {
                                TileMarker::Pipe => break,
                                _ => (),
                            }
                        }
                    }
                };
            }
            //print_map(max_coordinate, &tile_map);

            //match movement_direction {
            //   Movement::Up => println!("Going UP"),
            //   Movement::Down => println!("Going DOWN"),
            //   Movement::Left => println!("Going LEFT"),
            //   Movement::Right => println!("Going RIGHT"),
            //};
        }
        print_map(max_coordinate, &tile_map);
        println!(
            "Inside {}",
            tile_map
                .values()
                .filter(|&x| *x == TileMarker::Inside)
                .count()
        );
    }
}
