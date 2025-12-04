use std::{io::BufRead, vec};

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let file = std::fs::File::open("./input.txt")?;
    let lines = std::io::BufReader::new(file).lines();

    let lines: Vec<_> = lines.filter_map(|line| line.ok()).collect();

    let w = lines.first().unwrap().len();
    let h = lines.len();
    let mut rolls: Vec<_> = lines
        .iter()
        .flat_map(|l| l.chars().into_iter())
        .map(|c| match c {
            '@' => 1u8,
            _ => 0u8,
        })
        .collect();

    let pt1 = remove_rolls(&mut rolls, w, h).unwrap_or(0);
    let mut pt2 = pt1;
    while let Some(n) = remove_rolls(&mut rolls, w, h) {
        pt2 += n;
    }

    println!("pt1: {:?}", pt1);
    println!("pt2: {:?}", pt2);
    Ok(())
}

fn remove_rolls(rolls: &mut Vec<u8>, w: usize, h: usize) -> Option<usize> {
    let mut acc: usize = 0;
    let neighbour_count = count_neighbours(&rolls, w, h);

    for (r, c) in rolls.iter_mut().zip(neighbour_count) {
        if *r != 1 {
            continue;
        }
        if c >= 4 {
            continue;
        }
        *r = 0;
        acc += 1;
    }

    match acc {
        0 => None,
        _ => Some(acc),
    }
}

fn count_neighbours(rolls: &Vec<u8>, w: usize, h: usize) -> Vec<u8> {
    let mut neighbour_count = vec![0u8; w * h];

    for (i, c) in rolls.iter().enumerate() {
        if *c == 0 {
            continue;
        }

        let x = i % w;
        let y = i / w;

        if x > 0 && y > 0 {
            neighbour_count[w * (y - 1) + (x - 1)] += 1;
        }
        if y > 0 {
            neighbour_count[w * (y - 1) + x] += 1;
        }
        if x < (w - 1) && y > 0 {
            neighbour_count[w * (y - 1) + (x + 1)] += 1;
        }
        if x > 0 {
            neighbour_count[w * (y) + (x - 1)] += 1;
        }
        if x < (w - 1) {
            neighbour_count[w * (y) + (x + 1)] += 1;
        }
        if x > 0 && y < (h - 1) {
            neighbour_count[w * (y + 1) + (x - 1)] += 1;
        }
        if y < (h - 1) {
            neighbour_count[w * (y + 1) + x] += 1;
        }
        if x < (w - 1) && y < (h - 1) {
            neighbour_count[w * (y + 1) + (x + 1)] += 1;
        }
    }
    neighbour_count
}
