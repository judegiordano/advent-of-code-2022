use std::collections::BTreeMap;

use anyhow::Result;
use rayon::{
    prelude::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
    slice::ParallelSlice,
    str::{Chars as ParChars, ParallelString},
};

const INPUT: &str = include_str!("../../inputs/day3.txt");

const ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

type LookupType = BTreeMap<char, u32>;

trait CharHelpers {
    fn score(&self) -> u32;
}

trait TupleCharSearcher {
    fn find_char_intersection(&self) -> u32;
}

trait ParCharsHelpers {
    fn fold_to_lookup(&self) -> LookupType;
    fn fold_to_duplicate_lookup(&self, target: &LookupType) -> LookupType;
}

impl TupleCharSearcher for (&str, &str) {
    fn find_char_intersection(&self) -> u32 {
        let mut count = 0;
        for char in self.0.chars() {
            if self.1.contains(char) {
                count += char.score();
                return count;
            }
        }
        count
    }
}

impl CharHelpers for char {
    fn score(&self) -> u32 {
        ALPHABET
            .par_iter()
            .position_last(|a| a == self)
            .map_or_else(
                || {
                    eprintln!("no char found");
                    std::process::exit(1)
                },
                |score| match (score + 1).try_into() {
                    Ok(score) => score,
                    Err(err) => {
                        eprintln!("unable to cast score {err}");
                        std::process::exit(1)
                    }
                },
            )
    }
}

impl ParCharsHelpers for ParChars<'_> {
    fn fold_to_lookup(&self) -> LookupType {
        self.clone()
            .into_par_iter()
            .fold(
                || BTreeMap::new(),
                |mut tree, char| {
                    tree.insert(char, char.score());
                    tree
                },
            )
            .reduce(
                || BTreeMap::new(),
                |mut tree, current| {
                    for (char, value) in &current {
                        tree.insert(*char, *value);
                    }
                    tree
                },
            )
    }

    fn fold_to_duplicate_lookup(&self, target: &LookupType) -> LookupType {
        self.clone()
            .into_par_iter()
            .fold(
                || BTreeMap::new(),
                |mut tree, char| {
                    if target.get(&char).is_some() {
                        tree.insert(char, char.score());
                    }
                    tree
                },
            )
            // .reduce(|| BTreeMap::new(), BTreeMap::new() || {});
            .reduce(
                || BTreeMap::new(),
                |mut tree, current| {
                    for (char, value) in &current {
                        tree.insert(*char, *value);
                    }
                    tree
                },
            )
    }
}

/// Lowercase item types `a` through `z` have priorities 1 through 26
///
/// Uppercase item types `A` through `Z` have priorities 27 through 52
fn part1(lines: &[&str]) -> u32 {
    let start = std::time::Instant::now();
    let answer = lines
        .par_iter()
        .fold(
            || 0,
            |mut count, line| {
                count += line.split_at(line.len() / 2).find_char_intersection();
                count
            },
        )
        .sum::<u32>();
    println!("operation complete in: {:#?}", start.elapsed());
    answer
}

fn part2(lines: &[&str]) -> u32 {
    let start = std::time::Instant::now();
    // split input into a vector of chunks of 3
    let chunks = lines.par_chunks(3).collect::<Vec<_>>();
    let answer = chunks
        .par_iter()
        .fold(
            || 0,
            |mut answer, chunk| {
                // find matches between 3 string inputs
                let (first, second, third) =
                    (chunk[0].par_chars(), chunk[1].par_chars(), chunk[2].chars());
                // set baseline of char
                let initial_lookup = first.fold_to_lookup();
                // reduce to only duplicates
                let potential_solutions = second.fold_to_duplicate_lookup(&initial_lookup);
                // break to final char that all 3 lines contain
                for char in third {
                    if potential_solutions.get(&char).is_some() {
                        answer += char.score();
                        // no need to continue
                        break;
                    }
                }
                answer
            },
        )
        .sum::<u32>();
    println!("operation complete in: {:#?}", start.elapsed());
    answer
}

pub fn main() -> Result<()> {
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    let total = part1(&lines);
    println!("{total:#?}");
    let total = part2(&lines);
    println!("{total:#?}");
    Ok(())
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day3_tests() -> Result<()> {
        let lines = INPUT.par_lines().collect::<Vec<_>>();
        let total = part1(&lines);
        assert_eq!(total, 7817);
        let total = part2(&lines);
        assert_eq!(total, 2444);
        Ok(())
    }
}
