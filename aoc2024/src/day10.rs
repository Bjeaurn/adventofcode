use super::utils::file_utils::load_file;

#[derive(Debug)]
struct Map {
    data: Vec<usize>,
    width: usize,
}

impl Map {
    pub fn new(data: &String) -> Self {
        let width = data.lines().next().unwrap().len(); // Assuming all lines are of the same length
        Map {
            data: data
                .lines()
                .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
                .collect(),
            width: width,
        }
    }

    pub fn xy(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.data.len() / self.width {
            return usize::MAX;
        }
        // Implement the logic to find the value at (x, y)
        self.data[x * self.width + y]
    }

    pub fn index_to_xy(&self, index: usize) -> (usize, usize) {
        // Implement the logic to find the coordinates of a given index
        (index / self.width, index % self.width)
    }

    pub fn find_all(&self, target: usize) -> Vec<usize> {
        // Implement the logic to find all occurrences of target
        self.data
            .iter()
            .enumerate()
            .filter(|(_, &value)| value == target)
            .map(|(i, _)| i)
            .collect()
    }
}

pub fn main() {
    let file_name = "day10.txt".to_string();
    let data: String = load_file(file_name).unwrap();

    let map = Map::new(&data);
    let targets = map.find_all(0);
    let xy: Vec<(usize, usize)> = targets.iter().map(|i| map.index_to_xy(*i)).collect();
    // println!("{:?}", xy);
    let part1: Vec<usize> = xy.iter().map(|(x, y)| map.find_path((*x, *y), 9)).collect();
    let part2: Vec<usize> = xy
        .iter()
        .map(|(x, y)| map.find_path_part2((*x, *y), 9))
        .collect();
    println!("Part 1: {:?}", part1.iter().sum::<usize>());
    println!("Part 2: {:?}", part2.iter().sum::<usize>());
}

impl Map {
    pub fn find_path(&self, start: (usize, usize), target: usize) -> usize {
        let mut score: usize = 0;
        let mut visited: Vec<bool> = vec![false; self.data.len()];
        let mut queue: Vec<(usize, usize, usize)> =
            vec![(start.0, start.1, self.xy(start.0, start.1))];
        // println!("{:?}", queue);
        while !queue.is_empty() {
            let (x, y, value) = queue.pop().unwrap();
            // let v = self.xy(x, y);
            // if v != value {
            //     println!("WTF, how did {x}, {y} become {value} but is actually {v}");
            // }

            if self.data[x * self.width + y] == target && visited[x * self.width + y] == false {
                visited[x * self.width + y] = true;
                // println!(
                //     "Target found: ({}, {}), started: {}, {}",
                //     x, y, start.0, start.1
                // );
                score += 1;
                continue;
            }
            // println!("Checking now, queue is: {:?}", queue);
            if x.checked_sub(1).is_some() && self.xy(x - 1, y) == value + 1 {
                queue.extend([(x - 1, y, value + 1)]);
            }
            if self.xy(x + 1, y) == value + 1 {
                queue.extend([(x + 1, y, value + 1)]);
            }
            if y.checked_sub(1).is_some() && self.xy(x, y - 1) == value + 1 {
                queue.extend([(x, y - 1, value + 1)]);
            }
            if self.xy(x, y + 1) == value + 1 {
                queue.extend([(x, y + 1, value + 1)]);
            }
            // Debugging
            // if start.0 == 0 && start.1 == 2 {
            //     println!("{:?}", queue);
            // }
        }

        score
    }

    pub fn find_path_part2(&self, start: (usize, usize), target: usize) -> usize {
        let mut score: usize = 0;
        let mut queue: Vec<(usize, usize, usize)> =
            vec![(start.0, start.1, self.xy(start.0, start.1))];
        // println!("{:?}", queue);
        while !queue.is_empty() {
            let (x, y, value) = queue.pop().unwrap();
            // let v = self.xy(x, y);
            // if v != value {
            //     println!("WTF, how did {x}, {y} become {value} but is actually {v}");
            // }

            if self.data[x * self.width + y] == target {
                // println!(
                //     "Target found: ({}, {}), started: {}, {}",
                //     x, y, start.0, start.1
                // );
                score += 1;
                continue;
            }
            // println!("Checking now, queue is: {:?}", queue);
            if x.checked_sub(1).is_some() && self.xy(x - 1, y) == value + 1 {
                queue.extend([(x - 1, y, value + 1)]);
            }
            if self.xy(x + 1, y) == value + 1 {
                queue.extend([(x + 1, y, value + 1)]);
            }
            if y.checked_sub(1).is_some() && self.xy(x, y - 1) == value + 1 {
                queue.extend([(x, y - 1, value + 1)]);
            }
            if self.xy(x, y + 1) == value + 1 {
                queue.extend([(x, y + 1, value + 1)]);
            }
            // Debugging
            // if start.0 == 0 && start.1 == 2 {
            // println!("{:?}", queue);
            // }
        }

        score
    }
}
