use itertools::Itertools;
use std::fs;
use std::time::Instant;

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let val: i64 = parts[0].parse().unwrap();
            let numbers: Vec<i64> = parts[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (val, numbers)
        })
        .collect()
}

fn generate_operator_combinations<'a>(
    length: usize,
    operators: &'a [&'a str],
) -> impl Iterator<Item = Vec<&'a str>> + 'a {
    std::iter::repeat(operators)
        .take(length)
        .multi_cartesian_product()
        .map(|vec| vec.into_iter().copied().collect()) // Convert Vec<&&str> to Vec<&str>
}

fn part1(input: &str) -> i64 {
    let data = parse_input(input);

    data.iter()
        .filter(|(val, numbers)| {
            let mut combinations = generate_operator_combinations(numbers.len() - 1, &["+", "*"]);
            combinations.any(|ops| {
                let mut result = numbers[0];
                for (idx, op) in ops.iter().enumerate() {
                    if result > *val {
                        break;
                    }
                    match *op {
                        "+" => result += numbers[idx + 1],
                        "*" => result *= numbers[idx + 1],
                        _ => unreachable!(),
                    }
                }
                result == *val
            })
        })
        .map(|(val, _)| *val)
        .sum()
}

fn concatenate(a: i64, b: i64) -> i64 {
    let b_digits = (b as f64).log10() as u32 + 1;
    a * 10_i64.pow(b_digits as u32) + b
}

fn part2(input: &str) -> i64 {
    let data = parse_input(input);

    data.iter()
        .filter(|(val, numbers)| {
            let mut combinations =
                generate_operator_combinations(numbers.len() - 1, &["+", "*", "||"]);
            combinations.any(|ops| {
                let mut result = numbers[0];
                for (idx, op) in ops.iter().enumerate() {
                    match *op {
                        "+" => result += numbers[idx + 1],
                        "*" => result *= numbers[idx + 1],
                        "||" => result = concatenate(result, numbers[idx + 1]),
                        _ => unreachable!(),
                    }
                    if result > *val {
                        return false;
                    }
                }
                result == *val
            })
        })
        .map(|(val, _)| *val)
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
