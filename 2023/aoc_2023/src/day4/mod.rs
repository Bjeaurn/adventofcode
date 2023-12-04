use std::collections::HashMap;

use super::utils::file_utils::load_file;
pub fn main() {
    let file_name = "day4.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines();
    let mut cards: Vec<Card> = vec![];
    let mut card_instances: HashMap<usize, i32> = HashMap::new();

    for line in lines {
        cards.push(Card::new(line));
    }

    let _: Vec<&mut Card> = cards
        .iter_mut()
        .enumerate()
        .map(|(idx, card)| {
            let (len, points) = card.score();
            card.points = points;
            // println!("-- idx: {}, len: {} ---", idx, len);
            for i in (idx + 1)..(idx + len as usize + 1) {
                let times = card_instances.get(&idx).unwrap_or(&1);
                // println!("idx: {}, i: {}, times: {}", idx, i, times);
                *card_instances.entry(i).or_insert(1) += 1 * times;
            }
            card
        })
        .collect();
    println!("{:?}", card_instances);
    let scores: Vec<(i16, i32)> = cards.iter().map(|card| card.score()).collect();
    // let wins = scores
    //     .clone()
    //     .iter()
    //     .enumerate()
    //     .map(|(idx, (len, score))| (idx, len, score));
    // cards.iter().enumerate().

    println!("{}", scores.iter().map(|(_, b)| b).sum::<i32>());
    println!(
        "{:?}",
        cards
            .iter()
            .enumerate()
            .map(|(idx, _)| card_instances.get(&idx).unwrap_or(&1))
            .sum::<i32>()
    );
}

type Number = i16;

#[derive(Debug)]
pub struct Card {
    id: Number,
    winning: Vec<Number>,
    on_card: Vec<Number>,
    // instances: Number,
    points: i32,
}

impl Card {
    // pub fn new(line: &str) -> Self {
    //     let parts: Vec<&str> = line.split(":").collect();
    //     let winning_numbers = parts[1]
    //         .split("|")
    //         .next()
    //         .unwrap()
    //         .split_whitespace()
    //         .collect();
    //     let numbers_given = parts[1]
    //         .split("|")
    //         .nth(1)
    //         .unwrap()
    //         .split_whitespace()
    //         .collect();

    //     Card {
    //         id: id,
    //         winning: winning_numbers,
    //         on_card: numbers_given,
    //     }
    // }
    pub fn new(line: &str) -> Self {
        let mut iter = line.split(": ");

        let id = iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<Number>()
            .unwrap();

        let left: Vec<Number> = iter
            .clone()
            .next()
            .unwrap()
            .split("|")
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .clone()
            .map(|v| v.parse::<Number>())
            .map(|v| v.unwrap())
            .collect();

        let right: Vec<Number> = iter
            .clone()
            .next()
            .unwrap()
            .split("|")
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .clone()
            .map(|v| v.parse::<Number>())
            .map(|v| v.unwrap())
            .collect();

        Card {
            id: id,
            winning: left,
            on_card: right,
            points: 0,
        }
    }

    fn score(&self) -> (i16, i32) {
        let won_matches: Vec<Number> = self
            .winning
            .iter()
            .filter(|w| self.on_card.contains(w))
            .cloned()
            .collect();

        // println!("{:?}", won_matches);

        let len: i16 = won_matches.len() as i16;
        let score: i32 = match len {
            0 => 0,
            1 => 1,
            len => 1 * (1 << (len - 1)),
        } as i32;
        (len, score)
    }
}

// fn score(matches: usize) -> i32 {
//     match matches {
//         0 => 0,
//         1 => 1,
//         len => 1 * (1 << (len - 1)),
//     }
// }
