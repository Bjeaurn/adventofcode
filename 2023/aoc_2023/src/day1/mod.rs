use super::utils::file_utils::load_file;
fn main() {
    let file_name = "day1.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines();

    let replacements = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
        ("zero", "z0o"),
    ];

    let mut results: Vec<String> = Vec::new();
    for line in lines {
        let mut numbers: Vec<u64> = Vec::new();

        let mut string = line.to_string();

        for (pattern, replacement) in replacements {
            let replaced = string.replace(&pattern, &replacement);
            string = replaced
        }
        // println!("{}", string);
        for chr in string.chars() {
            if chr.is_numeric() {
                numbers.push(chr.to_digit(10).unwrap())
            }
        }
        results.push(get(numbers));
    }
    let total: u64 = results.iter().map(|s| s.parse::<u64>().unwrap()).sum();
    println!("{:?}", total);
}

fn get(v: Vec<u64>) -> String {
    let f = v.first().unwrap();
    let l = v.last().unwrap();
    format!("{f}{l}")
}
