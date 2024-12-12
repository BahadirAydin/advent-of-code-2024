use std::fs;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Data {
    id: usize,
    free: i32,
    used: i32,
}

fn parse_data(input: &str) -> Vec<Data> {
    let mut data = Vec::new();
    let mut id = 0;
    let mut trimmed = input.trim().to_owned();
    if trimmed.len() % 2 != 0 {
        trimmed += "0";
    }
    let mut chars = trimmed.chars();
    while let (Some(used), Some(free)) = (chars.next(), chars.next()) {
        data.push(Data {
            id,
            used: used.to_digit(10).unwrap() as i32,
            free: free.to_digit(10).unwrap() as i32,
        });
        id += 1;
    }
    data
}

fn compute_checksum(disk: &[usize]) -> i64 {
    disk.iter()
        .enumerate()
        .map(|(idx, &id)| id as i64 * idx as i64)
        .sum()
}

fn part1(input: &str) -> i64 {
    let mut data = parse_data(input);
    // we know the size of the disk will be the sum of all memory used in part 1
    // so it is more efficient to preallocate the vector
    let mut disk = Vec::with_capacity(data.iter().map(|d| d.used as usize).sum());
    for i in 0..data.len() {
        let initial_allocations = std::iter::repeat(data[i].id).take(data[i].used as usize);
        disk.extend(initial_allocations);
        data[i].used = 0;
        for j in (i..data.len()).rev() {
            let amount = data[i].free.min(data[j].used);
            let allocation = std::iter::repeat(data[j].id).take(amount as usize);
            disk.extend(allocation);
            data[i].free -= amount;
            data[j].used -= amount;
            data[j].free = 0;
            if data[i].free == 0 {
                break;
            }
        }
    }
    compute_checksum(&disk)
}

fn move_data(data: &mut Vec<Data>, idx: usize, location: usize) {
    data[location].free -= data[idx].used;
    if idx != data.len() - 1 {
        data[idx - 1].free += data[idx].free + data[idx].used;
    }
    data[idx].free = data[location].free;
    data[location].free = 0;
    let temp = data.remove(idx);
    data.insert(location + 1, temp);
}

fn compute_checksum_p2(disk: &Vec<Data>) -> i64 {
    let mut checksum = 0;
    let mut multiple = 0;
    for i in 0..disk.len() {
        for _ in 0..disk[i].used {
            checksum += multiple * disk[i].id as i64;
            multiple += 1;
        }
        multiple += disk[i].free as i64;
    }
    checksum
}

fn part2(input: &str) -> i64 {
    let mut data = parse_data(input);
    let mut i = data.len() - 1;
    while i > 0 {
        let elem = data[i];
        if elem.used == 0 {
            continue;
        }
        for j in 0..i {
            if data[j].free >= elem.used {
                move_data(&mut data, i, j);
                i += 1; // adjust the index since we changed the vector order
                break;
            }
        }
        i -= 1;
    }
    compute_checksum_p2(&data)
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
