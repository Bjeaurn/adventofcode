use std::collections::HashMap;

use super::utils::file_utils::load_file;
pub fn main() {
    let file_name = "day6.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines().into_iter().enumerate();

    // let mut times: Vec<u32> = vec![];
    let mut times: Vec<u64> = vec![];
    // let mut distances: Vec<u32> = vec![];
    let mut distances: Vec<u64> = vec![];

    for (idx, line) in lines {
        let split: Vec<&str> = line.split(":").collect();
        // println!("idx: {}, split, {:?}", idx, split);
        match idx {
            0 => {
                let str: String = split[1]
                    .trim()
                    .split_whitespace()
                    // .map(|v| v.parse::<u32>().unwrap())
                    .map(|n| n.to_string())
                    .collect();

                times = vec![str.parse::<u64>().unwrap()];
            }
            1 => {
                let str: String = split[1]
                    .trim()
                    .split_whitespace()
                    // .map(|v| v.parse::<u32>().unwrap())
                    .map(|n| n.to_string())
                    .collect();

                println!("{}", str);

                distances = vec![str.parse::<u64>().unwrap()];
            }
            _ => (),
        }
    }

    println!("{:?} {:?}", times, distances);

    let mut wins: Vec<u32> = vec![];

    times.iter().zip(&distances).enumerate().for_each(|v| {
        let idx = v.0;
        let (max_time, distance_to_beat) = v.1;

        for t in 0..max_time.clone() {
            let speed = t;
            let race = (max_time - t) * speed;
            if &race > distance_to_beat {
                if idx >= wins.len() {
                    wins.push(0);
                }
                wins[idx] += 1;
            }
        }
    });

    let result = wins.iter().copied().reduce(|a, b| a * b).unwrap();

    println!("{:?}", result);
    // println!("{:?}", distances);
}
