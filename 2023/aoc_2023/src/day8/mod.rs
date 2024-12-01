use std::collections::HashMap;

use super::utils::file_utils::load_file;
pub fn main() {
    let file_name = "day8.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines().enumerate();

    let mut command: String = "".to_string();

    let mut positions: HashMap<String, (String, String)> = HashMap::new();
    for (idx, line) in lines {
        if idx == 0 {
            line.chars().for_each(|c| {
                command.push(c);
            });
            continue;
        }

        if line.trim().len() > 0 {
            let mut split = line.split(" = ");
            let pos = split.clone().next().unwrap().trim();

            let cleaned_input = split
                .nth(1)
                .unwrap()
                .trim_start_matches('(')
                .trim_end_matches(')');
            let parts: Vec<&str> = cleaned_input.split(',').map(|part| part.trim()).collect();
            positions.insert(
                pos.to_string(),
                (parts[0].to_string(), parts[1].to_string()),
            );
        }
    }

    let mut cursor: String = "AAA".to_string();
    let mut steps: u64 = 0;
    let mut index: usize = 0;

    while index < command.len() {
        let c = command.chars().nth(index).unwrap();
        let result = match c {
            'L' => &positions.get(&cursor).unwrap().0,
            'R' => &positions.get(&cursor).unwrap().1,
            _ => panic!("Unknown command..."),
        };
        cursor = result.to_string();
        steps = steps + 1;
        if cursor == "ZZZ" {
            println!("Yaaaaaaay: {}", steps);
            break;
        } else {
            // println!("Step log {}, cursor: {}", steps, cursor);
            // println!("results: {}", result);
            if steps % 100 == 0 {}
        }

        if steps > u64::MAX {
            panic!("Oh noooo, too many steps");
        }
    }

    // println!("Command {}", command);
    // println!("Positions {:?}", positions);
}
