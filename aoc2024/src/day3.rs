use super::utils::file_utils::load_file;
use regex::Regex;

pub fn main() {
    let file_name = "day3.txt".to_string();
    let data: String = load_file(file_name).unwrap();

    let re_place = Regex::new(r"don\'t\(\)((.|\n))*?do\(\)").unwrap();
    let part2_data = re_place.replace_all(&data, "").to_string();
    let part1_data = &data;

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut results = Vec::new();
    // println!("{start_data}");
    for result in re.find_iter(&part2_data) {
        let left = result.as_str().split(",").nth(0).unwrap().split("(").nth(1).unwrap();
        let right = result.as_str().split(",").nth(1).unwrap().split(")").nth(0).unwrap();
        let sum: i32 = multiply(left, right);
        results.push(sum);
    }
    println!("{}", results.iter().sum::<i32>())
}

pub fn multiply(left: &str, right: &str) -> i32 {
    let result = left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
    result
}