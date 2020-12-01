#![allow(dead_code)]

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    part2(input)
}

fn part1(input: Vec<i32>) {
    for x in input.iter() {
        for y in input.iter() {
            if x + y == 2020 {
                println!("{}", x * y);
                return;
            }
        }
    }
}

fn part2(input: Vec<i32>) {
    for x in input.iter() {
        for y in input.iter() {
            for z in input.iter() {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    return;
                }
            }
        }
    }
}
