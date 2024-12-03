use std::fs;
use std::path::Path;

fn hashing_algorithm(s: &str) -> u8 {
    let mut hash: u32 = 0;

    for s in s.chars() {
        let cur: u32 = ((hash + (s as u32)) * 17) % 256;
        hash = cur.try_into().unwrap();
    }

    hash.try_into().unwrap()
}

fn load_sequence(file_path: &Path) -> Vec<String> {
    let mut sequence = Vec::new();
    let file_content = fs::read_to_string(file_path).expect("File could not be loaded");

    for seq in file_content.trim().split(",") {
        sequence.push(seq.to_string());
    }

    sequence
}

pub fn solve_part_01(file_path: &Path) -> u64 {
    let mut res = 0;
    let sequence = load_sequence(file_path);

    for item in sequence {
        let current_hash: u64 = hashing_algorithm(&item).into();
        res += current_hash;
    }

    res
}

pub fn solve_part_02(file_path: &Path) -> u64 {
    let mut res = 0;
    let sequence = load_sequence(file_path);

    println!("\n\n----------\n\n");

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing_algorithm() {
        assert_eq!(hashing_algorithm("HASH"), 52);
    }

    #[test]
    fn test_hashing_algorithm_init() {
        assert_eq!(hashing_algorithm("rn=1"), 30);
        assert_eq!(hashing_algorithm("cm-"), 253);
        assert_eq!(hashing_algorithm("ot=7"), 231);
    }

    #[test]
    fn test_solving_d14_part1() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d15_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        assert_eq!(solve_part_01(&input_filepath), 136);
    }

    #[test]
    fn test_solving_d14_part2() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d15_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        assert_eq!(solve_part_02(&input_filepath), 64);
    }
}
