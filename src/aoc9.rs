use std::fs;

fn parse_row_to_sequence(line: &str) -> Vec<i64> {
    let mut v = Vec::new();

    for str_val in line.split(" ") {
        match str_val.parse::<i64>() {
            Ok(num_val) => v.push(num_val),
            Err(_) => (),
        }
    }

    v
}

fn find_next_value(v: &Vec<i64>) -> i64 {
    let mut differences: Vec<Vec<i64>> = Vec::new();
    differences.push(v.clone());

    loop {
        let prev_vec = differences.last().unwrap();
        let mut next_differences = Vec::new();

        let mut prev_item = prev_vec[0];
        for i in 1..prev_vec.len() {
            let current_item = prev_vec[i];
            next_differences.push(current_item - prev_item);
            prev_item = current_item;
        }

        let number_of_zeros = next_differences.iter().clone().filter(|v| **v == 0).count();
        differences.push(next_differences.clone());
        if number_of_zeros == next_differences.len() {
            break;
        }
    }

    let mut prev_last_item = 0;
    let mut last_item = 0;
    for v in differences.iter_mut().rev() {
        last_item = v.last().unwrap() + prev_last_item;
        v.push(last_item);
        prev_last_item = *v.last().unwrap();
    }

    last_item
}

fn find_prev_value(v: &Vec<i64>) -> i64 {
    let mut differences: Vec<Vec<i64>> = Vec::new();
    differences.push(v.clone());

    loop {
        let prev_vec = differences.last().unwrap();
        let mut next_differences = Vec::new();

        let mut prev_item = prev_vec[0];
        for i in 1..prev_vec.len() {
            let current_item = prev_vec[i];
            next_differences.push(current_item - prev_item);
            prev_item = current_item;
        }

        let number_of_zeros = next_differences.iter().clone().filter(|v| **v == 0).count();
        differences.push(next_differences.clone());
        if number_of_zeros == next_differences.len() {
            break;
        }
    }

    let mut prev_first = 0;
    let mut first_item = 0;
    for v in differences.iter_mut().rev() {
        let first = *v.first().unwrap();
        first_item = first - prev_first;
        v.insert(0, first_item);
        prev_first = first_item;
    }

    first_item
}
pub fn day09() {
    let input_filepath = match std::env::current_dir() {
        //Ok(filepath) => filepath.join("input_d09_test"),
        Ok(filepath) => filepath.join("input_d09_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut sum = 0;
    let mut prev_sum = 0;
    for line in file_content.split("\n") {
        let seq = parse_row_to_sequence(line);
        if seq.len() > 0 {
            sum += find_next_value(&seq);
            prev_sum += find_prev_value(&seq);
        }
    }
    println!("Sum of next values is: {}", sum);
    println!("Sum of prev values is: {}", prev_sum);
}
