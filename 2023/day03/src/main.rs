use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::branch::alt;
pub use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::map_res;
use nom::multi::{many0, many1_count};
use nom::IResult;
use std::collections::HashMap;

const MAX_XY: usize = 139;

type NumberStore = HashMap<XBoundingBox, usize>;

fn main() {
    let lines = read_lines("./input.txt").unwrap();

    let mut numbers: NumberStore = NumberStore::new();
    let mut symbols: Vec<(Point, char)> = Vec::new();

    for (y, line) in lines.enumerate() {
        for entity in process_schematic_line(y, parse_line(&line.unwrap()).unwrap().1) {
            match entity {
                SchematicEntity::Number {
                    value,
                    bounding_box,
                } => {
                    numbers.insert(bounding_box, value);
                }
                SchematicEntity::Symbol(p, c) => symbols.push((p, c)),
            }
        }
    }

    let mut numbers_to_add_up: NumberStore = NumberStore::new();
    let mut gear_ratios_sum = 0;
    for p in symbols {
        let mut adjacent_numbers: NumberStore = NumberStore::new();
        for neigh in p.0.neighbours() {
            for (bb, val) in &numbers {
                if bb.contains(&neigh) {
                    adjacent_numbers.insert(bb.clone(), *val);
                }
            }
        }
        if p.1 == '*' && adjacent_numbers.len() == 2 {
            let gear_ratio = adjacent_numbers.iter().fold(1, |acc, el| acc * el.1);
            gear_ratios_sum += gear_ratio;
        }
        numbers_to_add_up.extend(adjacent_numbers.drain());
    }

    let sum: usize = numbers_to_add_up.iter().map(|(_k, v)| v).sum();
    println!("Pt1: sum of all numbers neighbouring symbols : {}", sum);
    println!("Pt2: sum of all gear ratios : {}", gear_ratios_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        let mut res = vec![];

        // line above
        if self.y > 0 {
            if self.x > 0 {
                // UL
                res.push(Point {
                    y: self.y - 1,
                    x: self.x - 1,
                });
            }
            res.push(Point {
                // U
                y: self.y - 1,
                x: self.x,
            });
            if self.x < MAX_XY {
                // UR
                res.push(Point {
                    y: self.y - 1,
                    x: self.x + 1,
                });
            }
        }
        // L
        if self.x > 0 {
            res.push(Point {
                y: self.y,
                x: self.x - 1,
            });
        }

        // R
        if self.x < MAX_XY {
            res.push(Point {
                y: self.y,
                x: self.x + 1,
            });
        }

        // line below
        if self.y < MAX_XY {
            if self.x > 0 {
                // DL
                res.push(Point {
                    y: self.y + 1,
                    x: self.x - 1,
                });
            }
            res.push(Point {
                // D
                y: self.y + 1,
                x: self.x,
            });
            if self.x < MAX_XY {
                // DR
                res.push(Point {
                    y: self.y + 1,
                    x: self.x + 1,
                });
            }
        }

        res
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct XBoundingBox {
    y: usize,
    x: (usize, usize),
}

impl XBoundingBox {
    fn contains(&self, p: &Point) -> bool {
        self.y == p.y && p.x >= self.x.0 && p.x <= self.x.1
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SchematicEntity {
    Number {
        value: usize,
        bounding_box: XBoundingBox,
    },
    Symbol(Point, char),
}

#[derive(Debug, PartialEq, Eq)]
enum LineEntity {
    Spacing(usize),
    Number(usize),
    Symbol(char),
}

fn parse_spacing(input: &str) -> IResult<&str, LineEntity> {
    many1_count(tag("."))(input).map(|(rest, n)| (rest, LineEntity::Spacing(n)))
}
fn parse_number(input: &str) -> IResult<&str, LineEntity> {
    map_res(digit1, str::parse::<usize>)(input).map(|(rest, n)| (rest, LineEntity::Number(n)))
}
fn parse_symbol(input: &str) -> IResult<&str, LineEntity> {
    anychar(input).map(|(rest, c)| (rest, LineEntity::Symbol(c)))
}

fn parse_line(input: &str) -> IResult<&str, Vec<LineEntity>> {
    many0(alt((parse_spacing, parse_number, parse_symbol)))(input)
}

fn process_schematic_line(line_y: usize, parsed_line: Vec<LineEntity>) -> Vec<SchematicEntity> {
    let mut running_x: usize = 0;
    let mut entities: Vec<SchematicEntity> = vec![];

    for le in parsed_line {
        match le {
            LineEntity::Number(n) => {
                let width = n.to_string().len();
                entities.push(SchematicEntity::Number {
                    value: n,
                    bounding_box: XBoundingBox {
                        y: line_y,
                        x: (running_x, running_x + width - 1),
                    },
                });
                running_x += width;
            }
            LineEntity::Spacing(n) => running_x += n,
            LineEntity::Symbol(c) => {
                entities.push(SchematicEntity::Symbol(
                    Point {
                        y: line_y,
                        x: running_x,
                    },
                    c,
                ));
                running_x += 1;
            }
        }
    }

    entities
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn bounding_box() {
        assert!(XBoundingBox { y: 2, x: (0, 10) }.contains(&Point { y: 2, x: 3 }));
        assert!(XBoundingBox { y: 2, x: (0, 10) }.contains(&Point { y: 2, x: 10 }));
        assert!(!XBoundingBox { y: 2, x: (0, 10) }.contains(&Point { y: 2, x: 11 }));
        assert!(!XBoundingBox { y: 2, x: (0, 10) }.contains(&Point { y: 0, x: 11 }));
    }

    #[test]
    fn spaceparser() {
        assert_eq!(parse_spacing("."), Ok(("", LineEntity::Spacing(1))));
        assert_eq!(parse_spacing("..."), Ok(("", LineEntity::Spacing(3))));
    }

    #[test]
    fn parse_entity() {
        assert_eq!(parse_line("."), Ok(("", vec![LineEntity::Spacing(1)])));
        assert_eq!(
            parse_line(".0..10....$"),
            Ok((
                "",
                vec![
                    LineEntity::Spacing(1),
                    LineEntity::Number(0),
                    LineEntity::Spacing(2),
                    LineEntity::Number(10),
                    LineEntity::Spacing(4),
                    LineEntity::Symbol('$')
                ]
            ))
        );
    }

    #[test]
    fn processscl() {
        assert_eq!(
            process_schematic_line(0, parse_line(".0..10....$").unwrap().1),
            vec![
                SchematicEntity::Number {
                    value: 0,
                    bounding_box: XBoundingBox { y: 0, x: (1, 1) }
                },
                SchematicEntity::Number {
                    value: 10,
                    bounding_box: XBoundingBox { y: 0, x: (4, 5) }
                },
                SchematicEntity::Symbol(Point { y: 0, x: 10 }, '$')
            ]
        );
    }
}
