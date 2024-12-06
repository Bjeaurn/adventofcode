use std::{collections::HashMap, hash::Hash};

use super::utils::file_utils::load_file;

#[derive(Debug, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<String>>,
    start: (usize, usize),
    orientation: Orientation,
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

pub fn main() {
    let file_name = "day6.txt".to_string();
    let data: String = load_file(file_name).unwrap();

    let mut map = create_map(data);
    let visited: HashMap<String, bool> = follow_lines(&mut map);
    println!("Part 1: {}", visited.len());
}

fn create_map(data: String) -> Map {
    let mut map: Vec<Vec<String>> = Vec::new();
    let mut start: (usize, usize) = (0, 0); // Initialize start position
    let mut orientation = Orientation::Up; // Default orientation
    for (idx, line) in data.lines().enumerate() {
        let row: Vec<String> = line.chars().map(|s| s.to_string()).collect();
        map.push(row.clone());
        let found_start = row.iter().position(|r| r.find("^").is_some());

        match found_start {
            Some(pos) => start = (idx, pos),
            None => {}
        }
    }
    return Map {
        map,
        start,
        orientation,
    };
}

fn follow_lines(map: &mut Map) -> HashMap<String, bool> {
    let mut colored_map = map.map.clone();
    let mut current_pos = map.start;
    let bounds: (usize, usize) = (colored_map.len(), colored_map[0].len());
    let mut out_of_bounds = false;
    let mut visited: HashMap<String, bool> = HashMap::new();
    // let mut steps = 0;
    while out_of_bounds == false {
        println!("{:?}", &map.orientation);
        let mut next_pos = get_next_position(&colored_map, &current_pos, &map.orientation);
        if is_out_of_bounds(&next_pos, &bounds) {
            println!("Out of bounds");
            out_of_bounds = true;
            break;
        }
        println!("{}, {}", next_pos.0, next_pos.1);
        next_pos = match map.map[next_pos.0][next_pos.1].as_str() {
            "#" => {
                map.orientation = rotate_90(&map.orientation);
                let value = get_next_position(&colored_map, &current_pos, &map.orientation);
                value
            }
            _ => next_pos,
        };
        current_pos = next_pos;
        visited.insert(tuple_to_string(&current_pos), true);
        // steps += 1;
    }
    return visited;
}

fn tuple_to_string(tup: &(usize, usize)) -> String {
    return format!("{}, {}", tup.0, tup.1);
}
fn rotate_90(orientation: &Orientation) -> Orientation {
    return match orientation {
        Orientation::Up => Orientation::Right,
        Orientation::Right => Orientation::Down,
        Orientation::Down => Orientation::Left,
        Orientation::Left => Orientation::Up,
    };
}

fn is_out_of_bounds(pos: &(usize, usize), bounds: &(usize, usize)) -> bool {
    pos.0 < 0 || pos.0 >= bounds.0 || pos.1 < 0 || pos.1 >= bounds.1
}

fn get_next_position(
    colored_map: &Vec<Vec<String>>,
    current_pos: &(usize, usize),
    orientation: &Orientation,
) -> (usize, usize) {
    let mut next_pos = *current_pos;
    // if next_pos.0.checked_sub(1) < Some(0) || next_pos.1.checked_sub(1) < Some(0) {
    //     println!("Out of bounds");
    //     return next_pos;
    // }
    // Move in the direction of the orientation (up, down, left, right
    match orientation {
        Orientation::Up => next_pos.0 -= 1,
        Orientation::Down => next_pos.0 += 1,
        Orientation::Left => next_pos.1 -= 1,
        Orientation::Right => next_pos.1 += 1,
    }
    return next_pos;
}
