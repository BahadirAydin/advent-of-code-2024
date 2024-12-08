use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
    fn is_valid(&self, max_x: i32, max_y: i32) -> bool {
        self.x >= 0 && self.x < max_x && self.y >= 0 && self.y < max_y
    }
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_node_pairs(map: &[Vec<char>]) -> HashMap<char, Vec<Point>> {
    map.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, &ch)| {
                if ch != '.' {
                    Some((ch, Point::new(row, col)))
                } else {
                    None
                }
            })
        })
        .fold(HashMap::new(), |mut acc, (ch, point)| {
            acc.entry(ch).or_default().push(point);
            acc
        })
}

fn get_antinodes(pairs: &[Point], max_x: i32, max_y: i32) -> HashSet<Point> {
    pairs
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            pairs.iter().skip(i + 1).flat_map(|b| {
                let diff_x = b.x - a.x;
                let diff_y = b.y - a.y;
                [
                    Point {
                        x: a.x - diff_x,
                        y: a.y - diff_y,
                    },
                    Point {
                        x: b.x + diff_x,
                        y: b.y + diff_y,
                    },
                ]
            })
        })
        .filter(|p| p.is_valid(max_x, max_y))
        .collect()
}

fn get_antinodes_p2(pairs: &[Point], max_x: i32, max_y: i32) -> HashSet<Point> {
    let mut antinodes = HashSet::new();
    for i in 0..pairs.len() {
        for j in i + 1..pairs.len() {
            let a = pairs[i];
            let b = pairs[j];
            antinodes.insert(a);
            antinodes.insert(b);
            let diff_x = b.x - a.x;
            let diff_y = b.y - a.y;

            let mut k = 1;
            loop {
                let p1 = Point {
                    x: a.x - diff_x * k,
                    y: a.y - diff_y * k,
                };
                if !p1.is_valid(max_x, max_y) {
                    break;
                }
                antinodes.insert(p1);
                k += 1;
            }
            k = 1;
            loop {
                let p2 = Point {
                    x: b.x + diff_x * k,
                    y: b.y + diff_y * k,
                };
                if !p2.is_valid(max_x, max_y) {
                    break;
                }
                antinodes.insert(p2);
                k += 1;
            }
        }
    }
    antinodes
}

fn part1(input: &str) -> usize {
    let map = parse_map(input);
    let max_x = map.len() as i32;
    let max_y = map[0].len() as i32;
    find_node_pairs(&map)
        .values()
        .flat_map(|node_points| get_antinodes(node_points, max_x, max_y))
        .unique()
        .count()
}

fn part2(input: &str) -> usize {
    let map = parse_map(input);
    let max_x = map.len() as i32;
    let max_y = map[0].len() as i32;
    find_node_pairs(&map)
        .values()
        .flat_map(|node_points| get_antinodes_p2(node_points, max_x, max_y))
        .unique()
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
