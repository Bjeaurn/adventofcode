mod file;

use file::file_utils::load_file;
fn main() {
    let file_name = "day2.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines();

    let mut games: Vec<Game> = Vec::new();

    for line in lines {
        games.push(parse_game(line).unwrap());
    }
    let compatible_games: Vec<&Game> = games
        .iter()
        .filter(|&game| {
            let compatible_rounds = game.filter_compatible_rounds();
            let len = compatible_rounds.len();
            let boolean = len == game.rounds.len();
            boolean
        })
        .collect();

    let result_1: i32 = compatible_games.iter().map(|game| game.id).sum();

    let min_stones: Vec<(i32, i32, i32)> = games
        .iter()
        .map(|game| game.find_lowest_color())
        .collect::<Vec<(i32, i32, i32)>>();

    let result_2: i32 = min_stones
        .iter()
        .map(|(red, green, blue)| red * green * blue)
        .sum();

    println!("{:?}", result_2);
}

pub fn parse_game(string: &str) -> Option<Game> {
    let mut iter = string.split(": ");
    let id = iter.next()?.split_whitespace().last()?.parse().ok()?;
    let round_data = iter.next()?.split("; ");

    let mut rounds = Vec::new();
    for round_info in round_data {
        let mut round = Round {
            green: 0,
            red: 0,
            blue: 0,
        };
        for entry in round_info.split(", ") {
            let mut parts = entry.split_whitespace();
            let count = parts.next()?.parse().ok()?;
            match parts.last()? {
                "green" => round.green = count,
                "red" => round.red = count,
                "blue" => round.blue = count,
                _ => {}
            }
        }
        rounds.push(round);
    }

    Some(Game { id, rounds })
}

impl Game {
    // Function to filter rounds that comply with the given rules
    fn filter_compatible_rounds(&self) -> Vec<&Round> {
        self.rounds
            .iter()
            .filter(|&round| {
                // Define your rules here
                round.red <= 12 && round.green <= 13 && round.blue <= 14
            })
            .collect()
    }

    fn find_lowest_color(&self) -> (i32, i32, i32) {
        let mut green = 0;
        let mut red = 0;
        let mut blue = 0;

        for round in &self.rounds {
            red = red.max(round.red);
            green = green.max(round.green);
            blue = blue.max(round.blue);
        }

        (red, green, blue)
    }
}

#[derive(Debug)]
pub struct Game {
    id: i32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
pub struct Round {
    green: i32,
    red: i32,
    blue: i32,
}
