use super::utils::file_utils::load_file;

pub fn main() {
    let file_name = "day4_test.txt".to_string();
    let data: String = load_file(file_name).unwrap();
    let map = create_map(data);
    let mut count = 0;
    &map.clone().into_iter().enumerate()
        .for_each(|(i, v)| {
             v.into_iter().enumerate().for_each(|(j, _)| 
            count += find_xmas(j, i, &map)
        )});
    println!("{count}");
}

fn create_map(data: String) -> Vec<Vec<String>> {
    let mut map: Vec<Vec<String>> = Vec::new();
    for line in data.lines() {
        let row: Vec<String> = line.chars().map(|s| s.to_string()).collect();
        map.push(row);
    }
    return map;
}

fn find_xmas(x: isize, y: isize, map: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    if map[y][x] == "X" {
        if find_right(x, y, &map) { count += 1; }
        if find_left(x, y, &map) { count += 1; }
        if find_up(x, y, &map) { count += 1; }
        if find_down(x, y, &map) { count += 1; }
    }
    return count;
}

fn find_right(x: isize, y: isize, map: &Vec<Vec<String>>) -> bool {
    println!("f_right: {x}, {y}");
    if x + 3 > map[y].len() - 1 { return false; }
    if map[y][x + 3] == "S" && map[y][x + 2] == "A" && map[y][x + 1] == "M" {
        return true
    }
    return false;
}

fn find_left(x: isize, y: isize, map: &Vec<Vec<String>>) -> bool {
    println!("f_left: {x}, {y}");
    if (x - 3) < 0 { return false; }
    if map[y][x - 3] == "S" && map[y][x - 2] == "A" && map[y][x - 1] == "M" {
        return true
    }
    return false;
}
fn find_up(y: isize, x: isize, map: &Vec<Vec<String>>) -> bool {
    println!("f_up: {x}, {y}");
    if (y - 3) < 0 { return false;}
    if map[y - 3][x] == "S" && map[y - 2][x] == "A" && map[y - 1][x] == "M" {
        return true
    }
    return false;
}

fn find_down(y: isize, x: isize, map: &Vec<Vec<String>>) -> bool {
    println!("f_down: {x}, {y}");
    if y + 3 > isize::from(map.len()) - 1 { return false; }
    if map[y + 3][x] == "S" && map[y + 2][x] == "A" && map[y + 1][x] == "M" {
        return true
    }
    return false;
}
