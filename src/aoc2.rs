use std::{collections::HashMap, fs, u64};

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

fn parse_draw_item(draw: &str) -> Option<(&str, u64)> {
    if let Some(stripped_draw) = draw.strip_prefix(" ") {
        let splitted_draw: Vec<_> = stripped_draw.split(" ").collect();
        if splitted_draw.len() == 2 {
            let (count, color) = (splitted_draw[0], splitted_draw[1]);
            return match count.parse::<u64>() {
                Ok(items_count) => Some((color, items_count)),
                Err(_) => None,
            };
        }
    }
    None
}

pub fn day02_task01() {
    println!("{} {}", "Game 132", parse_game_id("Game 132"));
    let cubes_rules = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut game_ids_sum = 0;

    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d02_t01"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    for game_line in file_content.split("\n") {
        let splitted_game: Vec<_> = game_line.split(":").collect();
        if splitted_game.len() == 2 {
            let mut game_accepted = true;

            let (game_id, game) = (splitted_game[0], splitted_game[1]);
            let game_id = parse_game_id(game_id);
            for draw in game.split(";") {
                let mut cubes_map: HashMap<&str, u64> = HashMap::new();
                for draw_item in draw.split(",") {
                    if let Some((item, count)) = parse_draw_item(draw_item) {
                        let curr_val = match cubes_map.get(item) {
                            Some(val) => val,
                            None => &0,
                        };
                        cubes_map.insert(item, curr_val + count);
                    }
                }
                for (item, count) in &cubes_map {
                    if count > &cubes_rules[item] {
                        game_accepted = false;
                        println!("Game {} not accepted, because {}={}", game_id, item, count);
                        break;
                    }
                }
                if !game_accepted {
                    break;
                }
            }
            if !game_accepted {
                continue;
            }

            println!("Game: {} '{}'", game_line, game_accepted);
            if game_accepted {
                println!("Accepted: {}", game_id);
                game_ids_sum += game_id;
            }
        }
    }
    println!("GameIds sum: {}", game_ids_sum);
}
