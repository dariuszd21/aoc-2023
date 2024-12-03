mod aoc15;

fn main() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d15_01"),
        Err(_) => panic!("Cannot find current directory"),
    };
    let result = aoc15::solve_part_01(&input_filepath);
    println!("And the result is {}", result);

    let result = aoc15::solve_part_02(&input_filepath);
    println!("And the result for part 2 is {}", result);
}
