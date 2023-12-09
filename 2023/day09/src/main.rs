use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use itertools::Itertools;
fn main() {
    let lines = read_lines("input.txt").unwrap();
    let sum = lines.map(|l| {
        let input = l
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>();
        return extrapolate(input);
    }).sum::<i64>();
    println!("Pt1: {}", sum);

    let lines = read_lines("input.txt").unwrap();
    let sum = lines.map(|l| {
        let input = l
            .unwrap()
            .split_whitespace()
            .rev()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>();
        return extrapolate(input);
    }).sum::<i64>();
    println!("Pt2: {}", sum);
}

fn adjacent_difference(input: Vec<i64>) -> Vec<i64> {
    input
        .into_iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| b - a)
        .collect()
}

fn extrapolate(input: Vec<i64>) -> i64 {
    let mut input = input;
    let mut tails = vec![];
    while let Some(tail) = input.last().copied() {
        tails.push(tail);
        input = adjacent_difference(input);
    }
    tails.into_iter().sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn adj_diff() {
        assert_eq!(adjacent_difference(vec![1, 2, 3]), vec![1, 1]);
        assert_eq!(adjacent_difference(vec![1]), vec![]);
    }

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(vec![1, 2, 3]), 4);

        assert_eq!(extrapolate(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(vec![10, 13, 16, 21, 30, 45]), 68);
    }
}
