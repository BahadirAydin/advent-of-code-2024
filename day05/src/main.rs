use std::cmp::Ordering;
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

fn is_valid(rules: &HashMap<&str, HashSet<&str>>, update: &[&str]) -> bool {
    let positions: HashMap<&str, usize> = update.iter().enumerate().map(|(i, &u)| (u, i)).collect();
    for (&before, after_set) in rules {
        if let Some(&before_index) = positions.get(before) {
            for &after in after_set {
                if let Some(&after_index) = positions.get(after) {
                    if after_index < before_index {
                        return false;
                    }
                }
            }
        }
    }
    true
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
    let mut precedence: HashMap<(&str, &str), Ordering> = HashMap::new();
    for (&before, after) in &rules {
        for &a in after {
            precedence.insert((before, a), Ordering::Greater);
            precedence.insert((a, before), Ordering::Less);
        }
    }
    updates
        .into_iter()
        .filter_map(|mut update| -> Option<i32> {
            if !is_valid(&rules, &update) {
                update.sort_by(|a, b| *precedence.get(&(a, b)).unwrap_or(&Ordering::Equal));
                update[update.len() / 2].parse::<i32>().ok()
            } else {
                None
            }
        })
        .sum()
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
