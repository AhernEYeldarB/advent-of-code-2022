use std::cmp::{max, min};

pub fn main() {
    println!(
        "part_one: {:?}",
        include_str!("data.txt")
            .lines()
            .map(|l| l
                .split_once(",")
                .map(|(a, b)| (
                    a.split_once("-")
                        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                        .unwrap(),
                    b.split_once("-")
                        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                        .unwrap()
                ))
                .unwrap())
            .fold(0, |accum, ((a, b), (c, d))| {
                if max(b - a, d - c) == (max(b, d) - min(a, c)) {
                    accum + 1
                } else {
                    accum
                }
            })
    );

    println!(
        "part_two: {:?}",
        include_str!("data.txt")
            .lines()
            .map(|l| l
                .split_once(",")
                .map(|(a, b)| (
                    a.split_once("-")
                        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                        .unwrap(),
                    b.split_once("-")
                        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                        .unwrap()
                ))
                .unwrap())
            .fold(0, |accum, ((a, b), (c, d))| {
                if (max(b,d) - min(a,c)) <= ((b-a) + (d-c)) {
                    accum + 1
                } else {
                    accum
                }
            })
    );
}
