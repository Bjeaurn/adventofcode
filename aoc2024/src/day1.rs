use std::collections::HashMap;

use super::utils::file_utils::load_file;
pub fn main() {
    let file_name = "day1.txt".to_string();
    let data = load_file(file_name).unwrap();

    let array = data.split_whitespace().collect::<Vec<&str>>();
    let mut left: Vec<i32> = array
        .clone()
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, e)| e.parse::<i32>().unwrap())
        .collect();
    let mut right: Vec<i32> = array
        .clone()
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0)
        .map(|(_, e)| e.parse::<i32>().unwrap())
        .collect();

    left.sort();
    right.sort();

    part1(&left, &right);
    part2(&left, &right);
}

fn part1(left: &Vec<i32>, right: &Vec<i32>) {
    let result: i32 = left
        .iter()
        .enumerate()
        .map(|(i, &e)| {
            if e > right[i] {
                return e - right[i];
            } else {
                return right[i] - e;
            }
        })
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{:?}", result);
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) {
    let indexed = right.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let result: i32 = left
        .iter()
        .enumerate()
        .map(|(_, &e)| e * indexed.get(&e).unwrap_or(&0))
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{:?}", result);
}
