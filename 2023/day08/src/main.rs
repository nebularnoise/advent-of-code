use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num::integer::lcm;

fn main() {
    let mut lines = read_lines("./input.txt").unwrap();

    let instructions = lines.next().unwrap().unwrap();
    println!("Instructions: {}", instructions);
    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    lines.next();
    while let Some(Ok(line)) = lines.next() {
        let (node_name, lr) = parse_node(&line).unwrap().1;
        graph.insert(node_name, lr);
    }

    let mut current_node = "AAA";
    let mut moves = 0;
    for direction in instructions.chars().cycle() {
        if current_node == "ZZZ" {
            break;
        }
        moves += 1;
        match direction {
            'L' => current_node = &graph[current_node].0,
            'R' => current_node = &graph[current_node].1,
            _ => panic!(),
        }
    }
    println!("Pt1 - Moves: {}", moves);

    let ghost_start_nodes: Vec<_> = graph.keys().filter(|k| k.ends_with('A')).collect();
    let moves_for_each_node = ghost_start_nodes.into_iter().map(|n|
    {
        current_node = n;
        let mut moves: usize = 0;
        for direction in instructions.chars().cycle() {
            if current_node.ends_with('Z') {
                break;
            }
            moves += 1;
            match direction {
                'L' => current_node = &graph[current_node].0,
                'R' => current_node = &graph[current_node].1,
                _ => panic!(),
            }
        }
        moves
    }).collect::<Vec<_>>();

    let moves = moves_for_each_node.into_iter().fold(1, |acc, n| lcm(acc, n));

    println!("Pt2 - Moves: {}", moves);
}

fn parse_node(input: &str) -> IResult<&str, (String, (String, String))> {
    separated_pair(
        alpha1,
        tuple((multispace0, tag("="), multispace0)),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)
    .map(|(rest, (n, (l, r)))| (rest, (n.to_owned(), (l.to_owned(), r.to_owned()))))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
