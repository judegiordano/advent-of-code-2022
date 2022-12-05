use advent_of_code_2022::utils::init_logger;
use rayon::{prelude::ParallelIterator, str::ParallelString};
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../../inputs/day5.txt");

#[derive(Debug)]
pub struct Command {
    pub amount: u32,
    pub target: u32,
    pub destination: u32,
}

fn part1(lines: &[&str]) {
    let start = std::time::Instant::now();
    let mut crates = BTreeMap::new();
    let mut line_count = 0;
    // build crate layout
    for line in lines {
        line_count += 1;
        if line.is_empty() {
            break;
        }
        let mut chars = line.split(' ').collect::<Vec<_>>();
        let crate_num = chars.first().unwrap().parse::<u32>().unwrap();
        chars.remove(0);
        crates.insert(
            crate_num,
            chars
                .iter()
                .map(|a| a.replace(&['[', ']'], ""))
                .collect::<Vec<_>>(),
        );
    }
    // parse instructions
    let mut instructions = vec![];
    for line in lines.iter().skip(line_count) {
        let parts = line.split_whitespace().fold(vec![], |mut acc, b| {
            match b.parse::<u32>() {
                Ok(int) => acc.push(int),
                Err(_) => (),
            };
            acc
        });
        let (amount, target, destination) = (parts[0], parts[1], parts[2]);
        instructions.push(Command {
            amount,
            target,
            destination,
        });
    }

    // execute instructions
    for command in instructions {
        let Command {
            amount,
            target,
            destination,
        } = command;
        let mut target_crates = crates.get(&target).unwrap().to_owned();
        let mut destination_crates = crates.get(&destination).unwrap().to_owned();
        for _ in 0..amount {
            let ayo = target_crates.pop().unwrap();
            destination_crates.push(ayo);
        }
        crates.insert(target, target_crates.to_vec());
        crates.insert(destination, destination_crates.to_vec());
    }
    // get top letters of each stack
    let tally = crates.iter().fold(String::new(), |mut acc, (_, map)| {
        acc.push_str(&map.last().unwrap().to_string());
        acc
    });
    println!("{:#?}", crates);
    println!("{:#?}", tally);
    tracing::info!("operation complete in: {:#?}", start.elapsed());
}

pub fn main() {
    init_logger();
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    let _ = part1(&lines);
    // tracing::info!("{answer:#?}");
    // let answer2 = part2(&lines);
    // tracing::info!("{answer2:#?}");
}
