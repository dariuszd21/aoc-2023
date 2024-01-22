use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Field {
    Galaxy,
    EmptySpace,
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

fn expand_galaxy(galaxy_map: &mut Vec<Vec<Field>>) {
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
                                Field::EmptySpace => 0,
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
                Field::Galaxy => {
                    println!("Not an extension candidate! {}", idx);
                }
            }
        }
    }
    for galaxy in galaxy_map.iter_mut() {
        for &idx in vertical_galaxies_to_expand.iter() {
            galaxy.insert(idx, Field::EmptySpace);
        }
    }

    // row expansion
    let mut galaxies_to_add = Vec::new();
    for (galaxy_idx, galaxy) in galaxy_map.iter().enumerate().rev() {
        let galaxies_count = galaxy.iter().filter(|&x| *x == Field::Galaxy).count();
        match galaxies_count {
            0 => {
                galaxies_to_add.push(galaxy_idx);
            },
            _ => {
                println!("Not an expansion candidate row {}", galaxy_idx);
            },
        }
    }
    let single_galaxy_len = match galaxy_map.first() {
        Some(galaxy) => galaxy.len(),
        None => 0,
    };

    for galaxy_to_add in galaxies_to_add {
        galaxy_map.insert(galaxy_to_add, vec![Field::EmptySpace; single_galaxy_len])
    }
}

fn print_galaxy(galaxy_map: &Vec<Vec<Field>>) {
    for (row_idx, galaxy_row) in galaxy_map.iter().enumerate() {
        for (col_idx, galaxy_tile) in galaxy_row.iter().enumerate() {
            let tile_marker = match galaxy_tile {
                Field::Galaxy => '#',
                Field::EmptySpace => '.',
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

    expand_galaxy(&mut galaxy_map);

    print_galaxy(&galaxy_map);
}
