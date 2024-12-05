use std::{cmp::Ordering, collections::HashMap, iter::Map};

use super::utils::file_utils::load_file;

#[derive(Debug, PartialEq, Eq)]
struct Rule(usize, usize);

pub fn main() {
    let file_name = "day5.txt".to_string();
    let data: String = load_file(file_name).unwrap();

    let (rules_raw, pages_raw) = setup_data(data);

    let rules: Vec<Rule> = rules_raw
        .into_iter()
        .map(|s| {
            let parts: Vec<&str> = s.split("|").collect();
            Rule(
                parts[0].trim().parse::<usize>().unwrap(),
                parts[1].trim().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let pages: Vec<Vec<usize>> = pages_raw
        .into_iter()
        .map(|s| {
            s.split(",")
                .map(|s| s.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let correct = check_pages(&pages, &rules);
    let result_part1: usize = correct.into_iter().map(|p| p[p.len() / 2]).sum();
    println!("{:?}", result_part1);

    let incorrect = check_incorrect(&pages, &rules);
    // println!("{:?}", incorrect);
    let fixed: Vec<Vec<usize>> = incorrect
        .into_iter()
        .map(|mut update| {
            update.sort_by(|a, b| {
                if rules.contains(&Rule(*a, *b)) {
                    Ordering::Less
                } else if rules.contains(&Rule(*a, *b)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .collect();
    println!(
        "part2: {:?}",
        fixed.into_iter().map(|p| p[p.len() / 2]).sum::<usize>()
    );
}

fn check_update(page: &Vec<usize>, rules: &Vec<Rule>) -> bool {
    page.is_sorted_by(|a, b| !rules.contains(&Rule(*b, *a)))
}

fn check_incorrect(pages: &Vec<Vec<usize>>, rules: &Vec<Rule>) -> Vec<Vec<usize>> {
    return pages
        .iter()
        .cloned()
        .filter(|page| !check_update(page, &rules))
        .collect();
}

fn check_pages(pages: &Vec<Vec<usize>>, rules: &Vec<Rule>) -> Vec<Vec<usize>> {
    let valid: Vec<Vec<usize>> = pages
        .iter()
        .cloned()
        .filter(|page| check_update(page, &rules))
        .collect();
    return valid;
}
// let mut correct: Vec<usize> = vec![];
// for pages in &pages {
//     let mut is_valid_page = true;
//     let mut seen: Vec<usize> = vec![];
//     pages.into_iter().for_each(|nr| {
//         seen.push(*nr);
//         let rule = rules.get(nr);
//         if rule.is_none() {
//             return;
//         }
//         if seen.contains(&rule.unwrap()) {
//             is_valid_page = false;
//             return;
//         }
//     });
//     if is_valid_page == true {
//         correct.push(pages);
//     }
// }
// correct

fn setup_data(data: String) -> (Vec<String>, Vec<String>) {
    let mut rules_raw: Vec<String> = vec![];
    let mut pages_raw: Vec<String> = vec![];

    let mut section = 0;
    for line in data.lines() {
        if line.trim() == "" {
            section = section + 1;
            continue;
        }
        match section {
            0 => rules_raw.push(line.to_string()),
            _ => pages_raw.push(line.to_string()),
        }
    }

    // println!("Rules: {:?}", rules_raw);
    // println!("Pages: {:?}", pages_raw);
    return (rules_raw, pages_raw);
}
