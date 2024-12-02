use super::utils::file_utils::load_file;

fn is_safe(numbers: &[i64]) -> bool {
    let mut diffs = Vec::new();
    for i in 1..numbers.len() {
        diffs.push(numbers[i] - numbers[i - 1]);
    }
    diffs.iter().all(|d| *d <= -1 && *d >= -3) || diffs.iter().all(|d| *d >= 1 && *d <= 3)
}

pub fn main() {
    for part1 in [false, false] {
        let file_name = "day2.txt".to_string();
        let data = load_file(file_name).unwrap();
        let lines = data.lines().collect::<Vec<_>>();

        let mut total = 0;
        for line in lines {
            let numbers = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let mut safe = is_safe(&numbers);
            if !part1 && !safe {
                for i in 0..numbers.len() {
                    let mut c = numbers.clone();
                    c.remove(i);
                    if is_safe(&c) {
                        safe = true;
                        break;
                    }
                }
            }

            if safe {
                total += 1;
            }
        }

        println!("{}", total);
    }
}