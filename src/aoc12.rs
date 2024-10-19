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

pub fn day12_task01() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_row() {
        let (parts_vec, damaged_count) = parse_row("#.#.### 1,1,3");
        let result = count_damaged(&parts_vec);
        assert_eq!(result, damaged_count);
    }
}
