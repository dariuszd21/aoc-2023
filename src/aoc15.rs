use std::collections::HashMap;
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
    let mut res: u64 = 0;
    let sequence = load_sequence(file_path);

    let mut hash_map: HashMap<u8, Vec<(String, u8)>> = HashMap::new();

    for i in 0..=255 {
        hash_map.insert(i, Vec::new());
    }

    println!("\n\n----------\n\n");

    for elem in sequence {
        if elem.ends_with("-") {
            if let Some(stripped_elem) = elem.strip_suffix("-") {
                let hash = hashing_algorithm(stripped_elem);
                let mut idx: Option<usize> = None;
                match &mut hash_map.get_mut(&hash) {
                    Some(vec) => {
                        for (i, (name, _)) in vec.iter().enumerate() {
                            if name == stripped_elem {
                                idx = Some(i);
                                break;
                            }
                        }
                        match idx {
                            Some(el_index) => {
                                let (name, lens) = vec.remove(el_index);
                                println!("Removing: {} {}", name, lens);
                            }
                            None => (),
                        };
                    }
                    None => (),
                }
            }
            continue;
        }
        let splitted_vec: Vec<_> = elem.split("=").collect();
        if splitted_vec.len() == 2 {
            let (name, lens_len) = (splitted_vec[0], splitted_vec[1].parse::<u8>().unwrap());
            println!("{} {}", name, lens_len);
            let hash = hashing_algorithm(name);
            match &mut hash_map.get_mut(&hash) {
                Some(vec) => {
                    let mut idx: Option<usize> = None;
                    for (i, (item_name, _)) in vec.iter().enumerate() {
                        if name == item_name {
                            idx = Some(i);
                            break;
                        }
                    }
                    match idx {
                        Some(el_index) => {
                            let (name, lens) = vec.remove(el_index);
                            println!("Removing: {} {}", name, lens);
                            println!("Inserting: {} {}", name, lens_len);
                            vec.insert(el_index, (name.to_string(), lens_len));
                        }
                        None => {
                            println!("Inserting: {} {}", name, lens_len);
                            vec.push((name.to_string(), lens_len));
                        }
                    };
                }
                None => (),
            }
        }
    }

    for (box_nr, box_content) in hash_map {
        println!("Analysing box: {}", box_nr as u64 + 1);
        for (i, (_, lens_focal)) in box_content.iter().enumerate() {
            res += (box_nr as u64 + 1) * (i as u64 + 1) * (*lens_focal as u64);
        }
        println!("Current res: {}", res);
    }

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
        assert_eq!(solve_part_01(&input_filepath), 1320);
    }

    #[test]
    fn test_solving_d14_part2() {
        let input_filepath = match std::env::current_dir() {
            Ok(filepath) => filepath.join("input_d15_test"),
            Err(_) => panic!("Cannot find current directory"),
        };
        assert_eq!(solve_part_02(&input_filepath), 145);
    }
}
