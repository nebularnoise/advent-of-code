use std::{io::BufRead, vec::Vec};

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let file = std::fs::File::open("./input.txt")?;
    let lines = std::io::BufReader::new(file).lines();

    let lines: Vec<_> = lines.map_while(Result::ok).collect();

    let ranges: Vec<_> = lines
        .iter()
        .take_while(|line| !line.trim().is_empty())
        .map(|r_str| r_str.split_once('-').unwrap())
        .map(|(s, e)| s.parse::<u64>().unwrap()..(e.parse::<u64>().unwrap() + 1))
        .collect();

    let ids: Vec<_> = lines
        .iter()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .map(|i_str| i_str.parse::<u64>().unwrap())
        .collect();

    let mut fresh_count: usize = 0;

    for id in ids {
        if ranges.iter().any(|r| r.contains(&id)) {
            fresh_count += 1;
        }
    }

    println!("pt1 = {:?}", fresh_count);

    //=================

    let mut ranges = ranges;
    ranges.sort_by_key(|r| r.start);

    let mut non_overlapping_ranges: Vec<std::ops::Range<u64>> = vec![];

    for r in ranges {
        match non_overlapping_ranges.last() {
            Some(nor) if nor.end >= r.start => {
                if r.end > nor.end {
                    non_overlapping_ranges.push(nor.end..r.end);
                }
            }
            _ => {
                non_overlapping_ranges.push(r);
            }
        }
    }

    let pt2: usize = non_overlapping_ranges.into_iter().map(|r| r.count()).sum();

    println!("pt2: {}", pt2);

    Ok(())
}
