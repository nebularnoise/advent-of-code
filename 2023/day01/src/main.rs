use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let sum: u32 = lines.map(|line| line.map_or(0, |l| parse_line(&l))).sum();
        println!("{}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> u32 {
    let f = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    let l = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    f * 10 + l
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_line_parser() {
        assert_eq!(parse_line("1abc2"), 12);
        assert_eq!(parse_line("pqr3stu8vwx"), 38);
        assert_eq!(parse_line("a1b2c3d4e5f"), 15);
        assert_eq!(parse_line("treb7uchet"), 77);
    }
}
