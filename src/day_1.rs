use std::collections::BTreeMap;

use anyhow::Result;

use crate::helpers::read_inputs_txt;

fn part1() -> Result<Vec<(u64, u64)>> {
    let content = read_inputs_txt("day1")?;
    let mut calories_by_elf: BTreeMap<u64, u64> = BTreeMap::new();
    let mut elf = 0;
    for a in content.lines() {
        if a.len() == 0 {
            elf += 1;
            continue;
        }
        let current_value = calories_by_elf.get(&elf);
        let value = a.parse::<u64>()?;
        if current_value.is_some() {
            calories_by_elf.insert(elf, current_value.unwrap() + value);
        } else {
            calories_by_elf.insert(elf, value);
        }
    }
    let mut v = Vec::from_iter(calories_by_elf);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    let (elf, calories) = v.first().unwrap();
    println!("top calories: {:#?} held by elf {:?}", calories, elf);
    Ok(v)
}

fn part2() -> Result<()> {
    let sorted_list = part1()?;
    let top_3 = sorted_list.iter().take(3).collect::<Vec<_>>();
    let total = top_3.iter().fold(0, |acc, (_, calories)| acc + calories);
    println!("total calories held by top 3 elves: {:#?}", total);
    Ok(())
}

pub fn run() -> Result<()> {
    // part1()?;
    part2()?;
    Ok(())
}
