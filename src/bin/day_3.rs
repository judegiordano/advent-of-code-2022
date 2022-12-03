use std::collections::BTreeMap;

use anyhow::Result;
use rayon::{
    prelude::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelBridge,
        ParallelIterator,
    },
    slice::ParallelSlice,
    str::ParallelString,
};

const INPUT: &str = include_str!("../../inputs/day3.txt");

const ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

trait CharHelpers {
    fn score(&self) -> u32;
}

impl CharHelpers for char {
    fn score(&self) -> u32 {
        if let Some(score) = ALPHABET.par_iter().position_last(|a| a == self) {
            let score = score as u32 + 1;
            score
        } else {
            panic!("handle later")
        }
    }
}

/// Lowercase item types `a` through `z` have priorities 1 through 26
///
/// Uppercase item types `A` through `Z` have priorities 27 through 52
fn part1() -> u32 {
    INPUT
        .par_lines()
        .fold(
            || 0,
            |mut count, line| {
                let (first_half, second_half) = line.split_at(line.len() / 2);
                for char in first_half.chars().into_iter() {
                    if second_half.contains(char) {
                        count += char.score();
                        break;
                    }
                }
                count
            },
        )
        .sum::<u32>()
}

fn part2() -> u32 {
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    // split input into a vector of chunks of 3
    let chunks = lines.par_chunks(3).collect::<Vec<_>>();
    chunks
        .par_iter()
        .fold(
            || 0,
            |mut answer, chunk| {
                let (first, second, third) =
                    (chunk[0].chars(), chunk[1].par_chars(), chunk[2].chars());
                let initial_lookup = first.into_iter().fold(BTreeMap::new(), |mut tree, char| {
                    tree.insert(char, char.score());
                    tree
                });
                let potential_solutions = &second
                    .into_par_iter()
                    .fold(
                        || BTreeMap::new(),
                        |mut tree, char| {
                            if initial_lookup.get(&char).is_some() {
                                tree.insert(char, char.score());
                            }
                            tree
                        },
                    )
                    .reduce(
                        || BTreeMap::new(),
                        |mut tree, current| {
                            current.iter().for_each(|(char, value)| {
                                tree.insert(char.clone(), value.clone());
                            });
                            tree
                        },
                    );
                // let potential_solutions =
                //     &second.into_iter().fold(BTreeMap::new(), |mut tree, char| {
                //         if initial_lookup.get(&char).is_some() {
                //             tree.insert(char, char.score());
                //         }
                //         tree
                //     });
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
        .sum::<u32>()
}

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let total = part1();
    println!("{:#?}", total);
    let total = part2();
    println!("{:#?}", total);
    println!("operation complete in: {:#?}", start.elapsed());
    Ok(())
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day3_tests() -> Result<()> {
        let total = part1();
        assert_eq!(total, 7817);
        let total = part2();
        assert_eq!(total, 2444);
        Ok(())
    }
}
