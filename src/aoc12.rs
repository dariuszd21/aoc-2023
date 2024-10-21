use std::fs;
use std::path::Path;

#[derive(Copy, Clone, PartialEq)]
enum PartState {
    WORKING,
    DAMAGED,
    UNKNOWN,
}

fn count_damaged(parts_state: &Vec<PartState>) -> Vec<u32> {
    let mut broken_count = Vec::new();
    let mut current_count = 0;
    for part in parts_state {
        match part {
            PartState::WORKING | PartState::UNKNOWN => {
                if current_count > 0 {
                    broken_count.push(current_count);
                    current_count = 0;
                }
            }
            PartState::DAMAGED => current_count += 1,
        }
    }
    if current_count > 0 {
        broken_count.push(current_count);
    }

    broken_count
}

fn expected_damaged_and_working(damaged: &Vec<u32>) -> (usize, usize) {
    let expected_damaged: u32 = damaged.iter().sum();
    let expected_working = damaged.len() - 1;

    (expected_damaged.try_into().unwrap(), expected_working)
}

fn count_in_row(parts_state: &Vec<PartState>, state: &PartState) -> usize {
    parts_state.iter().filter(|&p| p == state).count()
}

fn count_damaged_in_blueprint(parts_state: &Vec<PartState>) -> usize {
    count_in_row(parts_state, &PartState::DAMAGED)
}

fn count_unknown_in_blueprint(parts_state: &Vec<PartState>) -> usize {
    count_in_row(parts_state, &PartState::UNKNOWN)
}

fn count_working_in_blueprint(parts_state: &Vec<PartState>) -> usize {
    let mut first = parts_state.len() - 1;
    let mut last = 0;

    for i in 0..parts_state.len() {
        match parts_state[i] {
            PartState::WORKING => {}
            PartState::DAMAGED | PartState::UNKNOWN => {
                if i < first {
                    first = i;
                }
                if i > last {
                    last = i;
                }
            }
        }
    }

    count_in_row(&parts_state[first..last].into(), &PartState::WORKING)
}

fn parse_row(line: &str) -> (Vec<PartState>, Vec<u32>) {
    let mut part_state = Vec::new();
    let mut damaged_parts_count = Vec::new();
    let splitted_val: Vec<_> = line.split(" ").collect();
    if splitted_val.len() == 2 {
        let (parts, values) = (splitted_val[0], splitted_val[1]);
        for char in parts.chars() {
            part_state.push(match char {
                '#' => PartState::DAMAGED,
                '.' => PartState::WORKING,
                '?' | _ => PartState::UNKNOWN,
            })
        }
        for val in values.split(',') {
            if let Ok(num_val) = val.parse::<u32>() {
                damaged_parts_count.push(num_val);
            }
        }
    }

    (part_state, damaged_parts_count)
}

fn permutate(parts_state: &Vec<PartState>) -> Vec<Vec<PartState>> {
    let mut permutations = Vec::new();
    for part_state in parts_state {
        match part_state {
            PartState::WORKING | PartState::DAMAGED => {
                if permutations.len() == 0 {
                    permutations.push(vec![part_state.clone()]);
                } else {
                    for permutation in &mut permutations {
                        permutation.push(part_state.clone());
                    }
                }
            }
            PartState::UNKNOWN => {
                if permutations.len() == 0 {
                    let first_perm = vec![PartState::WORKING];
                    permutations.push(first_perm);
                    let second_per = vec![PartState::DAMAGED];
                    permutations.push(second_per);
                } else {
                    // Copy item with both ? and #
                    let mut additional_per = Vec::new();
                    for permutation in &mut permutations {
                        let mut clonned_per = permutation.clone();
                        clonned_per.push(PartState::DAMAGED);
                        additional_per.push(clonned_per);
                        permutation.push(PartState::WORKING);
                    }
                    permutations.extend(additional_per);
                }
            }
        }
    }

    permutations
}

fn solve(file_path: &Path) -> u64 {
    let mut res = 1;

    let file_content = fs::read_to_string(file_path).expect("File could not be loaded");
    for line in file_content.split("\n") {
        let (parts_state, damaged_amount) = parse_row(line);
        let (expected_damaged, expected_working) = expected_damaged_and_working(&damaged_amount);
        let number_of_damaged = count_damaged_in_blueprint(&parts_state);
        let number_of_unknown = count_unknown_in_blueprint(&parts_state);
        let number_of_working = count_working_in_blueprint(&parts_state);
        let number_of_missing_working = if expected_working > number_of_working {
            expected_working - number_of_working
        } else {
            0
        };
        let number_of_missing_damaged = expected_damaged - number_of_damaged;
        println!(
            "Number of per for {} : already damaged ({}), available slots ({})",
            line, number_of_damaged, number_of_unknown,
        );
        println!(
            "Expected damaged: {}, expected working: {}. Damaged to draw: {}, missing working: {}",
            expected_damaged,
            expected_working,
            number_of_missing_damaged,
            number_of_missing_working,
        );
        let mut local_res = 1;
        for _ in 1..(number_of_unknown - number_of_missing_damaged - number_of_missing_working) {
            local_res *= 2;
        }
        println!("Res: {}", local_res);
        println!(
            "Res2: {}",
            (number_of_missing_damaged + number_of_missing_working) * number_of_unknown
        );
        res += local_res;

        let mut old_res = 0;
        let permutations = permutate(&parts_state);
        let (mut ends_with_damaged, mut ends_with_working): (u64, u64) = (0, 0);
        for permutation in &permutations {
            if count_damaged(&permutation) == damaged_amount {
                if let Some(&part) = permutation.last() {
                    match part {
                        PartState::WORKING => ends_with_working += 1,
                        PartState::DAMAGED => ends_with_damaged += 1,
                        PartState::UNKNOWN => (),
                    }
                }
                old_res += 1;
            }
        }
        let mut number_of_working_prefixed: u64 = 0;
        if ends_with_working > 0 {
            let mut extended_part_state = parts_state.clone();
            extended_part_state.insert(0, PartState::UNKNOWN);
            for permutation in permutate(&extended_part_state) {
                if count_damaged(&permutation) == damaged_amount {
                    number_of_working_prefixed += 1;
                }
            }
        }
        println!("Old res: {}", old_res);
        println!(
            "Number of ending with '#': {}, with '.': {}",
            ends_with_damaged, ends_with_working
        );

        println!(
            "Results for x5: {}",
            ends_with_damaged.pow(5) + ends_with_working * number_of_working_prefixed.pow(4),
        );

        println!("Total res: {}", res);
    }

    res.try_into().unwrap()
}

pub fn day12_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d12_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };
    println!("Result is {}", solve(&input_filepath));
}

pub fn day12_task02() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d12_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };
    println!("Result is {}", solve(&input_filepath));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_row() {
        let (parts_vec, damaged_count) = parse_row("#.#.### 1,1,3");
        let result = count_damaged(&parts_vec);
        assert_eq!(result, damaged_count);
    }

    #[test]
    fn test_answering_test_puzzle() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d12_t01_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        let result = solve(&input_filepath);
        assert_eq!(result, 21);
    }
}
