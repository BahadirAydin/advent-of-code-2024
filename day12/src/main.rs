use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const MOVEMENTS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn parse_data(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[inline]
fn is_in(grid: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && (row as usize) < grid.len() && (col as usize) < grid[0].len()
}

fn get_perimeter_for_cell(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    MOVEMENTS
        .iter()
        .filter(|&&(dy, dx)| {
            let new_row = row as i32 + dy;
            let new_col = col as i32 + dx;

            !is_in(grid, new_row, new_col)
                || grid[new_row as usize][new_col as usize] != grid[row][col]
        })
        .count()
}

fn get_next_locations(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    region_char: char,
) -> Vec<(usize, usize)> {
    MOVEMENTS
        .iter()
        .filter(|&&(dy, dx)| {
            let new_row = row as i32 + dy;
            let new_col = col as i32 + dx;

            is_in(grid, new_row, new_col) && grid[new_row as usize][new_col as usize] == region_char
        })
        .map(|&(dy, dx)| (row + dy as usize, col + dx as usize))
        .collect()
}

fn get_area_and_perimeter(
    grid: &mut Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
) -> (usize, usize) {
    let mut stack = vec![(start_row, start_col)];
    let mut area = 0;
    let mut perimeter = 0;
    let region_char = grid[start_row][start_col];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    // Perform DFS to calculate area and perimeter, marking cells as visited
    while let Some((r, c)) = stack.pop() {
        if !visited[r][c] {
            perimeter += get_perimeter_for_cell(grid, r, c);
            area += 1;
            visited[r][c] = true;

            let next_locations = get_next_locations(grid, r, c, region_char);
            stack.extend(next_locations);
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited[i][j] {
                grid[i][j] = '*';
            }
        }
    }

    (area, perimeter)
}

fn find_number_of_sides(path: Vec<(usize, usize)>) -> usize {
    let mut corners: HashMap<(i32, i32), Vec<(usize, usize)>> = HashMap::new();
    const CORNER_MOVEMENTS: [(f32, f32); 4] = [(0.5, 0.5), (0.5, -0.5), (-0.5, 0.5), (-0.5, -0.5)];

    // Record all corners and the points contributing to them
    for &(r, c) in path.iter() {
        for i in CORNER_MOVEMENTS.iter() {
            let new_r = (r as f32 + i.0).round() as i32;
            let new_c = (c as f32 + i.1).round() as i32;
            corners.entry((new_r, new_c)).or_default().push((r, c));
        }
    }

    let mut sides = 0;

    for (_corner, points) in corners {
        let count = points.len();
        if count % 2 == 1 {
            sides += 1;
        }

        if count == 2 {
            // EDGE CASE:
            // AAAAAA
            // AAABBA
            // AAABBA
            // ABBAAA
            // ABBAAA
            // AAAAAA

            // A has 12 sides, fences do not go through the diagonal touching B

            let (p1, p2) = (points[0], points[1]);
            let is_diagonal =
                (p1.0 as i32 - p2.0 as i32).abs() == 1 && (p1.1 as i32 - p2.1 as i32).abs() == 1;
            if is_diagonal {
                sides += 2;
            }
        }
    }

    sides
}

fn get_area_and_sides(
    grid: &mut Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
) -> (usize, usize) {
    let mut stack = vec![(start_row, start_col)];
    let mut area = 0;
    let region_char = grid[start_row][start_col];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut path = Vec::new();

    while let Some((r, c)) = stack.pop() {
        if !visited[r][c] {
            path.push((r, c));
            area += 1;
            visited[r][c] = true;

            let next_locations = get_next_locations(grid, r, c, region_char);
            stack.extend(next_locations);
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited[i][j] {
                grid[i][j] = '*';
            }
        }
    }

    (area, find_number_of_sides(path))
}

fn part1(input: &str) -> usize {
    let mut grid = parse_data(input);
    let mut total_sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '*' {
                let (area, perimeter) = get_area_and_perimeter(&mut grid, i, j);
                total_sum += area * perimeter;
            }
        }
    }
    total_sum
}

fn part2(input: &str) -> usize {
    let mut grid = parse_data(input);
    let mut total_sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '*' {
                let (area, sides) = get_area_and_sides(&mut grid, i, j);
                total_sum += area * sides;
            }
        }
    }
    total_sum
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
