use itertools::Itertools;
use std::fs;
use std::time::Instant;

const MOVEMENTS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn parse_data(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap()) as i32)
                .collect()
        })
        .collect()
}

#[inline]
fn is_in(data: &Vec<Vec<i32>>, row: i32, col: i32) -> bool {
    0 <= row && row < data.len() as i32 && 0 <= col && col < data[0].len() as i32
}

fn is_valid(data: &Vec<Vec<i32>>, row: i32, col: i32, old_value: i32) -> bool {
    is_in(data, row, col) && old_value + 1 == data[row as usize][col as usize]
}

#[inline]
fn is_completed(data: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    data[row][col] == 9
}

fn get_next_locations(data: &Vec<Vec<i32>>, row: i32, col: i32) -> Vec<(i32, i32)> {
    MOVEMENTS
        .iter()
        .filter(|c| is_valid(data, row + c.0, col + c.1, data[row as usize][col as usize]))
        .map(|c| (row + c.0, col + c.1))
        .collect()
}

fn get_endings(data: &Vec<Vec<i32>>, start_row: i32, start_col: i32) -> Vec<(i32, i32)> {
    let mut vec = Vec::new();
    if is_completed(&data, start_row as usize, start_col as usize) {
        vec.push((start_row, start_col));
    } else {
        let next_locations = get_next_locations(data, start_row, start_col);
        for loc in next_locations {
            let (y, x) = loc;
            vec.append(&mut get_endings(data, y, x));
        }
    }
    vec
}

fn part1(input: &str) -> usize {
    let mut res = 0;
    let grid = parse_data(input);
    for (i, r) in grid.iter().enumerate() {
        for (j, val) in r.iter().enumerate() {
            if *val == 0 {
                let endings = get_endings(&grid, i as i32, j as i32);
                res += endings.iter().unique().count();
            }
        }
    }
    res
}

fn part2(input: &str) -> usize {
    let grid = parse_data(input);
    let mut res = 0;
    for (i, r) in grid.iter().enumerate() {
        for (j, val) in r.iter().enumerate() {
            if *val == 0 {
                res += get_endings(&grid, i as i32, j as i32).len();
            }
        }
    }
    res
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
