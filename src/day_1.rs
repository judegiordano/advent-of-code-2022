use std::collections::BTreeMap;

use anyhow::Result;
use rayon::{
    prelude::{
        FromParallelIterator, IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
    slice::ParallelSliceMut,
};

// answer 69626
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
    match v.first() {
        Some((elf, calories)) => println!("top calories: {calories:?} held by elf {elf:?}"),
        None => anyhow::bail!("empty"),
    }
    Ok(v)
}

// answer 206780
fn part2(input: &str) -> Result<()> {
    let sorted_list = part1(input)?;
    let top_3 = sorted_list.par_iter().take(3).collect::<Vec<_>>();
    let total = top_3.iter().fold(0, |acc, (_, calories)| acc + calories);
    println!("total calories held by top 3 elves: {total:?}");
    Ok(())
}

pub fn run(input: &str) -> Result<()> {
    part2(input)?;
    Ok(())
}
