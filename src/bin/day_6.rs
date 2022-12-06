use std::collections::HashSet;

use advent_of_code_2022::utils::init_logger;

const INPUT: &str = include_str!("../../inputs/day6.txt");

fn part1() -> usize {
    let start = std::time::Instant::now();
    let mut index = 0;
    let chars = INPUT
        .chars()
        .into_iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>();
    for _ in &chars {
        index += 1;
        if index < 4 {
            continue;
        }
        let slice = &chars[(index - 4)..index];
        let mut uniq = HashSet::new();
        let no_duplicates = slice.into_iter().all(move |x| uniq.insert(x));
        if no_duplicates {
            break;
        }
    }
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    index
}

pub fn main() {
    init_logger();
    let answer = part1();
    tracing::info!("{answer}");
}
