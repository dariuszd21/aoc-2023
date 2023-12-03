use std::fs;

fn mapper(given_string: &str) -> u8 {
    let mut number = String::new();
    let patterns = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let mut f_index = given_string.len();
    let mut f_pattern = "";
    let mut l_index = 0;
    let mut l_pattern = "";
    for pat in patterns {
        if let Some(index) = given_string.find(&pat.to_string()) {
            if index < f_index {
                f_index = index;
                f_pattern = pat;
            }
        }
        if let Some(index) = given_string.rfind(&pat.to_string()) {
            if index >= l_index {
                l_index = index;
                l_pattern = pat;
            }
        }
    }

    match f_pattern {
        "one" => number += "1",
        "two" => number += "2",
        "three" => number += "3",
        "four" => number += "4",
        "five" => number += "5",
        "six" => number += "6",
        "seven" => number += "7",
        "eight" => number += "8",
        "nine" => number += "9",
        &_ => number += f_pattern,
    }

    match l_pattern {
        "one" => number += "1",
        "two" => number += "2",
        "three" => number += "3",
        "four" => number += "4",
        "five" => number += "5",
        "six" => number += "6",
        "seven" => number += "7",
        "eight" => number += "8",
        "nine" => number += "9",
        &_ => number += l_pattern,
    }

    match number.is_empty() {
        false => number.parse::<u8>().expect("Cannot parse"),
        true => 0,
    }
}

pub fn day01_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d01_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let splitted_content = file_content.split("\n");

    let sum = splitted_content
        .map(mapper)
        .fold(0, |total, v| (total + v as u64));
    println!("Sum {}:", sum);
    for line in file_content.split("\n") {
        println!("{} {}", line, mapper(line));
        break;
    }
}
