use std::{collections::HashMap, fs};

fn parse_numbers(numbers_space_separated: &str) -> Vec<u64> {
    let numbers: Vec<_> = numbers_space_separated
        .split(" ")
        .filter_map(|v| match v.parse::<u64>() {
            Ok(i_val) => Some(i_val),
            _ => None,
        })
        .collect();
    numbers
}

pub fn day04_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d04_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let splitted_content = file_content.split("\n");

    let mut sum = 0;
    for line in splitted_content {
        let splitted_card: Vec<_> = line.split(":").collect();
        if splitted_card.len() == 2 {
            let (_card_name, values) = (splitted_card[0], splitted_card[1]);
            let splitted_values: Vec<_> = values.split("|").collect();
            if splitted_values.len() == 2 {
                let winning_values = parse_numbers(splitted_values[0]);
                let elf_values = parse_numbers(splitted_values[1]);
                println!("Winning values: {}", winning_values.len());
                let winning_elf_numbers: Vec<_> = winning_values
                    .iter()
                    .filter(|x| elf_values.contains(x))
                    .collect();
                println!("Winning elf values: {}", winning_elf_numbers.len());
                sum += match winning_elf_numbers.len() {
                    val if val > 0 => 2u64.pow(val as u32 - 1),
                    _ => 0,
                }
            }
        }
    }
    println!("Sum of winning powers: {}", sum);
}

pub fn day04_task02() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d04_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let splitted_content = file_content.split("\n");

    let mut scratchcards: HashMap<usize, u64> = HashMap::new();
    for (idx, line) in splitted_content.enumerate() {
        let card_idx = idx + 1;
        let splitted_card: Vec<_> = line.split(":").collect();
        if splitted_card.len() == 2 {
            let (_card_name, values) = (splitted_card[0], splitted_card[1]);
            let splitted_values: Vec<_> = values.split("|").collect();
            if splitted_values.len() == 2 {
                let winning_values = parse_numbers(splitted_values[0]);
                let elf_values = parse_numbers(splitted_values[1]);
                let scratchcards_amount = match scratchcards.get(&card_idx) {
                    Some(val) => val + 1,
                    _ => 1,
                };
                scratchcards.insert(card_idx, scratchcards_amount);
                println!("Number of cards: {} {}", card_idx, scratchcards_amount);
                let winning_elf_numbers: Vec<_> = winning_values
                    .iter()
                    .filter(|x| elf_values.contains(x))
                    .collect();
                let winning_cards = winning_elf_numbers.len();
                println!("Winning elf values: {}", winning_cards);
                for val in 0..winning_cards {
                    let won_card_id = card_idx + val + 1;
                    println!(
                        "Increasing card {} number by: {}",
                        won_card_id, scratchcards_amount
                    );

                    let scratchcards_amount = match scratchcards.get(&won_card_id) {
                        Some(val) => val + scratchcards_amount,
                        _ => scratchcards_amount,
                    };
                    scratchcards.insert(won_card_id, scratchcards_amount);
                }
            }
        }
    }
    println!(
        "Sum of winning powers: {}",
        scratchcards.values().fold(0, |total, x| total + x)
    );
}
