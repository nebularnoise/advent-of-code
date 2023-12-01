use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let sum: usize = lines
            .map(|line| {
                line.map_or(0, |l| {
                    let number = parse_line(&l);
                    number
                })
            })
            .sum();
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

const DIGITS_SPELLED_OUT: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_digits_in(line: &str) -> Vec<(usize, usize)> {
    let digits = DIGITS_SPELLED_OUT.iter().enumerate().skip(1);
    let indices_of_spelled_out_digits: Vec<(usize, usize)> = digits
        .flat_map(|(digit, spelling)| {
            line.match_indices(spelling)
                .map(move |(idx, _)| (idx, digit))
        })
        .collect();

    let indices_of_normal_digits: Vec<(usize, usize)> = (1..=9)
        .filter_map(|digit| line.find(&digit.to_string()).map(|index| (index, digit)))
        .collect();

    let mut all_digits_with_indices =
        [indices_of_spelled_out_digits, indices_of_normal_digits].concat();
    all_digits_with_indices.sort_by_key(|(index, _digit)| *index);
    all_digits_with_indices
}

fn parse_line(line: &str) -> usize {
    let digits_in_string = find_digits_in(line);
    let f = digits_in_string.first().unwrap().1;
    let l = digits_in_string.last().unwrap().1;
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
        assert_eq!(parse_line("two1nine"), 29);
        assert_eq!(parse_line("eightwothree"), 83);
        assert_eq!(parse_line("abcone2threexyz"), 13);
        assert_eq!(parse_line("xtwone3four"), 24);
        assert_eq!(parse_line("4nineeightseven2"), 42);
        assert_eq!(parse_line("zoneight234"), 14);
        assert_eq!(parse_line("7pqrstsixteen"), 76);
        assert_eq!(parse_line("oneight"), 18);
    }
}
