use std::fs;

fn parse_game_id(game_name: &str) -> u64 {
    let splitted_name: Vec<_> = game_name.split(" ").collect();
    if splitted_name.len() == 2 {
        let game_id = splitted_name[1];
        match game_id.parse::<u64>() {
            Ok(id) => id,
            Err(_) => 0,
        }
    } else {
        0
    }
}

pub fn day02_task01() {
    println!("{} {}", "Game 132", parse_game_id("Game 132"));
}
