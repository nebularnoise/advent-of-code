use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let (mut left, mut right): (Vec<_>, Vec<_>) = lines
        .filter_map(|line| {
            line.ok().and_then(|l| {
                l.split_once(' ').map(|(a, b)| {
                    (
                        a.trim().parse::<u32>().unwrap(),
                        b.trim().parse::<u32>().unwrap(),
                    )
                })
            })
        })
        .unzip();

    left.sort();
    right.sort();

    let pt1: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b) as u64)
        .sum();
    println!("pt1 {}", pt1);

    let pt2: usize = left
        .iter()
        .map(|n| (*n as usize) * weight(*n, &right))
        .sum();
    println!("pt2 {}", pt2);

    Ok(())
}

fn weight(n: u32, s: &[u32]) -> usize {
    let low = s.partition_point(|x| x < &n);
    let high = s.partition_point(|x| x <= &n);
    return high - low;
}
