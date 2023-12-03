use std::fs;

fn mapper(given_string: &str) -> u8 {
    let mut number = String::new();
    for char in given_string.chars() {
            if char.is_digit(10) {
                number += &char.to_string();
                break;
            }
        }
        for char in given_string.chars().rev() {
            if char.is_digit(10) {
                number += &char.to_string();
                break;
            }
        }
    println!("{} {}", given_string, number);
    match number.is_empty() {
        false => number.parse::<u8>().expect("Cannot parse"),
        true => 0,
    }
}

fn day01_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d01_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath)
        .expect("File could not be loaded");

    let splitted_content = file_content.split("\n");

    let sum = splitted_content.map(mapper).fold(0, |total, v| (total + v as u64));
    println!("Sum {}:", sum);
    for line in file_content.split("\n") {
        println!("{} {}", line, mapper(line));
        break;
    }
}

fn main() {
    day01_task01();
}
