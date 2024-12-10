use super::utils::file_utils::load_file;

pub fn main() {
    let file_name = "day9_test.txt".to_string();
    let data: String = load_file(file_name).unwrap();

    let mapped = create_mapping(&data);
    println!("Mapped: {}", mapped);
}

fn create_mapping(data: &String) -> String {
    let mut prev: char = data.chars().nth(0).unwrap();
    let mut free_space: bool = false;
    let mut index: i32 = 0;
    return data
        .clone()
        .chars()
        .map(|c| {
            // println!("{}", c);
            if prev != c {
                free_space = !free_space;
                index += 1;
            }
            prev = c;

            if free_space {
                return '.'.to_string();
            }
            // println!("{}", index);
            return index.to_string();
        })
        .collect();
}
