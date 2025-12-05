use std::{io::BufRead, vec::Vec};

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let file = std::fs::File::open("./input.txt")?;
    let lines = std::io::BufReader::new(file).lines();

    let lines: Vec<_> = lines.filter_map(|line| line.ok()).collect();

    let ranges: Vec<_> = lines
        .iter()
        .take_while(|line| !line.trim().is_empty())
        .map(|r_str| r_str.split_once('-').unwrap())
        .map(|(s, e)| s.parse::<u64>().unwrap()..=e.parse::<u64>().unwrap())
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

    Ok(())
}
