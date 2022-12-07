use std::collections::HashSet;

use advent_of_code_2022::utils::init_logger;

const INPUT: &str = include_str!("../../inputs/day6.txt");

trait CharHelpers {
    fn find_unique_marker(&self, index: usize) -> usize;
}

impl CharHelpers for Vec<String> {
    fn find_unique_marker(&self, marker_length: usize) -> usize {
        let mut index = 0;
        for _ in self {
            index += 1;
            if index < marker_length {
                continue;
            }
            let slice = &self[(index - marker_length)..index];
            let mut uniq = HashSet::new();
            let no_duplicates = slice.iter().all(move |x| uniq.insert(x));
            if no_duplicates {
                break;
            }
        }
        index
    }
}

fn part1() -> usize {
    let start = std::time::Instant::now();
    let chars = INPUT
        .chars()
        .into_iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>();
    let index = chars.find_unique_marker(4);
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    index
}

fn part2() -> usize {
    let start = std::time::Instant::now();
    let chars = INPUT
        .chars()
        .into_iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>();
    let index = chars.find_unique_marker(14);
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    index
}

pub fn main() {
    init_logger();
    let answer = part1();
    tracing::info!("{answer}");
    let answer = part2();
    tracing::info!("{answer}");
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day5_tests() {
        init_logger();
        let answer = part1();
        assert_eq!(answer, 1093);
        let answer = part2();
        assert_eq!(answer, 3534);
    }
}
