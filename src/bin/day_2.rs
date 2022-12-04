use advent_of_code_2022::utils::init_logger;
use anyhow::Result;
use rayon::{
    prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

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

trait OutPutKey {
    fn to_attack(&self) -> Result<Attack>;
    fn to_game_result(&self) -> Result<GameOutcome>;
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

impl OutPutKey for Option<&String> {
    // map a given input to a logical attack enum
    fn to_attack(&self) -> Result<Attack> {
        match *self {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "A" | "X" => Ok(Attack::Rock),
                "B" | "Y" => Ok(Attack::Paper),
                "C" | "Z" => Ok(Attack::Scissors),
                _ => anyhow::bail!("{play} not supported"),
            },
            None => anyhow::bail!("play option not found"),
        }
    }

    // for pt 2, we discover our input key was actually game result
    fn to_game_result(&self) -> Result<GameOutcome> {
        match *self {
            Some(play) => match play.trim().to_uppercase().as_ref() {
                "X" => Ok(GameOutcome::Lose),
                "Y" => Ok(GameOutcome::Draw),
                "Z" => Ok(GameOutcome::Win),
                _ => anyhow::bail!("{play} not supported"),
            },
            None => anyhow::bail!("play option not found"),
        }
    }
}

fn part1(lines: &[&str]) -> Vec<GameOutput> {
    let start = std::time::Instant::now();
    let rounds = lines
        .par_iter()
        .map(|line| -> GameOutput {
            let throws = line
                .split_whitespace()
                .into_iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
            let their_attack = match throws.first().to_attack() {
                Ok(attack) => attack,
                Err(err) => {
                    tracing::error!("{err}");
                    std::process::exit(1)
                }
            };
            let my_attack = match throws.last().to_attack() {
                Ok(attack) => attack,
                Err(err) => {
                    tracing::error!("{err}");
                    std::process::exit(1)
                }
            };
            GameOutput::Part1((their_attack, my_attack))
        })
        .collect::<Vec<_>>();
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    rounds
}

fn part2(lines: &[&str]) -> Vec<GameOutput> {
    let start = std::time::Instant::now();
    let rounds = lines
        .par_iter()
        .map(|line| -> GameOutput {
            let throws = line
                .split_whitespace()
                .into_iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
            let their_attack = match throws.first().to_attack() {
                Ok(attack) => attack,
                Err(err) => {
                    tracing::error!("{err}");
                    std::process::exit(1)
                }
            };
            let game_outcome = match throws.last().to_game_result() {
                Ok(outcome) => outcome,
                Err(err) => {
                    tracing::error!("{err}");
                    std::process::exit(1)
                }
            };
            GameOutput::Part2((their_attack, game_outcome))
        })
        .collect::<Vec<_>>();
    tracing::info!("operation complete in: {:#?}", start.elapsed());
    rounds
}

pub fn main() -> Result<()> {
    init_logger();
    let lines = INPUT.par_lines().collect::<Vec<_>>();
    let total = part1(&lines).game_tally();
    tracing::info!("initial game info total: {total:#?}");
    let total = part2(&lines).game_tally();
    tracing::info!("final game info total: {total:#?}");
    Ok(())
}

#[allow(unused_imports)]
#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn day2_tests() -> Result<()> {
        init_logger();
        let lines = INPUT.par_lines().collect::<Vec<_>>();
        let total = part1(&lines).game_tally();
        assert_eq!(total, 15691);
        let total = part2(&lines).game_tally();
        assert_eq!(total, 12989);
        Ok(())
    }
}
