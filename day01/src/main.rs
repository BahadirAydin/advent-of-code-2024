use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn read_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let left: i32 = split[0].parse().unwrap();
        let right: i32 = split[1].parse().unwrap();
        first_list.push(left);
        second_list.push(right);
    }
    (first_list, second_list)
}

fn part1(input: &str) -> i32 {
    let (first_list, second_list) = read_lists(input);
    let mut l1_sorted = first_list.to_vec();
    let mut l2_sorted = second_list.to_vec();

    l1_sorted.sort_unstable();
    l2_sorted.sort_unstable();

    l1_sorted
        .iter()
        .zip(l2_sorted.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (first_list, second_list) = read_lists(input);
    let right_counts: HashMap<i32, usize> =
        second_list.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    first_list
        .iter()
        .map(|&left_num| {
            let count = right_counts.get(&left_num).cloned().unwrap_or(0);
            left_num * count as i32
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
