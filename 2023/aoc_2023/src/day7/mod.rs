use std::collections::HashMap;

use super::utils::file_utils::load_file;
pub fn main() {
    let file_name = "day7.example.txt".to_string();
    let data = load_file(file_name).unwrap();
    let lines = data.lines();

    let mut hands: Vec<Hand> = vec![];

    for line in lines {
        let mut split = line.split_whitespace();
        let hand = Hand::new(
            split.next().unwrap(),
            split.next().unwrap().parse::<u64>().unwrap(),
        );

        hands.push(hand);
    }
    hands.sort();
    println!("{:?}", hands);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand<'a> {
    hand_str: &'a str,
    hand: Vec<Value>,
    // sorted_hand: Vec<Value>,
    value: HandRank,
    bet: u64,
}

impl<'a> Hand<'a> {
    pub fn new(line: &'a str, bet: u64) -> Self {
        let hand = Hand::get_hand(line);
        let value_hand = hand.clone();
        // let mut sorted_hand: Vec<Value> = hand.clone();
        // sorted_hand.sort();
        Self {
            hand_str: line,
            hand: hand,
            // sorted_hand: sorted_hand,
            value: Self::get_hand_rank(value_hand),
            bet: bet,
        }
    }

    fn get_hand(hand_str: &str) -> Vec<Value> {
        let hand = hand_str
            .chars()
            .map(|c| match c {
                '1' => Value::One,
                '2' => Value::Two,
                '3' => Value::Three,
                '4' => Value::Four,
                '5' => Value::Five,
                '6' => Value::Six,
                '7' => Value::Seven,
                '8' => Value::Eight,
                '9' => Value::Nine,
                'T' => Value::Ten,
                'J' => Value::Jack,
                'Q' => Value::Queen,
                'K' => Value::King,
                'A' => Value::Ace,
                _ => unreachable!(),
            })
            .collect();
        hand
    }

    fn get_hand_rank(hand: Vec<Value>) -> HandRank {
        HandRank::new(hand)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Hash)]

enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FourOfAKind,
    FiveOfAKind,
}

impl HandRank {
    pub fn new(hand: Vec<Value>) -> HandRank {
        let mut value_count: HashMap<&Value, usize> = HashMap::new();
        for card in &hand {
            *value_count.entry(card).or_insert(0) += 1;
        }

        if Self::is_five_of_a_kind(&value_count) {
            return Self::FiveOfAKind;
        }
        if Self::is_four_of_a_kind(&value_count) {
            return Self::FourOfAKind;
        }
        if Self::is_three_of_a_kind(&value_count) {
            return Self::ThreeOfAKind;
        }
        if Self::is_two_pair(&value_count) {
            return Self::TwoPair;
        }
        if Self::is_one_pair(&value_count) {
            return Self::OnePair;
        }
        Self::HighCard
    }

    fn is_five_of_a_kind(value_count: &HashMap<&Value, usize>) -> bool {
        value_count.values().any(|&count| count == 5)
    }

    fn is_four_of_a_kind(value_count: &HashMap<&Value, usize>) -> bool {
        value_count.values().any(|&count| count == 4)
    }

    fn is_three_of_a_kind(value_count: &HashMap<&Value, usize>) -> bool {
        value_count.values().any(|&count| count == 3)
    }

    fn is_two_pair(value_count: &HashMap<&Value, usize>) -> bool {
        value_count.values().any(|&count| count == 2)
    }

    fn is_one_pair(value_count: &HashMap<&Value, usize>) -> bool {
        value_count.values().any(|&count| count == 1)
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Implement custom comparison logic here based on Poker hand rankings
        // You'll need to define the relative ordering among the hand ranks
        match (self, other) {
            (HandRank::FiveOfAKind, HandRank::FiveOfAKind) => std::cmp::Ordering::Equal, // Needs a thing.
            (HandRank::FiveOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandRank::FiveOfAKind) => std::cmp::Ordering::Less,

            (HandRank::FourOfAKind, HandRank::FourOfAKind) => std::cmp::Ordering::Equal,
            (HandRank::FourOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandRank::FourOfAKind) => std::cmp::Ordering::Less,

            (HandRank::ThreeOfAKind, HandRank::ThreeOfAKind) => std::cmp::Ordering::Equal,
            (HandRank::ThreeOfAKind, _) => std::cmp::Ordering::Greater,
            (_, HandRank::ThreeOfAKind) => std::cmp::Ordering::Less,

            (HandRank::TwoPair, HandRank::TwoPair) => std::cmp::Ordering::Equal,
            (HandRank::TwoPair, _) => std::cmp::Ordering::Greater,
            (_, HandRank::TwoPair) => std::cmp::Ordering::Less,

            (HandRank::OnePair, HandRank::OnePair) => std::cmp::Ordering::Equal,
            (HandRank::OnePair, _) => std::cmp::Ordering::Greater,
            (_, HandRank::OnePair) => std::cmp::Ordering::Less,

            (HandRank::HighCard, HandRank::HighCard) => std::cmp::Ordering::Equal,
            (HandRank::HighCard, _) => std::cmp::Ordering::Greater,
            (_, HandRank::HighCard) => std::cmp::Ordering::Less,
            // Add comparisons for other hand ranks here...
            _ => std::cmp::Ordering::Equal, // Default to Equal if unspecified
        }
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Implement partial comparison logic for HandRank here
        // ...

        // This is just a placeholder logic; update it based on your actual comparison logic
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
