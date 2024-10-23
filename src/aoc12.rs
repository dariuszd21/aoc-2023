use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone, PartialEq)]
enum PartState {
    WORKING,
    DAMAGED,
    UNKNOWN,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum TypeOfSolution {
    Classic,
    PrefixedWithUnknown,
    PostfixedWithUnknown,
    PreAndPostfixedWithUknown,
}

#[derive(Debug)]
struct Solution {
    starts_with_working: u64,
    starts_with_damaged: u64,
    ends_with_working: u64,
    ends_with_damaged: u64,
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

fn calculate_solution(parts_state: &Vec<PartState>, damaged_amount: &Vec<u32>) -> Solution {
    let mut classic_solution = Solution {
        starts_with_working: 0,
        starts_with_damaged: 0,
        ends_with_working: 0,
        ends_with_damaged: 0,
    };
    for permutation in permutate(parts_state) {
        if count_damaged(&permutation) == *damaged_amount {
            if let Some(&part) = permutation.last() {
                match part {
                    PartState::WORKING => {
                        classic_solution.ends_with_working += 1;

                        if let Some(&part) = permutation.first() {
                            match part {
                                PartState::WORKING => classic_solution.starts_with_working += 1,
                                PartState::DAMAGED => classic_solution.starts_with_damaged += 1,
                                PartState::UNKNOWN => (),
                            }
                        }
                    }
                    PartState::DAMAGED => {
                        classic_solution.ends_with_damaged += 1;

                        if let Some(&part) = permutation.first() {
                            match part {
                                PartState::WORKING => classic_solution.starts_with_working += 1,
                                PartState::DAMAGED => classic_solution.starts_with_damaged += 1,
                                PartState::UNKNOWN => (),
                            }
                        }
                    }
                    PartState::UNKNOWN => (),
                }
            }
        }
    }
    classic_solution
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
        let mut result: HashMap<TypeOfSolution, Solution> = HashMap::new();
        // Calculate how many results we got from current (non-extended) flow
        let (mut ends_with_damaged, mut ends_with_working): (u64, u64) = (0, 0);
        for permutation in &permutations {
            if count_damaged(&permutation) == damaged_amount {
                if let Some(&part) = permutation.last() {
                    match part {
                        PartState::WORKING => {
                            ends_with_working += 1;
                        }
                        PartState::DAMAGED => {
                            ends_with_damaged += 1;
                        }
                        PartState::UNKNOWN => (),
                    }
                }
                old_res += 1;
            }
        }
        result.insert(
            TypeOfSolution::Classic,
            calculate_solution(&parts_state, &damaged_amount),
        );
        println!(
            "Classic solution: {:?}",
            result.get(&TypeOfSolution::Classic).unwrap()
        );
        {
            let mut extended_part_state = parts_state.clone();
            extended_part_state.insert(0, PartState::UNKNOWN);
            result.insert(
                TypeOfSolution::PrefixedWithUnknown,
                calculate_solution(&extended_part_state, &damaged_amount),
            );
            println!(
                "Prefixed solution: {:?}",
                result.get(&TypeOfSolution::PrefixedWithUnknown).unwrap()
            );
        }
        {
            let mut extended_part_state = parts_state.clone();
            extended_part_state.push(PartState::UNKNOWN);
            result.insert(
                TypeOfSolution::PostfixedWithUnknown,
                calculate_solution(&extended_part_state, &damaged_amount),
            );
            println!(
                "Postfixed solution: {:?}",
                result.get(&TypeOfSolution::PostfixedWithUnknown).unwrap()
            );
        }
        {
            let mut extended_part_state = parts_state.clone();
            extended_part_state.insert(0, PartState::UNKNOWN);
            extended_part_state.push(PartState::UNKNOWN);
            result.insert(
                TypeOfSolution::PreAndPostfixedWithUknown,
                calculate_solution(&extended_part_state, &damaged_amount),
            );
            println!(
                "Pre and postfixed solution: {:?}",
                result
                    .get(&TypeOfSolution::PreAndPostfixedWithUknown)
                    .unwrap()
            );
        }

        let number_of_el = 5;
        let mut solutions: Vec<Vec<TypeOfSolution>> = Vec::new();
        for i in 0..number_of_el {
            if i == 0 {
                solutions.push(vec![TypeOfSolution::Classic]);
                solutions.push(vec![TypeOfSolution::PostfixedWithUnknown]);
                continue;
            }
            if i == number_of_el - 1 {
                for sol in &mut solutions {
                    match sol.last() {
                        Some(sol_type) => match sol_type {
                            TypeOfSolution::Classic => {
                                sol.push(TypeOfSolution::PrefixedWithUnknown);
                            }
                            TypeOfSolution::PrefixedWithUnknown => {
                                sol.push(TypeOfSolution::PrefixedWithUnknown);
                            }
                            TypeOfSolution::PostfixedWithUnknown => {
                                sol.push(TypeOfSolution::Classic);
                            }
                            TypeOfSolution::PreAndPostfixedWithUknown => {
                                sol.push(TypeOfSolution::Classic);
                            }
                        },
                        None => (),
                    }
                }
                continue;
            }

            let mut new_solutions = Vec::new();
            for sol in &solutions {
                match sol.last() {
                    Some(sol_type) => match sol_type {
                        TypeOfSolution::Classic => {
                            let mut new_sol_with_prefix = sol.clone();
                            let mut new_sol_with_both_fixes = sol.clone();
                            new_sol_with_prefix.push(TypeOfSolution::PrefixedWithUnknown);
                            new_sol_with_both_fixes.push(TypeOfSolution::PreAndPostfixedWithUknown);
                            new_solutions.push(new_sol_with_prefix);
                            new_solutions.push(new_sol_with_both_fixes);
                        }
                        TypeOfSolution::PrefixedWithUnknown => {
                            let mut new_sol_with_prefix = sol.clone();
                            let mut new_sol_with_both_fixes = sol.clone();
                            new_sol_with_prefix.push(TypeOfSolution::PrefixedWithUnknown);
                            new_sol_with_both_fixes.push(TypeOfSolution::PreAndPostfixedWithUknown);
                            new_solutions.push(new_sol_with_prefix);
                            new_solutions.push(new_sol_with_both_fixes);
                        }
                        TypeOfSolution::PostfixedWithUnknown => {
                            let mut new_sol_with_postfix = sol.clone();
                            let mut new_sol_with_classic = sol.clone();
                            new_sol_with_postfix.push(TypeOfSolution::PostfixedWithUnknown);
                            new_sol_with_classic.push(TypeOfSolution::Classic);
                            new_solutions.push(new_sol_with_postfix);
                            new_solutions.push(new_sol_with_classic);
                        }
                        TypeOfSolution::PreAndPostfixedWithUknown => {
                            let mut new_sol_with_postfix = sol.clone();
                            let mut new_sol_with_classic = sol.clone();
                            new_sol_with_postfix.push(TypeOfSolution::PostfixedWithUnknown);
                            new_sol_with_classic.push(TypeOfSolution::Classic);
                            new_solutions.push(new_sol_with_postfix);
                            new_solutions.push(new_sol_with_classic);
                        }
                    },
                    None => (),
                }
            }
            solutions = new_solutions;
        }

        for sol in solutions {
            println!("Available solution: {:?}", sol);
        }

        println!("Old res: {}", old_res);
        println!(
            "Number of ending with '#': {}, with '.': {}",
            ends_with_damaged, ends_with_working
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
