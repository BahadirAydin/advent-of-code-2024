use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn find_next_block(
    current_row: usize,
    current_col: usize,
    blocks_positions: &[(usize, usize)],
    direction: &Direction,
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => blocks_positions
            .iter()
            .filter(|&&(row, col)| row < current_row && col == current_col)
            .max_by_key(|&&(row, _)| row)
            .map(|&(row, col)| (row + 1, col)),
        Direction::Down => blocks_positions
            .iter()
            .filter(|&&(row, col)| row > current_row && col == current_col)
            .min_by_key(|&&(row, _)| row)
            .map(|&(row, col)| (row - 1, col)),
        Direction::Left => blocks_positions
            .iter()
            .filter(|&&(row, col)| col < current_col && row == current_row)
            .max_by_key(|&&(_, col)| col)
            .map(|&(row, col)| (row, col + 1)),
        Direction::Right => blocks_positions
            .iter()
            .filter(|&&(row, col)| col > current_col && row == current_row)
            .min_by_key(|&&(_, col)| col)
            .map(|&(row, col)| (row, col - 1)),
    }
}

fn get_info(map: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, (usize, usize)) {
    let blocks_positions: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, &c)| if c == '#' { Some((row, col)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect();
    let (row, col) = map
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find_map(|(col, &c)| if c == '^' { Some((row, col)) } else { None })
        })
        .unwrap();

    (blocks_positions, (row, col))
}

fn part1(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (blocks_positions, (mut row, mut col)) = get_info(&map);

    let mut direction = Direction::Up;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    visited.insert((row, col));

    loop {
        if let Some((new_row, new_col)) = find_next_block(row, col, &blocks_positions, &direction) {
            match direction {
                Direction::Up => (new_row..row).rev().for_each(|r| {
                    visited.insert((r, col));
                }),
                Direction::Down => (row..=new_row).for_each(|r| {
                    visited.insert((r, col));
                }),
                Direction::Left => (new_col..col).rev().for_each(|c| {
                    visited.insert((row, c));
                }),
                Direction::Right => (col..=new_col).for_each(|c| {
                    visited.insert((row, c));
                }),
            }

            direction = direction.turn_right();
            (row, col) = (new_row, new_col);
        } else {
            let range: Vec<_> = match direction {
                Direction::Up => (0..row).rev().map(|r| (r, col)).collect(),
                Direction::Down => (row..map.len()).map(|r| (r, col)).collect(),
                Direction::Left => (0..col).rev().map(|c| (row, c)).collect(),
                Direction::Right => (col..map[0].len()).map(|c| (row, c)).collect(),
            };

            visited.extend(range);
            break;
        }
    }
    visited.len() as i32
}

fn loop_detected(
    mut row: usize,
    mut col: usize,
    blocks: &Vec<(usize, usize)>,
    rown: usize,
    coln: usize,
) -> bool {
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut direction = Direction::Up;

    visited.insert(((row, col), direction));
    loop {
        if let Some((new_row, new_col)) = find_next_block(row, col, &blocks, &direction) {
            match direction {
                Direction::Up => (new_row..row).rev().for_each(|r| {
                    visited.insert(((r, col), direction));
                }),
                Direction::Down => (row..=new_row).for_each(|r| {
                    visited.insert(((r, col), direction));
                }),
                Direction::Left => (new_col..col).rev().for_each(|c| {
                    visited.insert(((row, c), direction));
                }),
                Direction::Right => (col..=new_col).for_each(|c| {
                    visited.insert(((row, c), direction));
                }),
            }
            direction = direction.turn_right();
            (row, col) = (new_row, new_col);
        } else {
            let range: Vec<_> = match direction {
                Direction::Up => (0..row).rev().map(|r| ((r, col), direction)).collect(),
                Direction::Down => (row..rown).map(|r| ((r, col), direction)).collect(),
                Direction::Left => (0..col).rev().map(|c| ((row, c), direction)).collect(),
                Direction::Right => (col..coln).map(|c| ((row, c), direction)).collect(),
            };

            visited.extend(range);
            break;
        }
        if visited.contains(&((row, col), direction)) {
            return true;
        }
    }
    false
}

fn part2(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (blocks_positions, (row, col)) = get_info(&map);
    let (rown, coln) = (map.len(), map[0].len());

    let mut loops = 0;
    for r in 0..rown {
        for c in 0..coln {
            if map[r][c] == '.' {
                let mut copy_blocks = blocks_positions.clone();
                copy_blocks.push((r, c));
                if loop_detected(row, col, &copy_blocks, rown, coln) {
                    loops += 1;
                }
            }
        }
    }
    loops
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
