use advent_of_code_2022::{
    types::{StringHelpers, VecHelpers},
    utils::init_logger,
};
use rayon::{
    prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../../inputs/day5.txt");

trait CrateTree {
    fn generate_tree(&self) -> (BTreeMap<u32, Vec<String>>, usize);
    fn generate_instructions(&self, skip: usize) -> Vec<Command>;
}

trait CrateTreeHelper {
    fn aggregate_top_letters(&self) -> String;
    fn get_safe(&self, index: u32) -> Vec<String>;
}

#[derive(Debug)]
pub struct Command {
    pub amount: u32,
    pub target: u32,
    pub destination: u32,
}

impl CrateTree for [&str] {
    fn generate_tree(&self) -> (BTreeMap<u32, Vec<String>>, usize) {
        let mut skip_lines = 0;
        let mut crates = BTreeMap::new();
        for line in self {
            skip_lines += 1;
            if line.is_empty() {
                break;
            }
            let mut chars = line
                .split(' ')
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>();
            let crate_num = chars.get_first().parse_safe::<u32>();
            chars.remove(0);
            crates.insert(
                crate_num,
                chars
                    .iter()
                    .map(|a| a.replace(['[', ']'], ""))
                    .collect::<Vec<_>>(),
            );
        }
        (crates, skip_lines)
    }

    fn generate_instructions(&self, skip: usize) -> Vec<Command> {
        self.par_iter()
            .skip(skip)
            .map(|line| {
                let parts = line.split_whitespace().fold(vec![], |mut acc, str| {
                    str.parse::<u32>().map_or((), |int| acc.push(int));
                    acc
                });
                let (amount, target, destination) = (parts[0], parts[1], parts[2]);
                Command {
                    amount,
                    target,
                    destination,
                }
            })
            .collect::<Vec<_>>()
    }
}

impl CrateTreeHelper for BTreeMap<u32, Vec<String>> {
    fn aggregate_top_letters(&self) -> String {
        self.par_iter()
            .fold(
                || String::new(),
                |mut acc, (_, map)| {
                    acc.push_str(&map.get_last());
                    acc
                },
            )
            .reduce(
                || String::new(),
                |mut acc, char| {
                    acc.push_str(&char);
                    acc
                },
            )
    }

    fn get_safe(&self, index: u32) -> Vec<String> {
        self.get(&index).map_or_else(
            || {
                tracing::error!("no create elem found at index {index}");
                std::process::exit(1)
            },
            std::clone::Clone::clone,
        )
    }
}

fn part1(lines: &[&str]) -> String {
    let start = std::time::Instant::now();
    let (mut crates, skip) = lines.generate_tree();
    // // parse instructions
    let instructions = lines.generate_instructions(skip);
    // execute instructions - afaik must be executed sequentially
    for command in instructions {
        let Command {
            amount,
            target,
            destination,
        } = command;
        let mut target_crates = crates.get_safe(target);
        let mut destination_crates = crates.get_safe(destination);
        for _ in 0..amount {
            destination_crates.push(target_crates.pop_last());
        }
        crates.insert(target, target_crates.clone());
        crates.insert(destination, destination_crates.clone());
    }
    // get top letters of each stack
    let answer = crates.aggregate_top_letters();
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    answer
}

fn part2(lines: &[&str]) -> String {
    let start = std::time::Instant::now();
    let (mut crates, skip) = lines.generate_tree();
    // // parse instructions
    let instructions = lines.generate_instructions(skip);
    // execute instructions - afaik must be executed sequentially
    for command in instructions {
        let Command {
            amount,
            target,
            destination,
        } = command;
        let mut target_crates = crates.get_safe(target);
        let mut destination_crates = crates.get_safe(destination);
        let mut temp = vec![];
        for _ in 0..amount {
            temp.push(target_crates.pop_last());
        }
        temp.reverse();
        destination_crates.append(&mut temp);
        crates.insert(target, target_crates.clone());
        crates.insert(destination, destination_crates.clone());
    }
    // get top letters of each stack
    let answer = crates.aggregate_top_letters();
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    answer
}

pub fn main() {
    init_logger();
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    let answer = part1(&lines);
    tracing::info!("{answer}");
    let answer = part2(&lines);
    tracing::info!("{answer}");
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day5_tests() {
        init_logger();
        let lines = INPUT.par_lines().collect::<Vec<_>>();
        let answer = part1(&lines);
        assert_eq!(answer, "VPCDMSLWJ");
        let answer = part2(&lines);
        assert_eq!(answer, "TPWCGNCCG");
    }
}
