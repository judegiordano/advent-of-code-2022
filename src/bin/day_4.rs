use std::ops::Not;

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

impl StringHelper for Option<&&str> {
    fn input_to_vec(&self) -> Vec<u32> {
        self.map_or_else(
            || {
                eprintln!("no vec option found");
                std::process::exit(1);
            },
            |str| str.split('-').collect::<Vec<_>>().to_num_vec(),
        )
    }

    fn option_to_num(&self) -> u32 {
        self.map_or_else(
            || {
                eprintln!("no option found");
                std::process::exit(1);
            },
            |val| match val.parse::<u32>() {
                Ok(int) => int,
                Err(err) => {
                    eprintln!("error parsing int {err}");
                    std::process::exit(1)
                }
            },
        )
    }

    fn get_str(&self) -> &str {
        self.map_or_else(
            || {
                eprintln!("no string option found");
                std::process::exit(1)
            },
            |str| str,
        )
    }
}

impl ArrayHelper for Vec<&str> {
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
                let elves = line.split(',').collect::<Vec<_>>();
                let (first, last) = (elves.first(), elves.last());
                let first = first.get_str().split('-').collect::<Vec<_>>();
                let last = last.get_str().split('-').collect::<Vec<_>>();
                let (a, b) = (first.first().option_to_num(), first.last().option_to_num());
                let (x, y) = (last.first().option_to_num(), last.last().option_to_num());
                if a <= x && b >= y || x <= a && y >= b {
                    acc += 1;
                }
                acc
            },
        )
        .sum::<u32>();
    println!("operation complete in: {:#?}", start.elapsed());
    total
}

fn part2(lines: &[&str]) -> u32 {
    let start = std::time::Instant::now();
    let total = lines
        .par_iter()
        .fold(
            || 0,
            |mut acc, line| {
                let elves = line.split(',').collect::<Vec<_>>();
                let first_sect = elves.first().input_to_vec();
                let last_sect = elves.last().input_to_vec();
                if first_sect.intersect(last_sect).is_empty().not() {
                    acc += 1;
                }
                acc
            },
        )
        .sum::<u32>();
    println!("operation complete in: {:#?}", start.elapsed());
    total
}

pub fn main() {
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    let answer = part1(&lines);
    println!("{answer:#?}");
    let answer2 = part2(&lines);
    println!("{answer2:#?}");
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day3_tests() {
        let lines = INPUT.par_lines().collect::<Vec<_>>();
        let total = part1(&lines);
        assert_eq!(total, 503);
        let total = part2(&lines);
        assert_eq!(total, 827);
    }
}
