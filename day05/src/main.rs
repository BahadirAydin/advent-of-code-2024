use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn read_input(content: &str) -> (HashMap<&str, HashSet<&str>>, Vec<Vec<&str>>) {
    let sections: Vec<&str> = content.lines().collect();
    let split_index = sections
        .iter()
        .position(|&line| line.is_empty())
        .unwrap_or(sections.len());

    let mut rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in &sections[..split_index] {
        let parts: Vec<_> = line.split('|').collect();
        let before = parts[0];
        let after = parts[1];
        rules.entry(before).or_insert_with(HashSet::new);
        rules.entry(after).or_insert_with(HashSet::new);
        rules.get_mut(&before).unwrap().insert(after);
    }

    let updates: Vec<Vec<&str>> = sections[split_index + 1..]
        .iter()
        .map(|update_line| update_line.split(',').collect())
        .collect();
    (rules, updates)
}

fn is_valid(rules: &HashMap<&str, HashSet<&str>>, update: &Vec<&str>) -> bool {
    for (&before, after) in rules {
        match update.iter().position(|&u| u == before) {
            Some(before_index) => {
                for &v in after {
                    match update.iter().position(|&u| u == v) {
                        Some(after_index) => {
                            if after_index < before_index {
                                return false;
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }
    true
}

fn sort(update: &mut Vec<&str>, rules: &HashMap<&str, HashSet<&str>>) {
    update.sort_by(|a, b| {
        for (&before, after) in rules {
            if &before == a && after.contains(b) {
                return std::cmp::Ordering::Greater;
            }
            if &before == b && after.contains(a) {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    });
}

fn part1(input: &str) -> i32 {
    let (rules, updates) = read_input(input);
    let mut sum = 0;
    for update in updates {
        if is_valid(&rules, &update) {
            sum += update[update.len() / 2].parse::<i32>().unwrap();
        }
    }
    sum
}
fn part2(input: &str) -> i32 {
    let (rules, updates) = read_input(input);
    let mut sum = 0;
    for mut update in updates {
        sort(&mut update, &rules);
        sum += update[update.len() / 2].parse::<i32>().unwrap();
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let start = Instant::now();
    let res1 = part1(&input);
    let duration = start.elapsed();
    println!("Part 1: {} took {:#?}", res1, duration);
    let start = Instant::now();
    let res2 = part2(&input);
    let duration = start.elapsed();
    println!("Part 2: {} took {:#?}", res2, duration);
}
