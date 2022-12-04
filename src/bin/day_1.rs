use rayon::{
    prelude::{
        FromParallelIterator, IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
    slice::ParallelSliceMut,
    str::ParallelString,
};
use std::collections::BTreeMap;
use tracing_subscriber::FmtSubscriber;

const INPUT: &str = include_str!("../../inputs/day1.txt");

fn init_logger() {
    let level = std::env::var("LOG_LEVEL").map_or(tracing::Level::ERROR, |found| {
        match found.trim().to_uppercase().as_ref() {
            "INFO" => tracing::Level::INFO,
            "DEBUG" => tracing::Level::DEBUG,
            "WARN" => tracing::Level::WARN,
            "TRACE" => tracing::Level::TRACE,
            _ => tracing::Level::ERROR,
        }
    });
    match tracing::subscriber::set_global_default(
        FmtSubscriber::builder().with_max_level(level).finish(),
    ) {
        Ok(_) => (),
        Err(err) => {
            tracing::error!("{err}");
            std::process::exit(1)
        }
    }
}

pub fn part1(lines: &[&str]) -> Vec<(u64, u64)> {
    let start = std::time::Instant::now();
    let mut calories_by_elf: BTreeMap<u64, u64> = BTreeMap::new();
    let mut elf = 0;
    for a in lines {
        // whitespace in input used as separator
        if a.is_empty() {
            // new elf
            elf += 1;
            continue;
        }
        let current_value = calories_by_elf.get(&elf);
        let value = match a.parse::<u64>() {
            Ok(int) => int,
            Err(err) => {
                tracing::error!("{err}");
                std::process::exit(1)
            }
        };
        match current_value {
            // update elf total count if exists
            Some(current) => calories_by_elf.insert(elf, current + value),
            // create new elf
            None => calories_by_elf.insert(elf, value),
        };
    }
    let mut v = Vec::from_par_iter(calories_by_elf);
    v.par_sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    v
}

pub fn part2(sorted_list: &[(u64, u64)]) -> u64 {
    let start = std::time::Instant::now();
    let top_3 = sorted_list.par_iter().take(3).collect::<Vec<_>>();
    let total = top_3.iter().fold(0, |acc, (_, calories)| acc + calories);
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    total
}

pub fn main() {
    init_logger();
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    let sorted_list = part1(&lines);
    match sorted_list.first() {
        Some((elf, calories)) => tracing::info!("top calories: {calories:?} held by elf {elf:?}"),
        None => {
            tracing::error!("not found");
            std::process::exit(1)
        }
    }
    let top_3_total = part2(&sorted_list);
    tracing::info!("total calories held by top 3 elves: {top_3_total:?}");
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day1_tests() {
        init_logger();
        let lines = INPUT.par_lines().collect::<Vec<_>>();
        let sorted_list = part1(&lines);
        match sorted_list.first() {
            Some((_, calories)) => assert_eq!(*calories, 69626),
            None => {
                tracing::error!("not found");
                std::process::exit(1)
            }
        }
        let top_3_total = part2(&sorted_list);
        assert_eq!(top_3_total, 206780);
    }
}
