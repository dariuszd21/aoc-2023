use std::fs;
use std::path::Path;

#[derive(Copy, Clone)]
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
    let mut res = 0;

    let file_content = fs::read_to_string(file_path).expect("File could not be loaded");
    for line in file_content.split("\n") {
        let (parts_state, damaged_amount) = parse_row(line);
        let permutations = permutate(&parts_state);
        // println!("Number of per for {}, {}", line, permutations.len());
        for permutation in permutations {
            if count_damaged(&permutation) == damaged_amount {
                res += 1;
            }
        }
    }

    res
}

pub fn day12_task01() {
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
