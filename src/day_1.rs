use std::collections::BTreeMap;

use crate::helpers::read_inputs_txt;

fn part1() -> anyhow::Result<()> {
    let content = read_inputs_txt("day1part1.txt")?;
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
    println!("{:#?}", elf);
    println!("{:#?}", calories);
    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    part1()
}
