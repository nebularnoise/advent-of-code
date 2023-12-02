use nom::branch::alt;
pub use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    if let Ok(lines) = read_lines("./input.txt") {
        let (sum_of_ids, sum_of_powers): (usize, usize) = lines
            .map(|line| {
                line.map_or(
                    //
                    (0, 0),
                    |l| {
                        parse_game(&l).map_or((0, 0), |(_, g)| {
                            (
                                if g.possible_given(&bag) { g.id.0 } else { 0 },
                                minimum_bag(&g).power(),
                            )
                        })
                    },
                )
            })
            .fold((0, 0), |acc, tup| (acc.0 + tup.0, acc.1 + tup.1));
        println!("Pt1. Sum of all possible games: {}", sum_of_ids);
        println!("Pt2. Sum of all powers of minimum bags: {}", sum_of_powers);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq)]
struct Handful {
    red: usize,
    green: usize,
    blue: usize,
}
#[derive(Debug, PartialEq)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq)]
struct GameId(usize);

#[derive(Debug, PartialEq)]
struct Game {
    id: GameId,
    handfuls: Vec<Handful>,
}

impl Handful {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn possible_given(&self, bag: &Bag) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    fn add(&mut self, cubes: &BunchOfCubes) {
        match cubes {
            BunchOfCubes::Red(r) => self.red += r,
            BunchOfCubes::Green(g) => self.green += g,
            BunchOfCubes::Blue(b) => self.blue += b,
        }
    }
}

impl From<Vec<BunchOfCubes>> for Handful {
    fn from(v: Vec<BunchOfCubes>) -> Self {
        let mut hf = Handful::new();
        for cubes in v {
            hf.add(&cubes)
        }
        hf
    }
}

impl Game {
    fn possible_given(&self, bag: &Bag) -> bool {
        self.handfuls.iter().all(|h| h.possible_given(bag))
    }
}

impl Bag {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[derive(Debug, PartialEq)]
enum BunchOfCubes {
    Red(usize),
    Green(usize),
    Blue(usize),
}

fn line_header(input: &str) -> IResult<&str, GameId> {
    terminated(
        tuple((tag("Game"), space1, map_res(digit1, str::parse))),
        delimited(space0, tag(":"), space0),
    )(input)
    .map(|(rest, (_, _, i))| (rest, GameId(i)))
}

fn parse_bunch_of_cubes(input: &str) -> IResult<&str, BunchOfCubes> {
    delimited(
        space0,
        tuple((
            map_res(digit1, str::parse::<usize>),
            space1,
            alt((tag("red"), tag("green"), tag("blue"))),
        )),
        space0,
    )(input)
    .map(|(rest, (amount, _, colour))| {
        let cubes = match colour {
            "red" => BunchOfCubes::Red(amount),
            "green" => BunchOfCubes::Green(amount),
            "blue" => BunchOfCubes::Blue(amount),
            _ => BunchOfCubes::Red(0),
        };
        (rest, cubes)
    })
}

fn parse_handful_of_cubes(input: &str) -> IResult<&str, Handful> {
    let (rest, vec) = separated_list1(tag(","), parse_bunch_of_cubes)(input)?;
    Ok((rest, vec.into()))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (rest, game_id) = line_header(input)?;
    let (rest, vec) =
        separated_list1(tag(";"), delimited(space0, parse_handful_of_cubes, space0))(rest)?;
    Ok((
        rest,
        Game {
            id: game_id,
            handfuls: vec,
        },
    ))
}

fn minimum_bag(game: &Game) -> Bag {
    let min_red = game.handfuls.iter().map(|h| h.red).max().unwrap();
    let min_green = game.handfuls.iter().map(|h| h.green).max().unwrap();
    let min_blue = game.handfuls.iter().map(|h| h.blue).max().unwrap();

    Bag {
        red: min_red,
        green: min_green,
        blue: min_blue,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn empty_handful_always_possible() {
        let h0 = Handful {
            red: 0,
            green: 0,
            blue: 0,
        };
        let b = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };
        assert_eq!(h0.possible_given(&b), true);
    }
    #[test]
    fn impossible_handful_given_in_problem_statement() {
        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        let g4_h3 = Handful {
            red: 14,
            green: 3,
            blue: 15,
        };
        assert_eq!(g4_h3.possible_given(&bag), false);
    }

    #[test]
    fn boc_parser() {
        assert_eq!(
            parse_bunch_of_cubes("8 green"),
            Ok(("", BunchOfCubes::Green(8)))
        );
        assert_eq!(
            parse_bunch_of_cubes(" 8 green"),
            Ok(("", BunchOfCubes::Green(8)))
        );
        assert_eq!(
            parse_bunch_of_cubes("8 green     "),
            Ok(("", BunchOfCubes::Green(8)))
        );
        assert_eq!(
            parse_bunch_of_cubes("123 blue"),
            Ok(("", BunchOfCubes::Blue(123)))
        );
    }

    #[test]
    fn hfparser() {
        assert_eq!(
            parse_handful_of_cubes("123 blue"),
            Ok((
                "",
                Handful {
                    red: 0,
                    green: 0,
                    blue: 123
                }
            ))
        );
        assert_eq!(
            parse_handful_of_cubes("8 green, 6 blue, 20 red"),
            Ok((
                "",
                Handful {
                    red: 20,
                    green: 8,
                    blue: 6
                }
            ))
        );
    }

    #[test]
    fn game_parser() {
        assert_eq!(
            parse_game("Game 1 : 8 green, 6 blue, 20 red"),
            Ok((
                "",
                Game {
                    id: GameId(1),
                    handfuls: vec![Handful {
                        red: 20,
                        green: 8,
                        blue: 6
                    }]
                }
            ))
        );

        assert_eq!(
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok((
                "",
                Game {
                    id: GameId(1),
                    handfuls: vec![
                        Handful {
                            red: 4,
                            green: 0,
                            blue: 3
                        },
                        Handful {
                            red: 1,
                            green: 2,
                            blue: 6
                        },
                        Handful {
                            red: 0,
                            green: 2,
                            blue: 0
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn minbag() {
        assert_eq!(
            minimum_bag(&Game {
                id: GameId(1),
                handfuls: vec![
                    Handful {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Handful {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Handful {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ]
            }),
            Bag {
                red: 4,
                green: 2,
                blue: 6
            }
        )
    }
}
