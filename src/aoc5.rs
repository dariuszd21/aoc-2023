use std::{fs, collections::HashMap, u64};

fn find_in_map(map: &HashMap<(u64, u64), u64>, val: u64) -> u64 {
    let found_val = val;
    for ((range_lower, range_upper), dst_start) in map {
       if *range_lower <= val && val < *range_upper {
            println!("Found value {} in ({}, {}]", val, *range_lower, *range_upper);
            let dst_val = dst_start + (val - *range_lower);
            println!("Assigning {}", dst_val);
            return dst_val;
        }
    }
    println!("Not found, returning as is {}", found_val);
    return found_val;
}

pub fn day05_task01() {
    let input_filepath = match std::env::current_dir() {
        //Ok(filepath) => filepath.join("input_d05_t01"),
        Ok(filepath) => filepath.join("input_d05_test"),
        Err(_) => panic!("Cannot find current directory"),
    };

    println!("Input filepath: {}", input_filepath.display());

    let file_content = fs::read_to_string(input_filepath).expect("File could not be loaded");

    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil: HashMap<(u64, u64), u64> = HashMap::new();
    let mut soil_to_fertilizer: HashMap<(u64, u64), u64> = HashMap::new();
    let mut fertilizer_to_water: HashMap<(u64, u64), u64> = HashMap::new();
    let mut water_to_light: HashMap<(u64, u64), u64> = HashMap::new();
    let mut light_to_temperature: HashMap<(u64, u64), u64> = HashMap::new();
    let mut temperature_to_humidity: HashMap<(u64, u64), u64> = HashMap::new();
    let mut humidity_to_location: HashMap<(u64, u64), u64> = HashMap::new();

    let mut current_map = &mut seed_to_soil;

    for line in file_content.split("\n") {
        if line.starts_with("seeds:") {
            let seeds_splitted: Vec<_> = line.split(":").collect();
            if seeds_splitted.len() == 2 {
                let seeds_string = seeds_splitted[1];
                for seed_nr_string in seeds_string.split(" ") {
                    match seed_nr_string.parse::<u64>() {
                        Ok(val) => seeds.push(val),
                        Err(_) => (),
                    }
                }
            }
            continue;
        }
        if line.starts_with("seed-to-soil") {
            current_map = &mut seed_to_soil;
            continue;
        }
        if line.starts_with("soil-to-fertilizer") {
            current_map = &mut soil_to_fertilizer;
            continue;
        }
        if line.starts_with("fertilizer-to-water") {
            current_map = &mut fertilizer_to_water;
            continue;
        }
        if line.starts_with("water-to-light") {
            current_map = &mut water_to_light;
            continue;
        }
        if line.starts_with("light-to-temperature") {
            current_map = &mut light_to_temperature;
            continue;
        }
        if line.starts_with("temperature-to-humidity") {
            current_map = &mut temperature_to_humidity;
            continue;
        }

        if line.starts_with("humidity-to-location") {
            current_map = &mut humidity_to_location;
            continue;
        }

        let unparsed_numbers: Vec<_> = line.split(" ").collect();

        if unparsed_numbers.len() == 3 {
            let (destination_range_start_str, source_range_start_str, range_len_str) = (unparsed_numbers[0], unparsed_numbers[1], unparsed_numbers[2]);
            
            match destination_range_start_str.parse::<u64>() {
                Ok(destinaton_range_start) => {
                    match source_range_start_str.parse::<u64>() {
                        Ok(source_range_start) => {
                             match range_len_str.parse::<u64>() {
                                Ok(range_len) => {
                                    println!("Putting ({}, {}) => {} match", source_range_start, source_range_start+range_len, destinaton_range_start);
                                    current_map.insert((source_range_start, source_range_start+range_len), destinaton_range_start);
                                },
                                Err(_) => println!("Cannot parse {}", range_len_str),
                                 
                             }
                        }
                        Err(_) => println!("Cannot parse {}", source_range_start_str),
                    }
                }
                Err(_) => println!("Cannot parse {}", destination_range_start_str),
            }

        }


    }

    let mut locations: Vec<_> = Vec::new();
    for s in seeds {
        let mut found = find_in_map(&seed_to_soil, s);
        found = find_in_map(&soil_to_fertilizer, found);
        found = find_in_map(&fertilizer_to_water, found);
        found = find_in_map(&water_to_light, found);
        found = find_in_map(&light_to_temperature, found);
        found = find_in_map(&temperature_to_humidity, found);
        found = find_in_map(&humidity_to_location, found);
        locations.push(found);
    }

    for l in locations {
        println!("Location {}", l);
    }

}
