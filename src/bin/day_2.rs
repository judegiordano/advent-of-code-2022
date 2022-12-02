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

enum GameOutput {
    Part1((Attack, Attack)),
    Part2((Attack, GameOutcome)),
}

#[derive(Debug, Clone)]
enum Attack {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone)]
enum GameOutcome {
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

trait Part1Key {
    fn to_opponent_attack(&self) -> Attack;
    fn to_my_attack(&self) -> Attack;
}

trait Part2Key {
    fn to_game_result(&self) -> GameOutcome;
}

impl Round for GameOutput {
    fn round_score(&self) -> u32 {
        let mut score = 0;
        match self {
            // per part1 requirements, tally score based on attack vs attack
            Self::Part1(input) => {
                let (their_attack, my_attack) = input;
                // modify base score based on what i threw
                match my_attack {
                    Attack::Rock => score += 1,
                    Attack::Paper => score += 2,
                    Attack::Scissors => score += 3,
                }
                // match opponent attack to outcome
                match their_attack {
                    Attack::Rock => match my_attack {
                        Attack::Rock => score += 3,     // draw
                        Attack::Paper => score += 6,    // win
                        Attack::Scissors => score += 0, // loss
                    },
                    Attack::Paper => match my_attack {
                        Attack::Rock => score += 0,     // loss
                        Attack::Paper => score += 3,    // draw
                        Attack::Scissors => score += 6, // win
                    },
                    Attack::Scissors => match my_attack {
                        Attack::Rock => score += 6,     // win
                        Attack::Paper => score += 0,    // loss
                        Attack::Scissors => score += 3, // draw
                    },
                }
            }
            // per part2 requirements, tally score based on attack -> game result
            Self::Part2(input) => {
                let (their_attack, outcome) = input;
                match their_attack {
                    Attack::Rock => match outcome {
                        GameOutcome::Win => score += 8,  // paper + win
                        GameOutcome::Lose => score += 3, // scissors + loss
                        GameOutcome::Draw => score += 4, // rock + draw
                    },
                    Attack::Paper => match outcome {
                        GameOutcome::Win => score += 9,  // scissors + win
                        GameOutcome::Lose => score += 1, // rock + loss
                        GameOutcome::Draw => score += 5, // paper + draw
                    },
                    Attack::Scissors => match outcome {
                        GameOutcome::Win => score += 7,  // rock + win
                        GameOutcome::Lose => score += 2, // paper + loss
                        GameOutcome::Draw => score += 6, // scissors + draw
                    },
                }
            }
        }
        score
    }
}

impl Game for Vec<GameOutput> {
    // tally entire game score given input of games
    fn game_tally(&self) -> u32 {
        self.into_par_iter()
            .map(Round::round_score)
            .fold(|| 0, |total, round| total + round)
            .sum::<u32>()
    }
}

impl Part1Key for Option<&String> {
    // map an opponents given input to a logical attack enum
    fn to_opponent_attack(&self) -> Attack {
        match *self {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "A" => Attack::Rock,
                "B" => Attack::Paper,
                "C" => Attack::Scissors,
                _ => panic!("handle this"),
            },
            None => panic!("handle this"),
        }
    }

    // map my given input to a logical attack enum
    fn to_my_attack(&self) -> Attack {
        match self {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "X" => Attack::Rock,
                "Y" => Attack::Paper,
                "Z" => Attack::Scissors,
                _ => panic!("handle this"),
            },
            None => panic!("handle this"),
        }
    }
}

impl Part2Key for Option<&String> {
    fn to_game_result(&self) -> GameOutcome {
        match *self {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "X" => GameOutcome::Lose,
                "Y" => GameOutcome::Draw,
                "Z" => GameOutcome::Win,
                _ => panic!("handle this"),
            },
            None => panic!("handle this"),
        }
    }
}

fn part1() -> Vec<GameOutput> {
    let mut rounds = vec![];
    for line in INPUT.lines() {
        let line = line.to_string();
        let throws = line
            .split(' ')
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        let their_attack = throws.first().to_opponent_attack();
        let my_attack = throws.last().to_my_attack();
        rounds.push(GameOutput::Part1((their_attack, my_attack)));
    }
    rounds
}

fn part2() -> Vec<GameOutput> {
    let mut rounds = vec![];
    for line in INPUT.lines() {
        let throws = line
            .split(' ')
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        let their_attack = throws.first().to_opponent_attack();
        let my_outcome = throws.last().to_game_result();
        rounds.push(GameOutput::Part2((their_attack, my_outcome)));
    }
    rounds
}

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let total = part1().game_tally();
    println!("initial game info total: {total:#?}");
    let total = part2().game_tally();
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
        let total = part1().game_tally();
        assert_eq!(total, 15691);
        let total = part2().game_tally();
        assert_eq!(total, 12989);
        Ok(())
    }
}
