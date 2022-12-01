use std::collections::BTreeMap;

use anyhow::Result;
use rayon::{
    prelude::{
        FromParallelIterator, IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
    slice::ParallelSliceMut,
};

fn part1(input: &str) -> Result<Vec<(u64, u64)>> {
    let mut calories_by_elf: BTreeMap<u64, u64> = BTreeMap::new();
    let mut elf = 0;
    for a in input.lines() {
        // whitespace in input used as separator
        if a.is_empty() {
            // new elf
            elf += 1;
            continue;
        }
        let current_value = calories_by_elf.get(&elf);
        let value = a.parse::<u64>()?;
        match current_value {
            // update elf total count if exists
            Some(current) => calories_by_elf.insert(elf, current + value),
            // create new elf
            None => calories_by_elf.insert(elf, value),
        };
    }
    let mut v = Vec::from_par_iter(calories_by_elf);
    v.par_sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    Ok(v)
}

fn part2(sorted_list: &[(u64, u64)]) -> Result<u64> {
    let top_3 = sorted_list.par_iter().take(3).collect::<Vec<_>>();
    let total = top_3.iter().fold(0, |acc, (_, calories)| acc + calories);
    Ok(total)
}

pub fn run() -> Result<()> {
    let input = include_str!("../inputs/day1.txt");
    let sorted_list = part1(input)?;
    match sorted_list.first() {
        Some((elf, calories)) => println!("top calories: {calories:?} held by elf {elf:?}"),
        None => anyhow::bail!("empty"),
    }
    let top_3_total = part2(&sorted_list)?;
    println!("total calories held by top 3 elves: {top_3_total:?}");
    Ok(())
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day1_tests() -> Result<()> {
        let input = include_str!("../inputs/day1.txt");
        let sorted_list = part1(&input)?;
        match sorted_list.first() {
            Some((_, calories)) => assert_eq!(*calories, 69626),
            None => anyhow::bail!("empty"),
        }
        let top_3_total = part2(&sorted_list)?;
        assert_eq!(top_3_total, 206780);
        Ok(())
    }
}
