pub use nom::bytes::complete::tag;
use nom::character::complete::{
    alpha1, char, digit1, multispace0, multispace1, newline, space0, space1,
};
use nom::combinator::{eof, map_res};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

use core::cmp::{max, min};
use core::ops::Range;

use itertools::Itertools;

fn intersect<N>(a: &Range<N>, b: &Range<N>) -> Range<N>
where
    N: Ord + Copy,
{
    max(a.start, b.start)..min(a.end, b.end)
}

fn main() {
    let real_input = std::fs::read_to_string("./input.txt").unwrap();

    let (seeds, almanac) = parse_almanac(&real_input).unwrap().1;

    let locations_pt1 = almanac.location_of_seeds(seeds.clone());
    println!(
        "Pt1 - Lowest location number for a seed to plant: {}",
        locations_pt1.iter().min().unwrap()
    );

    let seeds = reinterpret_seed_list_as_ranges(seeds);
    let locations_pt2 = almanac.range_of_locations_of_seeds(seeds);
    let min_location = locations_pt2.iter().min_by_key(|r| r.start).unwrap().start;
    println!(
        "Pt2 - Lowest location number for a seed to plant: {}",
        min_location
    );
}

fn reinterpret_seed_list_as_ranges(list: Vec<usize>) -> Vec<Range<usize>> {
    list.into_iter()
        .tuple_windows::<(_, _)>()
        .step_by(2)
        .map(|(a, b)| a..a + b)
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq)]
struct MapEntry {
    destination_range_start: usize,
    source_range_start: usize,
    range_size: usize,
}

impl MapEntry {
    fn source_range(&self) -> Range<usize> {
        self.source_range_start..self.source_range_start + self.range_size
    }

    fn remap(&self, n: &usize) -> Option<usize> {
        if !self.source_range().contains(n) {
            return None;
        }

        Some(self.remap_impl(&n))
    }

    fn remap_impl(&self, n: &usize) -> usize {
        if self.destination_range_start > self.source_range_start {
            return n + (self.destination_range_start - self.source_range_start);
        } else {
            return n - (self.source_range_start - self.destination_range_start);
        }
    }

    fn remap_range(&self, r: Range<usize>) -> Option<(Range<usize>, Vec<Range<usize>>)> {
        let remapped = intersect(&r, &self.source_range());
        if remapped.is_empty() {
            return None;
        }
        let remapped = self.remap_impl(&remapped.start)..self.remap_impl(&remapped.end);
        let mut rest = vec![];

        let ensure_not_empty = |r: Range<usize>| -> Option<Range<usize>> {
            if r.is_empty() {
                None
            } else {
                Some(r)
            }
        };

        if let Some(head) = ensure_not_empty(
            min(r.start, self.source_range_start)..min(r.end, self.source_range_start),
        ) {
            rest.push(head);
        }
        if let Some(tail) =
            ensure_not_empty(max(self.source_range_start + self.range_size, r.start)..r.end)
        {
            rest.push(tail);
        }
        Some((remapped, rest))
    }
}

#[derive(Debug, PartialEq)]
struct Map<'a> {
    source_name: &'a str,
    destination_name: &'a str,
    entries: Vec<MapEntry>,
}

impl<'a> Map<'a> {
    fn remap(&self, n: &usize) -> usize {
        self.entries.iter().find_map(|e| e.remap(n)).unwrap_or(*n)
    }

    fn remap_range(&self, r: Range<usize>) -> Vec<Range<usize>> {
        let mut to_process = vec![r];
        let mut processed = vec![];
        while let Some(r) = to_process.pop() {
            if let Some((remapped, rest)) =
                self.entries.iter().find_map(|e| e.remap_range(r.clone()))
            {
                processed.push(remapped);
                to_process.extend(rest);
            } else {
                processed.push(r);
            }
        }
        processed
    }
}

#[derive(PartialEq, Debug)]
struct Almanac<'a>(Vec<Map<'a>>);

impl<'a> Almanac<'a> {
    fn location_of_seeds(&self, seeds: Vec<usize>) -> Vec<usize> {
        self.0.iter().fold(seeds, |acc, map| {
            acc.into_iter().map(|n| map.remap(&n)).collect()
        })
    }

    fn range_of_locations_of_seeds(&self, seeds: Vec<Range<usize>>) -> Vec<Range<usize>> {
        self.0.iter().fold(seeds, |acc, map| {
            acc.into_iter().flat_map(|n| map.remap_range(n)).collect()
        })
    }
}

impl<'a> From<Vec<Map<'a>>> for Almanac<'a> {
    fn from(v: Vec<Map<'a>>) -> Self {
        Self(v)
    }
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
            terminated(map_res(digit1, str::parse::<usize>), space1),
            terminated(map_res(digit1, str::parse::<usize>), space1),
            terminated(map_res(digit1, str::parse::<usize>), space0),
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

fn parse_seeds_to_plant(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("seeds:"),
        many1(delimited(
            space0,
            map_res(digit1, str::parse::<usize>),
            space0,
        )),
    )(input)
}

fn parse_almanac(input: &str) -> IResult<&str, (Vec<usize>, Almanac)> {
    separated_pair(
        parse_seeds_to_plant,
        newline,
        terminated(
            separated_list1(multispace1, parse_map),
            tuple((multispace0, eof)),
        ),
    )(input)
    .map(|(rest, (seeds, maps))| (rest, (seeds, maps.into())))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
                    .into()
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

        let (seeds, almanac) = parse_almanac(example_almanac).unwrap().1;

        let locations_pt1 = almanac.location_of_seeds(seeds.clone());
        assert_eq!(locations_pt1, vec![82, 43, 86, 35]);

        let seeds_pt2 = reinterpret_seed_list_as_ranges(seeds);
        assert_eq!(seeds_pt2.into_iter().flatten().count(), 27);
    }

    #[test]
    fn remap_range() {
        let entry = MapEntry {
            source_range_start: 100,
            destination_range_start: 1100,
            range_size: 100,
        };

        assert_eq!(entry.remap_range(0..10), None);
        assert_eq!(
            entry.remap_range(0..150),
            Some((1100..1150, vec![(0..100)]))
        );
        assert_eq!(entry.remap_range(150..160), Some((1150..1160, vec![])));
        assert_eq!(
            entry.remap_range(150..250),
            Some((1150..1200, vec![200..250]))
        );

        let map = Map {
            source_name: "",
            destination_name: "",
            entries: vec![
                MapEntry {
                    source_range_start: 100,
                    destination_range_start: 1100,
                    range_size: 100,
                },
                MapEntry {
                    source_range_start: 300,
                    destination_range_start: 100,
                    range_size: 10,
                },
            ],
        };
        assert_eq!(
            map.remap_range(0..1000).iter().collect::<HashSet<_>>(),
            vec![0..100, 1100..1200, 200..300, 100..110, 310..1000]
                .iter()
                .collect::<HashSet<_>>()
        );
    }
}
