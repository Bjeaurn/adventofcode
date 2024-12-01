use std::collections::HashMap;

use super::utils::file_utils::load_file;
use memoize::memoize;
pub fn main() {
    let file_name = "day5.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines().into_iter().enumerate();

    let mut seeds: Vec<u64> = vec![];
    let mut seeds_b: Vec<u64> = vec![];
    let mut cache: HashMap<u64, u64> = HashMap::new();
    let mut maps: Vec<Map> = vec![];
    let mut latest: Map = Map {
        name: "".to_string(),
        ranges: vec![],
    };

    for (idx, line) in lines {
        if line.is_empty() {
            continue;
        }
        let split: Vec<&str> = line.split(":").collect();
        if idx == 0 {
            seeds = split[1]
                .trim()
                .split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            println!("{:?}", seeds);

            seeds_b = seeds
                .chunks_exact(2)
                .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
                .collect();
            continue;
        }
        if split[0].ends_with("map") {
            if latest.name.len() > 0 {
                println!("{:?}", latest);
                maps.push(latest);
            }
            let name = split[0];
            latest = Map {
                name: name.to_owned(),
                ranges: vec![],
            };
            println!("{}", name);
            continue;
        }
        if split[0].len() > 0 {
            println!("{}", split[0]);
            let numbers: Vec<u64> = split[0]
                .trim()
                .split_whitespace()
                .map(|str| str.parse::<u64>().expect("it dun goofed"))
                .collect::<Vec<u64>>();
            let range = Range {
                from: numbers[1],
                to: numbers[0],
                range: numbers[2],
            };
            latest.ranges.push(range);
        }
    }
    maps.push(latest);

    // let result = find_locations(&maps, &seeds);
    // println!("Final Result: {}", result);

    let result_b: u64 = find_locations(&maps, &seeds_b);
    println!("Final Result part 2: {}", result_b);
}

fn find_locations(maps: &Vec<Map>, seeds: &Vec<u64>) -> u64 {
    let mut smallest = u64::MAX;
    for seed in seeds {
        let result = find_location_for_seed(maps, seed);
        if result < smallest {
            println!("Result {} is smaller then smallest: {}", result, smallest);
            smallest = result;
        }
    }
    smallest
}
fn get_map_result(map: Map, input: u64) -> u64 {
    map.get(input)
}

fn find_location_for_seed(maps: &Vec<Map>, seed: &u64) -> u64 {
    // println!("Checking seed: {}", seed);
    let mut result = seed.clone();
    for map in maps {
        result = map.get(result);
        // println!("Map name: {}, result: {}", map.name, result);
    }
    result
}


impl Map {
    pub fn get(&self, input: u64) -> u64 {
        for range in &self.ranges {
            if input >= range.from && input < (range.from + range.range) {
                return input - range.from + range.to;
            }
        }
        return input;
    }
}

#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
    range: u64,
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}
