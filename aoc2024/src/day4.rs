use super::utils::file_utils::load_file;

pub fn main() {
    let file_name = "day4_test.txt".to_string();
    let data: String = load_file(file_name).unwrap();

    println!("{data}");
}
