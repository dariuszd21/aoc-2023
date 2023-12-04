use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Number {
    value: u64,
    start_index: usize,
    number_len: usize,
}

enum Token {
    Symbol(usize),
    Number(Number),
}

fn tokenizer(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut number = String::new();

    let clear_number = |idx: &usize, number: &mut String, tokens: &mut Vec<Token>| {
        if !number.is_empty() {
            match number.parse::<u64>() {
                Ok(val) => {
                    tokens.push(Token::Number(Number {
                        value: val,
                        start_index: idx - number.len(),
                        number_len: number.len(),
                    }));
                    number.clear();
                }
                Err(_) => (),
            };
            number.clear();
        }
    };

    for (idx, char) in line.chars().enumerate() {
        match char {
            char if char.is_digit(10) => number += &char.to_string(),
            '.' => {
                clear_number(&idx, &mut number, &mut tokens);
            }
            _ => {
                clear_number(&idx, &mut number, &mut tokens);
                tokens.push(Token::Symbol(idx))
            }
        }
    }
    clear_number(&line.len(), &mut number, &mut tokens);

    tokens
}

pub fn day03_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d03_t01"),
        //Ok(filepath) => filepath.join("input_d03_test"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut prev_symbols = Vec::new();
    let mut prev_numbers = Vec::new();
    let mut digits_sum: u64 = 0;
    for (line_idx, engine_schematic_line) in file_content.split("\n").enumerate() {
        let current_line_tokens = tokenizer(engine_schematic_line);
        let symbols: Vec<_> = current_line_tokens
            .iter()
            .filter_map(|i| match *i {
                Token::Symbol(symbol_idx) => Some(symbol_idx),
                _ => None,
            })
            .collect();
        let mut numbers: Vec<_> = Vec::new();
        let mut used_numbers: HashSet<_> = HashSet::new();
        for token in &current_line_tokens {
            match token {
                Token::Number(number) => {
                    println!(
                        "Number found {} {} {}",
                        number.value, number.start_index, number.number_len
                    );
                    let mut number_used = false;
                    for symbol in &symbols {
                        if *symbol == number.start_index - 1 {
                            println!("Symbol before!");
                            number_used = true;
                            break;
                        } else if *symbol == number.start_index + number.number_len {
                            println!("Symbol after!");
                            number_used = true;
                            break;
                        }
                    }
                    if !number_used {
                        for symbol in &prev_symbols {
                            let min_index = match number.start_index {
                                0 => 0,
                                _ => number.start_index - 1,
                            };
                            if (min_index <= *symbol)
                                && (*symbol <= number.start_index + number.number_len)
                            {
                                println!("Symbol above!");
                                number_used = true;
                                break;
                            }
                        }
                    }

                    if number_used {
                        println!("Number added to sum: {}", number.value);
                        digits_sum += number.value;
                    } else {
                        numbers.push(Token::Number(number.clone()));
                    }
                }
                Token::Symbol(symbol_idx) => {
                    println!("Symbol found {}", symbol_idx);
                    for number in &prev_numbers {
                        match number {
                            Token::Number(number) => {
                                if used_numbers.contains(number) {
                                    println!(
                                        "Number {} {} {} already added",
                                        number.value, number.start_index, number.number_len
                                    );
                                    continue;
                                }
                                println!(
                                    "Processing number: {} {} {}",
                                    number.value, number.start_index, number.number_len
                                );
                                let min_index = match number.start_index {
                                    0 => 0,
                                    _ => number.start_index - 1,
                                };
                                if (min_index <= *symbol_idx)
                                    && (*symbol_idx <= number.start_index + number.number_len)
                                {
                                    println!("Number above!");
                                    println!("Number added to sum: {}", number.value);
                                    used_numbers.insert(number);
                                    digits_sum += number.value;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        println!("Line :{} sum until now {}", line_idx, digits_sum);
        prev_symbols = symbols;
        prev_numbers = numbers;
    }

    println!("Sum: {}", digits_sum);
}
