use anyhow::Result;
use rayon::{
    prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
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

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let total = part1();
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
        Ok(())
    }
}
