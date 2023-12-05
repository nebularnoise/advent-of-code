pub use nom::bytes::complete::tag;
use nom::character::complete::{
    alpha1, char, digit1, multispace0, multispace1, newline, space0, space1,
};
use nom::combinator::{eof, map_res};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

fn main() {
    let real_input = std::fs::read_to_string("./input.txt").unwrap();

    let almanac = parse_almanac(&real_input).unwrap().1;
    for map in almanac.1 {
        println!("{}\t->{}", map.source_name, map.destination_name);
    }
}

#[derive(Debug, PartialEq)]
struct MapEntry {
    destination_range_start: u32,
    source_range_start: u32,
    range_size: u32,
}

#[derive(Debug, PartialEq)]
struct Map<'a> {
    source_name: &'a str,
    destination_name: &'a str,
    entries: Vec<MapEntry>,
}

fn parse_map_header(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(
        separated_pair(alpha1, tag("-to-"), alpha1),
        tuple((space1, tag("map"), space0, char(':'))),
    )(input)
}

fn parse_map_entry(input: &str) -> IResult<&str, MapEntry> {
    delimited(
        space0,
        tuple((
            terminated(map_res(digit1, str::parse::<u32>), space1),
            terminated(map_res(digit1, str::parse::<u32>), space1),
            terminated(map_res(digit1, str::parse::<u32>), space0),
        )),
        space0,
    )(input)
    .map(|(rest, (a, b, c))| {
        (
            rest,
            MapEntry {
                destination_range_start: a,
                source_range_start: b,
                range_size: c,
            },
        )
    })
}

fn parse_map<'a>(input: &'a str) -> IResult<&str, Map<'a>> {
    tuple((
        delimited(multispace0, parse_map_header, newline),
        separated_list1(newline, parse_map_entry),
    ))(input)
    .map(|(rest, ((s, d), entries))| {
        (
            rest,
            Map {
                source_name: s,
                destination_name: d,
                entries: entries,
            },
        )
    })
}

fn parse_seeds_to_plant(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        tag("seeds:"),
        many1(delimited(
            space0,
            map_res(digit1, str::parse::<u32>),
            space0,
        )),
    )(input)
}

fn parse_almanac(input: &str) -> IResult<&str, (Vec<u32>, Vec<Map>)> {
    separated_pair(
        parse_seeds_to_plant,
        newline,
        terminated(
            separated_list1(multispace1, parse_map),
            tuple((multispace0, eof)),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_map_hearder_parser() {
        assert_eq!(
            parse_map_header("seed-to-soil map:"),
            Ok(("", ("seed", "soil")))
        );
    }

    #[test]
    fn test_map_entry_parser() {
        assert_eq!(
            parse_map_entry("50 98 2"),
            Ok((
                "",
                MapEntry {
                    destination_range_start: 50,
                    source_range_start: 98,
                    range_size: 2,
                }
            ))
        );
    }

    #[test]
    fn test_map_parser() {
        assert_eq!(
            parse_map(
                "seed-to-soil map:
            50 98 2
            52 50 48"
            ),
            Ok((
                "",
                Map {
                    source_name: "seed",
                    destination_name: "soil",
                    entries: vec![
                        MapEntry {
                            destination_range_start: 50,
                            source_range_start: 98,
                            range_size: 2,
                        },
                        MapEntry {
                            destination_range_start: 52,
                            source_range_start: 50,
                            range_size: 48,
                        }
                    ],
                }
            ))
        );
    }
    #[test]
    fn test_parse_seeds_to_plant() {
        assert_eq!(
            parse_seeds_to_plant("seeds: 79 14 55 13"),
            Ok(("", vec![79, 14, 55, 13]))
        );
    }

    #[test]
    fn test_parse_full_problem() {
        assert_eq!(
            parse_almanac(
                "seeds: 79 14 55 13
            
                seed-to-soil map:
                50 98 2
                52 50 48

                seed-to-soil map:
            50 98 2
            52 50 48"
            ),
            Ok((
                "",
                (
                    vec![79, 14, 55, 13],
                    vec![
                        Map {
                            source_name: "seed",
                            destination_name: "soil",
                            entries: vec![
                                MapEntry {
                                    destination_range_start: 50,
                                    source_range_start: 98,
                                    range_size: 2,
                                },
                                MapEntry {
                                    destination_range_start: 52,
                                    source_range_start: 50,
                                    range_size: 48,
                                }
                            ],
                        },
                        Map {
                            source_name: "seed",
                            destination_name: "soil",
                            entries: vec![
                                MapEntry {
                                    destination_range_start: 50,
                                    source_range_start: 98,
                                    range_size: 2,
                                },
                                MapEntry {
                                    destination_range_start: 52,
                                    source_range_start: 50,
                                    range_size: 48,
                                }
                            ],
                        }
                    ]
                )
            ))
        );
    }

    #[test]
    fn solves_example() {
        let example_almanac = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        println!("{:?}", parse_almanac(example_almanac));
    }
}
