mod file;

use file::file_utils::load_file;

fn main() {
    let file_name = "day3_example.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines();

    let mut arr: Vec<String> = vec![];
    let width = lines.clone().nth(0).unwrap().len() as i32;
    for line in lines {
        for ch in line.to_string().chars() {
            let val = match ch {
                '.' => "".to_string(),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    ch.to_digit(10).unwrap() as i32
                }
                _ => ch.to_string(),
            };
            arr.push(val)
        }
    }
    let mut pos: Vec<i32> = vec![];
    let chars: Vec<String> = arr
        .iter()
        .enumerate()
        .filter(|(i, v)| match v.as_str() {
            "#" | "?" | "+" | "*" => true,
            _ => false,
            // v if v. => false,
        })
        .map(|(i, v)| (i, v, index_to_xy(i as i32, &width)))
        .flat_map(|(i, v, coords)| {
            let results = find_neighbours(coords.0, coords.1, width, &arr)
                .iter()
                .filter(|v| v.as_str() != "")
                .cloned()
                .collect::<Vec<String>>();
            results
        })
        .collect();

    // println!("{}", get_xy(0, 3, &width, &arr).unwrap());
    // println!("{:?}, {}", arr, width);
    println!("{:?}", chars);
}

fn find_neighbours(x: i32, y: i32, width: i32, arr: &Vec<String>) -> Vec<String> {
    let mut neighbors = Vec::new();
    let offsets = [
        // Top
        (-1, -1),
        (0, -1),
        (1, -1),
        // Center
        (-1, 0),
        /* (x, y) */
        (1, 0),
        // Bottom
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let height = arr.len() as i32 / width;

    for (ox, oy) in offsets.iter() {
        let dx = x + ox;
        let dy = y + oy;

        if dx >= 0 && dy >= 0 && dx < width && dy < height {
            let neighbor_index = index(dx, dy, width) as usize;
            neighbors.push(arr.get(neighbor_index).unwrap().clone());
        }
    }
    neighbors
}

fn index(x: i32, y: i32, width: i32) -> i32 {
    y + width * x
}

fn index_to_xy(idx: i32, width: &i32) -> Coord {
    let y = idx % width;
    let x = idx / width;
    (x, y) as Coord
}

fn get_xy<'a>(ver_x: i32, hor_y: i32, width: &'a i32, arr: &'a Vec<String>) -> Option<&'a String> {
    let index: usize = (ver_x * width + hor_y) as usize;
    arr.get(index)
}

type Coord = (i32, i32);
