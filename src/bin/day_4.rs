use std::ops::Not;

use advent_of_code_2022::utils::init_logger;
use array_tool::vec::Intersect;
use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

const INPUT: &str = include_str!("../../inputs/day4.txt");

trait ArrayHelper {
    fn to_num_vec(&self) -> Vec<u32>;
}

trait StringHelper {
    fn input_to_vec(&self) -> Vec<u32>;
    fn option_to_num(&self) -> u32;
    fn get_str(&self) -> &str;
}

impl StringHelper for Option<&String> {
    fn input_to_vec(&self) -> Vec<u32> {
        self.map_or_else(
            || {
                tracing::error!("no vec option found");
                std::process::exit(1);
            },
            |str| {
                str.split('-')
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .to_num_vec()
            },
        )
    }

    fn option_to_num(&self) -> u32 {
        self.map_or_else(
            || {
                tracing::error!("no option found");
                std::process::exit(1);
            },
            |val| match val.parse::<u32>() {
                Ok(int) => int,
                Err(err) => {
                    tracing::error!("error parsing int {err}");
                    std::process::exit(1)
                }
            },
        )
    }

    fn get_str(&self) -> &str {
        &self.map_or_else(
            || {
                tracing::error!("no string option found");
                std::process::exit(1)
            },
            |str| str,
        )
    }
}

impl ArrayHelper for Vec<String> {
    fn to_num_vec(&self) -> Vec<u32> {
        let (a, b) = (self.first().option_to_num(), self.last().option_to_num());
        let mut temp = vec![];
        for i in a..=b {
            temp.push(i);
        }
        temp
    }
}

fn part1(lines: &[&str]) -> u32 {
    let start = std::time::Instant::now();
    let total = lines
        .par_iter()
        .fold(
            || 0,
            |mut acc, line| {
                let elves = line
                    .split(',')
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>();
                let (first, last) = (elves.first(), elves.last());
                let first = first
                    .get_str()
                    .split('-')
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>();
                let last = last
                    .get_str()
                    .split('-')
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>();
                let (a, b) = (first.first().option_to_num(), first.last().option_to_num());
                let (x, y) = (last.first().option_to_num(), last.last().option_to_num());
                if a <= x && b >= y || x <= a && y >= b {
                    acc += 1;
                }
                acc
            },
        )
        .sum::<u32>();
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    total
}

fn part2(lines: &[&str]) -> u32 {
    let start = std::time::Instant::now();
    let total = lines
        .par_iter()
        .fold(
            || 0,
            |mut acc, line| {
                let elves = line
                    .split(',')
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>();
                let first_sect = elves.first().input_to_vec();
                let last_sect = elves.last().input_to_vec();
                if first_sect.intersect(last_sect).is_empty().not() {
                    acc += 1;
                }
                acc
            },
        )
        .sum::<u32>();
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    total
}

pub fn main() {
    init_logger();
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    let answer = part1(&lines);
    tracing::info!("{answer:#?}");
    let answer2 = part2(&lines);
    tracing::info!("{answer2:#?}");
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day4_tests() {
        init_logger();
        let lines = INPUT.par_lines().collect::<Vec<_>>();
        let total = part1(&lines);
        assert_eq!(total, 503);
        let total = part2(&lines);
        assert_eq!(total, 827);
    }
}
