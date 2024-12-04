use super::utils::file_utils::load_file;

pub fn main() {
    let file_name = "day4_test.txt".to_string();
    let data: String = load_file(file_name).unwrap();
    let map = create_map(data);
    let mut count = 0;
    &map.clone().into_iter().enumerate().for_each(|(y, v)| {
        v.into_iter().enumerate().for_each(|(x, _)| {
            // Part 1
            // let add = find_xmas(x, y, &map);
            // count += add;

            // Part 2
            let add = find_x_mas(x, y, &map);
            count += add;
        });
    });
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

fn find_x_mas(x: usize, y: usize, map: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    if (map[y][x] == "A") {
        println!("A found");
    }
    return count;
}

fn find_xmas(x: usize, y: usize, map: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    if map[y][x] == "X" {
        if find_right(y, x, &map) {
            count += 1;
        }
        if find_left(y, x, &map) {
            count += 1;
        }
        if find_up(y, x, &map) {
            count += 1;
        }
        if find_down(y, x, &map) {
            count += 1;
        }
        if find_dianogal_dl(y, x, map) {
            count += 1;
        }
        if find_dianogal_dr(y, x, map) {
            count += 1;
        }
        if find_dianogal_ul(y, x, map) {
            count += 1;
        }
        if find_dianogal_ur(y, x, map) {
            count += 1;
        }
    }
    return count;
}

fn find_right(y: usize, x: usize, map: &Vec<Vec<String>>) -> bool {
    if x + 3 > map[y].len() - 1 {
        return false;
    }
    if map[y][x + 3] == "S" && map[y][x + 2] == "A" && map[y][x + 1] == "M" {
        return true;
    }
    return false;
}

fn find_left(y: usize, x: usize, map: &Vec<Vec<String>>) -> bool {
    // println!("f_left: {x}, {y}");
    if x.checked_sub(3).is_none() {
        return false;
    }
    if map[y][x - 3] == "S" && map[y][x - 2] == "A" && map[y][x - 1] == "M" {
        return true;
    }
    return false;
}
fn find_up(y: usize, x: usize, map: &Vec<Vec<String>>) -> bool {
    if y.checked_sub(3).is_none() {
        return false;
    }
    if map.get(y - 3).unwrap().get(x).unwrap() == "S"
        && map[y - 2][x] == "A"
        && map[y - 1][x] == "M"
    {
        return true;
    }
    return false;
}

fn find_down(y: usize, x: usize, map: &Vec<Vec<String>>) -> bool {
    if y + 3 > map.len() - 1 {
        return false;
    }

    if map.get(y + 3).unwrap().get(x).unwrap() == "S"
        && map[y + 2][x] == "A"
        && map[y + 1][x] == "M"
    {
        return true;
    }
    return false;
}

fn find_dianogal_ur(y: usize, x: usize, map: &Vec<Vec<String>>) -> bool {
    if y.checked_sub(3).is_none() || x + 3 > map[y].len() - 1 {
        return false;
    }
    if map.get(y - 3).unwrap().get(x + 3).unwrap() == "S"
        && map[y - 2][x + 2] == "A"
        && map[y - 1][x + 1] == "M"
    {
        return true;
    }
    return false;
}

fn find_dianogal_dr(y: usize, x: usize, map: &Vec<Vec<String>>) -> bool {
    if y + 3 > map.len() - 1 || x + 3 > map[y].len() - 1 {
        return false;
    }
    if map.get(y + 3).unwrap().get(x + 3).unwrap() == "S"
        && map[y + 2][x + 2] == "A"
        && map[y + 1][x + 1] == "M"
    {
        return true;
    }
    return false;
}

fn find_dianogal_ul(y: usize, x: usize, map: &Vec<Vec<String>>) -> bool {
    if y.checked_sub(3).is_none() || x.checked_sub(3).is_none() {
        return false;
    }
    if map.get(y - 3).unwrap().get(x - 3).unwrap() == "S"
        && map[y - 2][x - 2] == "A"
        && map[y - 1][x - 1] == "M"
    {
        return true;
    }
    return false;
}

fn find_dianogal_dl(y: usize, x: usize, map: &Vec<Vec<String>>) -> bool {
    if y + 3 > map.len() - 1 || x.checked_sub(3).is_none() {
        return false;
    }
    if map.get(y + 3).unwrap().get(x - 3).unwrap() == "S"
        && map[y + 2][x - 2] == "A"
        && map[y + 1][x - 1] == "M"
    {
        return true;
    }
    return false;
}
