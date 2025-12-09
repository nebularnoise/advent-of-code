use std::io::BufRead;

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let file = std::fs::File::open("./input.txt")?;
    let lines = std::io::BufReader::new(file).lines();

    let lines: Vec<_> = lines.map_while(Result::ok).collect();

    let tokens_n = lines.len();

    println!("tokens {:?}", tokens_n);

    let lines_pt1: Vec<_> = lines
        .iter()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
        .collect();

    println!("lpt1 {:?}", lines_pt1);

    let operations_n = lines_pt1.first().unwrap().len();
    println!("opn {:?}", operations_n);

    let mut tr_lines: Vec<Vec<&str>> = vec![Vec::new(); operations_n];

    for l in lines_pt1 {
        for (i, s) in l.iter().enumerate() {
            tr_lines[i].push(*s);
        }
    }

    let pt1: u64 = tr_lines
        .iter()
        .map(|v| {
            let operands: Vec<_> = v[..tokens_n - 1]
                .iter()
                .map(|&s| s.parse::<u64>().unwrap())
                .collect();
            match *v.last().unwrap() {
                "*" => operands.iter().product(),
                "+" => operands.iter().sum(),
                _ => 0,
            }
        })
        .sum();

    println!("pt1: {}", pt1);


    Ok(())
}
