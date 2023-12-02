mod file;

use file::file_utils::load_file;
fn main() {
    let file_name = "day1.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines();
}
