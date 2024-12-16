use std::fs;
use std::time::Instant;

struct Position {
    x: i32,
    y: i32,
}

struct Velocity {
    x: i32,
    y: i32,
}

struct Robot {
    position: Position,
    velocity: Velocity,
}

fn read_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let pos_str = parts[0].trim_start_matches("p=");
            let pos_coords: Vec<i32> = pos_str
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect();

            let vel_str = parts[1].trim_start_matches("v=");
            let vel_coords: Vec<i32> = vel_str
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect();

            Robot {
                position: Position {
                    x: pos_coords[0],
                    y: pos_coords[1],
                },
                velocity: Velocity {
                    x: vel_coords[0],
                    y: vel_coords[1],
                },
            }
        })
        .collect()
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

const SECONDS: i32 = 100;

fn move_robot(robot: &Robot, times: i32) -> Position {
    let x = (robot.position.x + robot.velocity.x * times).rem_euclid(WIDTH);
    let y = (robot.position.y + robot.velocity.y * times).rem_euclid(HEIGHT);
    Position { x, y }
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Middle,
}

fn determine_quadrant(pos: &Position) -> Quadrant {
    if pos.x < WIDTH / 2 && pos.y < HEIGHT / 2 {
        Quadrant::TopLeft
    } else if pos.x > WIDTH / 2 && pos.y < HEIGHT / 2 {
        Quadrant::TopRight
    } else if pos.x < WIDTH / 2 && pos.y > HEIGHT / 2 {
        Quadrant::BottomLeft
    } else if pos.x > WIDTH / 2 && pos.y > HEIGHT / 2 {
        Quadrant::BottomRight
    } else {
        Quadrant::Middle
    }
}

fn calculate_safety_factor(robots: &Vec<Robot>) -> i32 {
    let mut num_q1 = 0;
    let mut num_q2 = 0;
    let mut num_q3 = 0;
    let mut num_q4 = 0;

    robots
        .iter()
        .for_each(|robot| match determine_quadrant(&robot.position) {
            Quadrant::TopLeft => num_q1 += 1,
            Quadrant::TopRight => num_q2 += 1,
            Quadrant::BottomLeft => num_q3 += 1,
            Quadrant::BottomRight => num_q4 += 1,
            Quadrant::Middle => (),
        });
    num_q1 * num_q2 * num_q3 * num_q4
}

fn part1(input: &str) -> i32 {
    let mut robots = read_input(input);
    for robot in robots.iter_mut() {
        robot.position = move_robot(robot, SECONDS);
    }
    calculate_safety_factor(&robots)
}

const MAGIC_NUMBER: i32 = 10000;

fn part2(input: &str) -> i32 {
    let mut robots = read_input(input);
    let mut min_safety_factor = i32::MAX;
    let mut min_seconds = 0;
    for i in 0..MAGIC_NUMBER {
        for robot in robots.iter_mut() {
            robot.position = move_robot(robot, 1);
        }

        let safety_factor = calculate_safety_factor(&robots);
        if safety_factor < min_safety_factor {
            min_safety_factor = safety_factor;
            min_seconds = i + 1;
        }
    }
    min_seconds
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

