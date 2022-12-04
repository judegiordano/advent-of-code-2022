use std::ops::Not;

use array_tool::vec::Intersect;
use rayon::{prelude::ParallelIterator, str::ParallelString};

const INPUT: &str = include_str!("../../inputs/day4.txt");

trait ArrayHelper {
    fn to_num_vec(&self) -> Vec<u32>;
}

impl ArrayHelper for Vec<&str> {
    fn to_num_vec(&self) -> Vec<u32> {
        let (a, b) = (
            self.first().unwrap().parse::<u32>().unwrap(),
            self.last().unwrap().parse::<u32>().unwrap(),
        );
        let mut temp = vec![];
        for i in a..=b {
            temp.push(i)
        }
        temp
    }
}

fn part1(lines: &[&str]) -> u32 {
    let start = std::time::Instant::now();
    let mut total = 0;
    for line in lines {
        let elves = line.split(',').collect::<Vec<_>>();
        let first = elves.first().unwrap().split('-').collect::<Vec<_>>();
        let (a, b) = (
            first.first().unwrap().parse::<u32>().unwrap(),
            first.last().unwrap().parse::<u32>().unwrap(),
        );
        let last = elves.last().unwrap().split('-').collect::<Vec<_>>();
        let (x, y) = (
            last.first().unwrap().parse::<u32>().unwrap(),
            last.last().unwrap().parse::<u32>().unwrap(),
        );
        if a <= x && b >= y || x <= a && y >= b {
            total += 1
        }
    }
    println!("operation complete in: {:#?}", start.elapsed());
    total
}

fn part2(lines: &[&str]) -> u32 {
    let start = std::time::Instant::now();
    let mut total = 0;
    for line in lines {
        let elves = line.split(',').collect::<Vec<_>>();
        let first = elves
            .first()
            .unwrap()
            .split('-')
            .collect::<Vec<_>>()
            .to_num_vec();
        let last = elves
            .last()
            .unwrap()
            .split('-')
            .collect::<Vec<_>>()
            .to_num_vec();
        if first.intersect(last).is_empty().not() {
            total += 1;
        }
    }
    println!("operation complete in: {:#?}", start.elapsed());
    total
}

pub fn main() {
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    let answer = part1(&lines);
    println!("{:#?}", answer);
    let answer2 = part2(&lines);
    println!("{:#?}", answer2);
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
