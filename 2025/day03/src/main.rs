use std::io::BufRead;

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let file = std::fs::File::open("./input.txt")?;
    let lines = std::io::BufReader::new(file).lines();

    let lines: Vec<_> = lines.filter_map(|line| line.ok()).collect();

    let n = lines.first().unwrap().len();

    let mut pt1: u32 = 0;
    let mut pt1_alt: u32 = 0;
    let mut pt2: u64 = 0;
    for line in lines {
        let sline = &line[..n - 1];

        let first = sline.chars().max().unwrap();
        let first_index = sline.find(first).unwrap();

        let eline = &line[first_index + 1..];
        let second = eline.chars().max().unwrap();

        let num = 10 * (first as u8 - b'0') + (second as u8 - b'0');

        pt1 += num as u32;

        // ------ reimplemented pt1 in terms of more generic solution, written for pt2
        pt1_alt += full_chop(&line, 2) as u32;
        pt2 += full_chop(&line, 12);
    }

    println!("pt1: {}", pt1);
    println!("pt1_alt: {}", pt1_alt);
    println!("pt2: {}", pt2);

    Ok(())
}

fn chop(line: &str, i: u8) -> (u8, &str) {
    let n = line.len();
    let slice = &line[..n - (i - 1) as usize];
    let first = slice.chars().max().unwrap();
    let first_index = slice.find(first).unwrap();

    (first as u8 - b'0', &line[first_index + 1..n])
}

fn full_chop(line: &str, order: u8) -> u64 {
    let mut acc: u64 = 0;
    let mut remaining = line;
    for i in 0..order {
        let (n, rest) = chop(remaining, order - i);
        acc += n as u64 * 10u64.pow((order - i - 1) as u32);
        remaining = rest;
    }
    acc
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn chop_test() {
        assert_eq!(chop("987654321111111", 12), (9, "87654321111111"));
        assert_eq!(chop("87654321111111", 11), (8, "7654321111111"));
    }
    #[test]
    fn full_chop_test() {
        assert_eq!(full_chop("9", 1), 9);
        assert_eq!(full_chop("97", 2), 97);
        assert_eq!(full_chop("979", 2), 99);
    
        assert_eq!(full_chop("987654321111111", 12), 987654321111);
        assert_eq!(full_chop("234234234234278", 12), 434234234278);
    }
}
