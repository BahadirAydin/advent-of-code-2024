use regex::Regex;
use std::fs;
use std::time::Instant;

fn part1(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    regex.captures_iter(input).fold(0, |acc, cap| {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        acc + a * b
    })
}

fn part2(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),\s*(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut disabled = false;
    let mut sum = 0;
    for cap in regex.captures_iter(input) {
        if cap.get(0).unwrap().as_str() == "do()" {
            disabled = false;
        } else if cap.get(0).unwrap().as_str() == "don't()" {
            disabled = true;
        } else if !disabled {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            sum += a * b;
        }
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
