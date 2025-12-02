use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let commands: Vec<_> = lines
            .map(|line| line.map_or(0, |l| parse_line(&l)))
            .collect();
        const DIAL_INIT: u8 = 50;
        let mut dial = DIAL_INIT;
        let mut zero_crossings_pt2: u16 = 0;
        let mut zeros_pt1 = 0;
        for com in commands {
            let (new_dial, zc) = rot(dial, com);
            // println!("{} + {} -> ({}, {})", dial, com, new_dial, zc);
            dial = new_dial;
            zero_crossings_pt2 += zc as u16;
            if dial == 0 {
                zeros_pt1 += 1;
            }
        }
        println!("pt1: {:?}", zeros_pt1);
        println!("pt2: {:?}", zero_crossings_pt2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> i16 {
    let mult = match line.as_bytes()[0] {
        b'L' => -1,
        b'R' => 1,
        _ => 0,
    };

    let num = &line[1..];
    let num = num.parse::<u16>().unwrap();
    mult * (num as i16)
}

fn rot(dial: u8, l_r: i16) -> (u8, u8) {
    if l_r == 0 {
        return (dial, 0);
    }
    let mut new_dial = dial as i16;
    let mut zero_crossings: u8 = 0;
    zero_crossings += (l_r.abs() / 100) as u8;

    new_dial += l_r % 100;

    if new_dial < 0 {
        new_dial += 100;
        if dial != 0 {
            zero_crossings += 1;
        }
    } else if new_dial > 99 {
        new_dial -= 100;
        if dial != 0 {
            zero_crossings += 1;
        }
    } else if new_dial == 0 {
        zero_crossings += 1;
    }
    (new_dial.try_into().unwrap(), zero_crossings)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rot_wrap() {
        assert_eq!(rot(1, -2).0, 99);
        assert_eq!(rot(99, 2).0, 1);
        assert_eq!(rot(50, -100).0, 50);
        assert_eq!(rot(50, 100).0, 50);
    }
    #[test]
    fn rot_zero_crossings() {
        assert_eq!(rot(1, -2).1, 1);
        assert_eq!(rot(99, 2).1, 1);
        assert_eq!(rot(50, -100).1, 1);
        assert_eq!(rot(50, 100).1, 1);
        assert_eq!(rot(50, 1000).1, 10);
        assert_eq!(rot(50, -1000).1, 10);
        assert_eq!(rot(50, 50).1, 1);
    }

    #[test]
    fn parse_ok() {
        assert_eq!(parse_line("R99"), 99);
        assert_eq!(parse_line("L99"), -99);
        assert_eq!(parse_line("R999"), 999);
    }
}
