use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn parse_data(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_digit_count(number: u64) -> u32 {
    let mut count = 0;
    let mut num = number;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn transform_stone(stone: u64) -> [Option<u64>; 2] {
    if stone == 0 {
        [Some(1), None]
    } else {
        let digit_count = get_digit_count(stone);
        if digit_count % 2 == 0 {
            let half = digit_count / 2;
            let divisor = 10u64.pow(half as u32);
            let left = stone / divisor;
            let right = stone % divisor;

            [Some(left), Some(right)]
        } else {
            [Some(stone * 2024), None]
        }
    }
}

fn transform_n_times(stone: u64, n: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if n == 0 {
        return 1;
    }
    if let Some(&val) = cache.get(&(stone, n)) {
        return val;
    }
    let size = transform_stone(stone)
        .iter()
        .filter_map(|&x| {
            if let Some(t) = x {
                Some(transform_n_times(t, n - 1, cache))
            } else {
                None
            }
        })
        .sum();
    cache.insert((stone, n), size);
    size
}

fn part1(input: &str) -> u64 {
    const BLINK: u8 = 25;
    let stones = parse_data(input);
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|x| transform_n_times(*x, BLINK, &mut cache))
        .sum()
}

fn part2(input: &str) -> u64 {
    const BLINK: u8 = 75;
    let stones = parse_data(input);
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|x| transform_n_times(*x, BLINK, &mut cache))
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
