use std::fs;
use std::time::Instant;

const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];

fn part1(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let rows = grid.len();
    let cols = grid[0].len();

    let is_valid = |x: usize, y: usize| -> bool { x < rows && y < cols };
    let mut count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            for (dx, dy) in DIRS {
                let word: String = (0..4)
                    .map(|k| {
                        let x = i as i32 + dx * k;
                        let y = j as i32 + dy * k;
                        if x < 0 || y < 0 {
                            return ' ';
                        } else if !is_valid(x as usize, y as usize) {
                            return ' ';
                        } else {
                            grid[x as usize][y as usize]
                        }
                    })
                    .collect();
                if word == "XMAS" {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_mas(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 {
        return false;
    }
    let rows = grid.len();
    let cols = grid[0].len();

    let is_valid = |x: usize, y: usize| -> bool { x < rows && y < cols };

    // north-east and south-west should be like: MS or SM
    // similarly: north-west and south-east should be like: MS or SM
    if is_valid(i + 1, j + 1) && is_valid(i - 1, j - 1) {
        if !((grid[i + 1][j + 1] == 'M' && grid[i - 1][j - 1] == 'S')
            || (grid[i + 1][j + 1] == 'S' && grid[i - 1][j - 1] == 'M'))
        {
            return false;
        }
    } else {
        return false;
    }

    if is_valid(i + 1, j - 1) && is_valid(i - 1, j + 1) {
        if !((grid[i + 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S')
            || (grid[i + 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'M'))
        {
            return false;
        }
    } else {
        return false;
    }

    true
}

fn part2(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == 'A' && check_mas(i, j, &grid) {
                count += 1;
            }
        }
    }
    count
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
