use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use array_tool::vec::*;
pub use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

use std::ops::Range;

fn main() {
    let lines = read_lines("./input.txt").unwrap();

    let cards: Vec<Card> = lines
        .map(|l| l.map(|line| parse_line(&line).unwrap().1).unwrap())
        .collect();

    let winning_cards: SetOfCards = cards.into();
    let points: u32 = winning_cards.points();
    println!("Pt1 - Sum of all points : {}", points);

    let winnings = full_winnings_of_set(&winning_cards);
    println!("Pt2 - Final amount of scratch cards after winnings : {}", winnings);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_card_number(input: &str) -> IResult<&str, u16> {
    delimited(
        tuple((tag("Card"), space1)),
        map_res(digit1, str::parse::<u16>),
        tuple((space0, tag(":"))),
    )(input)
}

fn parse_number_list(input: &str) -> IResult<&str, Vec<u8>> {
    delimited(
        space0,
        separated_list1(space1, map_res(digit1, str::parse::<u8>)),
        space0,
    )(input)
}

#[derive(Debug)]
struct SetOfCards {
    winning_cards: HashMap<u16, Prize>,
}

impl SetOfCards {
    fn points(&self) -> u32 {
        self.winning_cards
            .iter()
            .map(|(_n, p)| p.0.clone().count() as u32)
            .sum()
    }
}

fn full_winnings_of_single_card(set: &SetOfCards, cache: &mut HashMap<u16, u32>, n: u16) -> u32 {
    match cache.get(&n).map(|entry| entry.clone()) {
        Some(val) => val,
        None => {
            if let Some(prize) = set.winning_cards.get(&n) {
                let winnings = prize
                    .0
                    .clone()
                    .map(|k| 1 + full_winnings_of_single_card(set, cache, k))
                    .sum();
                cache.insert(n, winnings);
                return winnings;
            } else {
                cache.insert(n, 0);
                return 0;
            }
        }
    }
}

fn full_winnings_of_set(set: &SetOfCards) -> u32 {
    let mut cache: HashMap<u16, u32> = HashMap::new();
    let full_wins = set
        .winning_cards
        .iter()
        .fold((&mut cache, 0 as u32), |(c, winnings), card| {
            let wins = full_winnings_of_single_card(set, c, *card.0);
            (c, winnings + wins)
        })
        .1;
    full_wins
}

impl From<Vec<Card>> for SetOfCards {
    fn from(cards: Vec<Card>) -> Self {
        SetOfCards {
            winning_cards: cards
                .into_iter()
                .map(|c| (c.number, c.prize()))
                .filter(|(_n, p)| !p.0.is_empty())
                .collect(),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Card {
    number: u16,
    winning_numbers: Vec<u8>,
    numbers_you_have: Vec<u8>,
}

#[derive(Debug)]
struct Prize(Range<u16>);

impl From<(u16, (Vec<u8>, Vec<u8>))> for Card {
    fn from(item: (u16, (Vec<u8>, Vec<u8>))) -> Self {
        Card {
            number: item.0,
            winning_numbers: item.1 .0,
            numbers_you_have: item.1 .1,
        }
    }
}
impl Card {
    fn points(&self) -> u16 {
        let matching_numbers = self
            .winning_numbers
            .intersect(self.numbers_you_have.clone())
            .len();
        if matching_numbers == 0 {
            return 0;
        }
        1 << (matching_numbers - 1)
    }

    fn prize(&self) -> Prize {
        Prize(self.number..(self.number + self.points()))
    }
}

fn parse_line(input: &str) -> IResult<&str, Card> {
    tuple((
        parse_card_number,
        separated_pair(
            parse_number_list,
            tuple((space0, tag("|"), space0)),
            parse_number_list,
        ),
    ))(input)
    .map(|(rest, tup)| (rest, tup.into()))
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_line_parser() {
        assert_eq!(
            parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Ok((
                "",
                (
                    1,
                    (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
                )
                    .into()
            ))
        );
    }
}
