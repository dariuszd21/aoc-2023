use std::fs;

fn fill_vector_with_values(line: &str, vec: &mut Vec<u64>) {
    let splitted_line: Vec<_> = line.split(":").collect();
    if splitted_line.len() == 2 {
        let times_str = splitted_line[1].split(" ");
        for time_str in times_str {
            match time_str.parse::<u64>() {
                Ok(val) => vec.push(val),
                Err(_) => (),
            }
        }
    }
}

fn fill_vector_with_value(line: &str, vec: &mut Vec<u64>) {
    let splitted_line: Vec<_> = line.split(":").collect();
    if splitted_line.len() == 2 {
        let time_str = splitted_line[1].replace(" ", "");
        match time_str.parse::<u64>() {
            Ok(val) => vec.push(val),
            Err(_) => (),
        }
    }
}

pub fn day06_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d06_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for line in file_content.split("\n") {
        if line.starts_with("Time") {
            fill_vector_with_values(line, &mut times);
            continue;
        }

        if line.starts_with("Distance") {
            fill_vector_with_values(line, &mut distances);
        }
    }

    let mut final_result = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut current = 0;
        for t in 0..*time {
            if (time - t) * t > *distance {
                //println!("Holding for t={}: Passes", t);
                current += 1;
            }
        }
        println!(
            "Time: {}, Distance: {}, Possibilities: {}",
            time, distance, current
        );
        final_result *= current;
    }

    println!("Final result: {}", final_result);
}

pub fn day06_task02() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d06_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for line in file_content.split("\n") {
        if line.starts_with("Time") {
            fill_vector_with_value(line, &mut times);
            continue;
        }

        if line.starts_with("Distance") {
            fill_vector_with_value(line, &mut distances);
        }
    }

    let mut final_result = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut current = 0;
        for t in 0..*time {
            if (time - t) * t > *distance {
                //println!("Holding for t={}: Passes", t);
                current += 1;
            }
        }
        println!(
            "Time: {}, Distance: {}, Possibilities: {}",
            time, distance, current
        );
        final_result *= current;
    }

    println!("Final result: {}", final_result);
}
