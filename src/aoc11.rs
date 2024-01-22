use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Field {
    Galaxy,
    EmptySpace,
    ExpandableSpace(u64),
}

fn load_galaxy(file_content: &str) -> Vec<Vec<Field>> {
    let mut galaxy_map: Vec<Vec<Field>> = Vec::new();

    for line in file_content.split("\n") {
        let mut single_galaxy = Vec::new();
        for char in line.chars() {
            let galaxy_field = match char {
                '#' => Field::Galaxy,
                _ => Field::EmptySpace,
            };
            single_galaxy.push(galaxy_field);
        }
        if single_galaxy.len() > 0 {
            galaxy_map.push(single_galaxy);
        }
    }

    galaxy_map
}

fn expand_galaxy(galaxy_map: &mut Vec<Vec<Field>>, times: u64) {
    // column expansion
    let mut vertical_galaxies_to_expand = Vec::new();
    if let Some(first_galaxy) = &galaxy_map.first() {
        for (idx, field) in first_galaxy.iter().enumerate().rev() {
            match field {
                Field::EmptySpace => {
                    let mut num_of_galaxies = 0;
                    for galaxy in galaxy_map.iter() {
                        num_of_galaxies += match &galaxy.get(idx) {
                            Some(field) => match field {
                                Field::Galaxy => 1,
                                _ => 0,
                            },
                            None => 0,
                        };
                    }

                    match num_of_galaxies {
                        0 => {
                            vertical_galaxies_to_expand.push(idx);
                        }
                        _ => {}
                    }
                }
                _ => {
                    //println!("Not an extension candidate! col {}", idx);
                }
            }
        }
    }
    for galaxy in galaxy_map.iter_mut() {
        for &idx in vertical_galaxies_to_expand.iter() {
            galaxy[idx] = Field::ExpandableSpace(times);
        }
    }

    // row expansion
    let mut galaxies_to_add = Vec::new();
    for (galaxy_idx, galaxy) in galaxy_map.iter().enumerate().rev() {
        let galaxies_count = galaxy.iter().filter(|&x| *x == Field::Galaxy).count();
        match galaxies_count {
            0 => {
                galaxies_to_add.push(galaxy_idx);
            }
            _ => {
                //println!("Not an expansion candidate! row {}", galaxy_idx);
            }
        }
    }
    let single_galaxy_len = match galaxy_map.first() {
        Some(galaxy) => galaxy.len(),
        None => 0,
    };

    for galaxy_to_add in galaxies_to_add {
        galaxy_map[galaxy_to_add] = vec![Field::ExpandableSpace(times); single_galaxy_len];
    }
}

fn find_galaxies(galaxy_map: &Vec<Vec<Field>>) -> Vec<(usize, usize)> {
    let mut galaxies_vec = Vec::new();
    for (row_idx, galaxy_row) in galaxy_map.iter().enumerate() {
        for (col_idx, galaxy_tile) in galaxy_row.iter().enumerate() {
            match galaxy_tile {
                Field::Galaxy => galaxies_vec.push((row_idx, col_idx)),
                _ => (),
            }
        }
    }
    galaxies_vec
}

fn find_expandables(
    galaxy_map: &Vec<Vec<Field>>,
    start_point: (usize, usize),
    end_point: (usize, usize),
) -> (i64, i64) {
    let (mut expandable_rows, mut expandable_cols) = (0, 0);
    for galaxy_row in &galaxy_map[start_point.0..end_point.0] {
        if galaxy_row
            .iter()
            .filter(|&x| !([Field::Galaxy, Field::EmptySpace].contains(x)))
            .count()
            == galaxy_row.len()
        {
            expandable_rows += 1;
        }
    }
    if let Some(&ref first_galaxy) = galaxy_map.first() {
        let (lower_column_idx, higher_column_idx) = match start_point.1 > end_point.1 {
            true => (end_point.1, start_point.1),
            false => (start_point.1, end_point.1),
        };

        for field in &first_galaxy[lower_column_idx..higher_column_idx] {
            match field {
                Field::ExpandableSpace(_) => expandable_cols += 1,
                _ => (),
            };
        }
    }

    (expandable_rows, expandable_cols)
}

fn print_galaxy(galaxy_map: &Vec<Vec<Field>>) {
    for (row_idx, galaxy_row) in galaxy_map.iter().enumerate() {
        for (col_idx, galaxy_tile) in galaxy_row.iter().enumerate() {
            let tile_marker = match galaxy_tile {
                Field::Galaxy => '#',
                Field::EmptySpace => '.',
                Field::ExpandableSpace(_) => 'E',
            };
            println!("{} {} {}", row_idx, col_idx, tile_marker);
        }
        print!("\n");
    }
}

pub fn day11_task01() {
    let input_filepath = match std::env::current_dir() {
        //Ok(filepath) => filepath.join("input_d11_t01"),
        Ok(filepath) => filepath.join("input_d11_test"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut galaxy_map: Vec<Vec<Field>> = load_galaxy(&file_content);

    expand_galaxy(&mut galaxy_map, 1);

    let galaxies = find_galaxies(&galaxy_map);

    let times = 2;
    let mut distances_sum = 0;
    for galaxy_id in 0..galaxies.len() - 1 {
        let galaxy_coordinates = match galaxies.get(galaxy_id) {
            Some(&x) => x,
            None => (0, 0),
        };
        for next_galaxy_id in galaxy_id + 1..galaxies.len() {
            let next_galaxy_coordinates = match galaxies.get(next_galaxy_id) {
                Some(&y) => y,
                None => (0, 0),
            };

            let (expandable_rows, expandable_cols) =
                find_expandables(&galaxy_map, galaxy_coordinates, next_galaxy_coordinates);
            let (higher_col_coord, lower_col_coord) =
                match galaxy_coordinates.1 > next_galaxy_coordinates.1 {
                    true => (galaxy_coordinates.1, next_galaxy_coordinates.1),
                    false => (next_galaxy_coordinates.1, galaxy_coordinates.1),
                };

            let distance = (higher_col_coord as i64 - lower_col_coord as i64).abs()
                + (next_galaxy_coordinates.0 as i64 - galaxy_coordinates.0 as i64).abs()
                + (times - 1) * (expandable_cols + expandable_rows);

            distances_sum += distance;
        }
    }
    println!("Sum distances: {}", distances_sum);

    //print_galaxy(&galaxy_map);
}

pub fn day11_task02() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d11_t01"),
        //Ok(filepath) => filepath.join("input_d11_test"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut galaxy_map: Vec<Vec<Field>> = load_galaxy(&file_content);

    expand_galaxy(&mut galaxy_map, 99999);

    let galaxies = find_galaxies(&galaxy_map);
    let times = 1000000;

    let mut distances_sum = 0;
    for galaxy_id in 0..galaxies.len() - 1 {
        let galaxy_coordinates = match galaxies.get(galaxy_id) {
            Some(&x) => x,
            None => (0, 0),
        };
        for next_galaxy_id in galaxy_id + 1..galaxies.len() {
            let next_galaxy_coordinates = match galaxies.get(next_galaxy_id) {
                Some(&y) => y,
                None => (0, 0),
            };

            let (expandable_rows, expandable_cols) =
                find_expandables(&galaxy_map, galaxy_coordinates, next_galaxy_coordinates);
            let (higher_col_coord, lower_col_coord) =
                match galaxy_coordinates.1 > next_galaxy_coordinates.1 {
                    true => (galaxy_coordinates.1, next_galaxy_coordinates.1),
                    false => (next_galaxy_coordinates.1, galaxy_coordinates.1),
                };

            let distance = (higher_col_coord as i64 - lower_col_coord as i64).abs()
                + (next_galaxy_coordinates.0 as i64 - galaxy_coordinates.0 as i64).abs()
                + (times - 1) * (expandable_cols + expandable_rows);

            distances_sum += distance;
        }
    }
    println!("Sum distances: {}", distances_sum);

    //print_galaxy(&galaxy_map);
}
