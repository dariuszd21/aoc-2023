use std::{fs, u64};

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
            println!("{}", count);
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
    if let Some((color, count)) = parse_draw_item(" 3 blue") {
        println!("{} {}", count, color);
    }
    else {
        println!("NOT PARSED!");
    }
}
