use std::collections::BTreeMap;

use anyhow::Result;
use rayon::{
    prelude::{
        FromParallelIterator, IndexedParallelIterator, IntoParallelIterator,
        IntoParallelRefIterator, ParallelIterator,
    },
    slice::ParallelSliceMut,
    str::ParallelString,
};

const INPUT: &str = include_str!("../../inputs/day2.txt");

#[derive(Debug, Clone)]
enum Attack {
    Rock,
    Paper,
    Scissors,
}

trait Round {
    fn round_score(&self) -> u32;
}

trait Game {
    fn game_tally(&self) -> u32;
}

impl Round for Vec<Attack> {
    fn round_score(&self) -> u32 {
        let mut score = 0;
        let their_attack = self.first().unwrap();
        let my_attack = self.last().unwrap();
        match my_attack {
            Attack::Rock => score += 1,
            Attack::Paper => score += 2,
            Attack::Scissors => score += 3,
        }
        match their_attack {
            Attack::Rock => match my_attack {
                Attack::Rock => score += 3,     // tie
                Attack::Paper => score += 6,    // win
                Attack::Scissors => score += 0, // loss
            },
            Attack::Paper => match my_attack {
                Attack::Rock => score += 0,     // loss
                Attack::Paper => score += 3,    // tie
                Attack::Scissors => score += 6, // win
            },
            Attack::Scissors => match my_attack {
                Attack::Rock => score += 6,     // win
                Attack::Paper => score += 0,    // loss
                Attack::Scissors => score += 3, // tie
            },
        }
        score
    }
}

impl Game for Vec<Vec<Attack>> {
    fn game_tally(&self) -> u32 {
        self.into_par_iter()
            .fold(|| 0, |total, round| total + round.round_score())
            .sum::<u32>()
    }
}

// 1 pts for rock (A) (X)
// 2 pts for paper (B) (Y)
// 3 pts for scissors (C) (Z)
// + score of the outcome of the round

// 0 for loss
// 3 for draw
// 6 for win
fn part1() -> Result<Vec<Vec<Attack>>> {
    let mut rounds = vec![];
    for line in INPUT.lines() {
        let throws = &line.split(' ').collect::<Vec<_>>()[..];
        let their_attack = match throws.first() {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "A" => Attack::Rock,
                "B" => Attack::Paper,
                "C" => Attack::Scissors,
                _ => anyhow::bail!("not supported"),
            },
            None => todo!(),
        };
        let my_attack = match throws.last() {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "X" => Attack::Rock,
                "Y" => Attack::Paper,
                "Z" => Attack::Scissors,
                _ => anyhow::bail!("not supported"),
            },
            None => todo!(),
        };
        rounds.push([their_attack, my_attack].to_vec())
    }
    Ok(rounds)
}

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let total = part1()?.game_tally();
    println!("{total:#?}");
    println!("operation complete in: {:#?}", start.elapsed());
    Ok(())
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day2_tests() -> Result<()> {
        let total = part1()?.game_tally();
        assert_eq!(total, 15691);
        Ok(())
    }
}
