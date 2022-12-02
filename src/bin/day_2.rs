use anyhow::Result;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const INPUT: &str = include_str!("../../inputs/day2.txt");

// 1 pts for rock (A)
// 2 pts for paper (B)
// 3 pts for scissors (C)

// + score of the outcome of the round

// 0 for loss
// 3 for draw
// 6 for win

// X -> lose
// Y -> draw
// Z -> win

#[derive(Debug, Clone)]
enum Attack {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

trait Round {
    fn round_score(&self) -> u32;
}

trait Game {
    fn game_tally(&self) -> u32;
}

impl Round for (Attack, Attack) {
    fn round_score(&self) -> u32 {
        let mut score = 0;
        let (their_attack, my_attack) = self;
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

impl Game for Vec<(Attack, Attack)> {
    fn game_tally(&self) -> u32 {
        self.into_par_iter()
            .fold(|| 0, |total, round| total + round.round_score())
            .sum::<u32>()
    }
}

impl Round for (Attack, Outcome) {
    fn round_score(&self) -> u32 {
        let mut score = 0;
        let (their_attack, outcome) = self;
        match their_attack {
            Attack::Rock => match outcome {
                Outcome::Win => score += 8,  // paper + win
                Outcome::Lose => score += 3, // scissors + loss
                Outcome::Draw => score += 4, // rock + draw
            },
            Attack::Paper => match outcome {
                Outcome::Win => score += 9,  // scissors + win
                Outcome::Lose => score += 1, // rock + loss
                Outcome::Draw => score += 5, // paper + draw
            },
            Attack::Scissors => match outcome {
                Outcome::Win => score += 7,  // rock + win
                Outcome::Lose => score += 2, // paper + loss
                Outcome::Draw => score += 6, // scissors + draw
            },
        }
        score
    }
}

impl Game for Vec<(Attack, Outcome)> {
    fn game_tally(&self) -> u32 {
        self.into_par_iter()
            .fold(|| 0, |total, round| total + round.round_score())
            .sum::<u32>()
    }
}

fn part1() -> Result<Vec<(Attack, Attack)>> {
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
            None => anyhow::bail!("not supported"),
        };
        let my_attack = match throws.last() {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "X" => Attack::Rock,
                "Y" => Attack::Paper,
                "Z" => Attack::Scissors,
                _ => anyhow::bail!("not supported"),
            },
            None => anyhow::bail!("not supported"),
        };
        rounds.push((their_attack, my_attack))
    }
    Ok(rounds)
}

fn part2() -> Result<Vec<(Attack, Outcome)>> {
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
            None => anyhow::bail!("not supported"),
        };
        let my_outcome = match throws.last() {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => anyhow::bail!("not supported"),
            },
            None => anyhow::bail!("not supported"),
        };
        rounds.push((their_attack, my_outcome))
    }
    Ok(rounds)
}

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let total = part1()?.game_tally();
    println!("initial game info total: {total:#?}");
    let total = part2()?.game_tally();
    println!("final game info total: {total:#?}");
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
        let total = part2()?.game_tally();
        assert_eq!(total, 12989);
        Ok(())
    }
}
