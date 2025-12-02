fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: String = std::fs::read_to_string("./input.txt")?;
    let ranges = input.split(',');

    let mut acc_pt1: u128 = 0;
    let mut acc_pt2: u128 = 0;
    for range in ranges {
        let (start, end) = range.split_at(range.find('-').unwrap());
        let end = &end[1..];

        let start: u64 = start.parse()?;
        let end: u64 = end.parse()?;

        for i in start..=end {
            if is_invalid_pt1(i) {
                acc_pt1 += i as u128;
            }
            if is_invalid_pt2(i) {
                acc_pt2 += i as u128;
            }
        }
    }
    println!("pt1 {}", acc_pt1);
    println!("pt2 {}", acc_pt2);

    Ok(())
}

fn base_10_len(n: u64) -> u8 {
    match n {
        0 => 1,
        _ => n.ilog10() as u8 + 1,
    }
}

// fn bisect(n: u64) -> Option<(u64, u64)> {
//     let len = base_10_len(n);
//     if (len % 2) != 0 {
//         return None;
//     }
//     let power_of_ten = 10u64.pow(len as u32 / 2);

//     let top_n = n / power_of_ten;
//     let bottom_n = n - (power_of_ten * top_n);

//     Some((top_n, bottom_n))
// }

fn is_invalid_pt1(n: u64) -> bool {
    is_invalid_k(n,2)
    // match bisect(n) {
    //     Some((a, b)) => a == b,
    //     None => false,
    // }
}

fn is_invalid_pt2(n: u64) -> bool {
    let len = base_10_len(n);
    for order in 2..=len {
        if is_invalid_k(n, order) {
            return true;
        }
    }
    return false;
}

fn is_invalid_k(n: u64, order: u8) -> bool {
    let len = base_10_len(n);

    if (len % order) != 0 {
        return false;
    }
    let power_of_ten = 10u64.pow(len as u32 / order as u32);

    let motif = n - (n / power_of_ten) * power_of_ten;
    let mut rest = n;
    for _i in 0..order {
        let bottom = rest - (rest / power_of_ten) * power_of_ten;
        rest /= power_of_ten;
        if bottom != motif {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn base_10_len_test() {
        for i in 0..1024 {
            assert_eq!(base_10_len(i), i.to_string().len() as u8);
        }
    }

    // #[test]
    // fn bisect_test() {
    //     assert_eq!(bisect(0), None);
    //     assert_eq!(bisect(10), Some((1, 0)));
    //     assert_eq!(bisect(1234), Some((12, 34)));
    // }
}
