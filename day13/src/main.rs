use nalgebra::{Matrix2, Vector2};
use std::fs;
use std::time::Instant;

struct Button {
    x: i64,
    y: i64,
}

struct Game {
    button_a: Button,
    button_b: Button,
    prize: (i64, i64),
}

fn read_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|chunk| {
            if chunk.len() == 3 {
                Some(Game {
                    button_a: parse_button(chunk[0]),
                    button_b: parse_button(chunk[1]),
                    prize: parse_prize(chunk[2]),
                })
            } else {
                None
            }
        })
        .collect()
}

fn parse_button(line: &str) -> Button {
    let parts: Vec<&str> = line.split(": ").nth(1).unwrap().split(", ").collect();
    Button {
        x: parts[0][1..].parse().unwrap(),
        y: parts[1][1..].parse().unwrap(),
    }
}

fn parse_prize(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split(": ").nth(1).unwrap().split(", ").collect();
    (
        parts[0][2..].parse().unwrap(),
        parts[1][2..].parse().unwrap(),
    )
}

fn solve_equation(game: &Game) -> Option<(i64, i64)> {
    let matrix = Matrix2::new(
        game.button_a.x as f64,
        game.button_b.x as f64,
        game.button_a.y as f64,
        game.button_b.y as f64,
    );
    let vector = Vector2::new(game.prize.0 as f64, game.prize.1 as f64);
    matrix.try_inverse().and_then(|inv| {
        let solution = inv * vector;
        let r1 = solution[0].round() as i64;
        let r2 = solution[1].round() as i64;

        let reconstructed_prize_x = game.button_a.x * r1 + game.button_b.x * r2;
        let reconstructed_prize_y = game.button_a.y * r1 + game.button_b.y * r2;

        if reconstructed_prize_x == game.prize.0 && reconstructed_prize_y == game.prize.1 {
            Some((r1, r2))
        } else {
            None
        }
    })
}

fn get_price(games: &[Game]) -> i64 {
    games
        .iter()
        .filter_map(|g| solve_equation(g).map(|s| s.0 * A_PRICE + s.1 * B_PRICE))
        .sum()
}

const A_PRICE: i64 = 3;
const B_PRICE: i64 = 1;

fn part1(input: &str) -> i64 {
    let games = read_input(input);
    get_price(&games)
}

const AMOUNT_TO_ADD: i64 = 10000000000000;
fn part2(input: &str) -> i64 {
    let mut games = read_input(input);
    for g in &mut games {
        g.prize.0 += AMOUNT_TO_ADD;
        g.prize.1 += AMOUNT_TO_ADD;
    }
    get_price(&games)
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
