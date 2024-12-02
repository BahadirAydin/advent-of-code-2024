use std::fs;
use std::time::Instant;

fn is_valid(nums: &Vec<i32>) -> bool {
    let increasing = nums.first() < nums.last();
    nums.windows(2).all(|w| {
        ((increasing && w[0] < w[1]) || (!increasing && w[0] > w[1])) && ((w[0] - w[1]).abs() <= 3)
    })
}

fn is_valid_part2(nums: &Vec<i32>) -> bool {
    (0..nums.len()).any(|i| is_valid(&[&nums[..i], &nums[i + 1..]].concat()))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|nums| is_valid(nums))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|nums| is_valid_part2(nums))
        .count()
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
