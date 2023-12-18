use std::{fs, collections::HashMap};

enum Direction {
    Right,
    Left,
}

fn parse_instructions(line: &str) -> Vec<Direction> {
    let mut directions = Vec::new();
    for c in line.chars() {
        match c {
            'R' => directions.push(Direction::Right),
            'L' => directions.push(Direction::Left),
            _ => (),
        }
    }
    
    directions
}

fn add_crossroad_to_map(map: &mut HashMap<String, (String,String)>, line: &str) {
    let splitted_line: Vec<_> = line.split(" = ").collect();
    if splitted_line.len() == 2 {
        let key = splitted_line[0].to_string();

        let splitted_values: Vec<_> = splitted_line[1].split(", ").collect();

        if splitted_values.len() == 2 {
            let (left, right) = (splitted_values[0].strip_prefix("(").unwrap().to_string(), splitted_values[1].strip_suffix(")").unwrap().to_string());
            
            map.insert(key, (left, right));
        }
    }

}

pub fn day08_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d08_t01"),
        //Ok(filepath) => filepath.join("input_d08_test2"),
        //Ok(filepath) => filepath.join("input_d08_test"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");
    let mut directions = Vec::new();

    let mut map: HashMap<_, _> = HashMap::new();

    for (idx, line) in file_content.split("\n").enumerate() {
        if idx == 0 {
            directions = parse_instructions(line);
            continue;
        }
        add_crossroad_to_map(&mut map, line);
    }

    if let Some(next_steps) = map.get("AAA") {
        //for d in directions.iter().cycle() {
        let (mut left, mut right) = next_steps.clone();
        //for d in directions.iter() {
        let mut steps = 0;
        for d in directions.iter().cycle() {
            steps += 1;
            match d {
                Direction::Left => {
                    println!("Going left to: {}", left);
                    if left == "ZZZ" {
                        println!("Finished");
                        break;
                    }
                    match map.get(&left) {
                        Some((l, r)) => {
                            left = l.to_string();
                            right = r.to_string();
                        }
                        None => (),
                    }
                }
                Direction::Right => {
                    println!("Going right to: {}", right);
                    if right == "ZZZ" {
                        println!("Finished");
                        break;
                    }
                    match map.get(&right) {
                        Some((l, r)) => {
                            left = l.to_string();
                            right = r.to_string();
                        }
                        None => (),
                    }
                
                }
                
            }
        }

        println!("Find finish in {} steps", steps);
        
    }

    
}
