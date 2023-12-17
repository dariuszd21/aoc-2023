use core::fmt;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

impl fmt::Display for HandType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


fn parse_card_label(label: char) -> u8 {
    match label {
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 1,
    }
}

fn parse_card(line: &str) -> Option<(HandType, (u8, u8, u8, u8, u8), u64)> {
    let mut values: HashMap<char, u8> = HashMap::new();
    let splitted_line: Vec<_> = line.split(" ").collect();
    if splitted_line.len() == 2 {
        let (hand, rank) = (splitted_line[0], splitted_line[1]);
        let rank_val = match rank.parse::<u64>() {
            Ok(val) => val,
            Err(_) => 0,
        };

        for c in hand.chars() {
            let current_val = match values.get(&c) {
                Some(v) => *v,
                None => 0,
            };
            values.insert(c, current_val + 1);
        }

        let is_four_in_hand = values.values().filter(|v| **v == 4).count() == 1;
        let is_three_in_hand = values.values().filter(|v| **v == 3).count() == 1;
        let number_of_twos = values.values().filter(|v| **v == 2).count();
        let hand_type: HandType;
        if values.len() == 1 {
            hand_type = HandType::FiveOfKind;
        } else if values.len() == 5 {
            hand_type = HandType::HighCard;
        } else if is_four_in_hand {
            hand_type = HandType::FourOfKind;
        } else if is_three_in_hand {
            hand_type = match number_of_twos {
                0 => HandType::ThreeOfKind,
                _ => HandType::FullHouse,
            };
        } else {
            hand_type = match number_of_twos {
                2 => HandType::TwoPairs,
                _ => HandType::OnePair,
            };
        }

        let parsed_values: Vec<_> = hand.chars().map(|v| parse_card_label(v)).collect();

        return Some((
            hand_type,
            (
                parsed_values[0],
                parsed_values[1],
                parsed_values[2],
                parsed_values[3],
                parsed_values[4],
            ),
            rank_val,
        ));
    }
    None
}

pub fn day07_task01() {
    let input_filepath = match std::env::current_dir() {
        Ok(filepath) => filepath.join("input_d07_t01"),
        //Ok(filepath) => filepath.join("input_d07_test"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut all_cards: Vec<_> = Vec::new();

    for line in file_content.split("\n") {
        println!("{}", line);
        if let Some((hand_type, values, rank_val)) = parse_card(line) {
            println!("HandType: {}, rank: {}, values: {}", hand_type, rank_val, values.0);
            all_cards.push((hand_type, values, rank_val));
        }
    }

    all_cards.sort();
    let mut total_winnings = 0;
    for (i, (hand_type, values, rank_val)) in all_cards.iter().enumerate() {
        let val: u64 = i as u64 + 1;
        println!("{} {} {}", val, hand_type, values.0);
        total_winnings += val*rank_val;
    }
    println!("Total winnings {}", total_winnings);

}
