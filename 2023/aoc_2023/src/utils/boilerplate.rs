mod file;

use file::file_utils::load_file;
fn main() {
    let file_name = "day1.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines();

    for line in lines {}
}

pub type DataID = u64;
pub struct Data {
    id: DataID,
}

impl Data {
    fn do_things(&self) -> &DataID {
        &self.id
    }
}
