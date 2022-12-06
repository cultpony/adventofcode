use std::str::FromStr;

use color_eyre::Report;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let input = read_file_lines("aoc2022/day2_1.txt").await?;
    let mut input = skip_empty_lines(input);
    let mut rounds = Vec::new();
    while let Some(round) = input.next().await {
        let round = Round::from_str(&round)?;
        trace!("Got round {round:?}, score for you {}", round.right_score());
        rounds.push(round);
    }
    let score = rounds.par_iter().map(|f| f.right_score()).sum();
    Ok(Reportable {
        year: 2022,
        day: 2,
        part: 1.into(),
        result: TaskResult::U32(score),
    })
}

#[cfg(test)]
#[tokio::test]
pub async fn validate_part1() -> Result<()> {
    let input = "A Y\nB X\nC Z".split('\n');
    let mut rounds = Vec::new();
    for line in input {
        rounds.push(Round::from_str(line)?);
    }
    assert_eq!(
        vec![
            Round(Play::Rock, Play::Paper),
            Round(Play::Paper, Play::Rock),
            Round(Play::Scissors, Play::Scissors),
        ],
        rounds
    );
    assert_eq!(15u32, rounds.iter().map(|f| f.right_score()).sum::<u32>());
    Ok(())
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let input = read_file_lines("aoc2022/day2_1.txt").await?;
    let mut input = skip_empty_lines(input);
    let mut rounds = Vec::new();
    while let Some(round) = input.next().await {
        let round = ORound::from_str(&round)?;
        trace!("Got round outcome {round:?}");
        let round: Round = round.into();
        trace!("Got round {round:?}, score for you {}", round.right_score());
        rounds.push(round);
    }
    let score = rounds.par_iter().map(|f| f.right_score()).sum();
    Ok(Reportable {
        year: 2022,
        day: 2,
        part: 2.into(),
        result: TaskResult::U32(score),
    })
}

#[cfg(test)]
#[tokio::test]
pub async fn validate_part2() -> Result<()> {
    let input = "A Y\nB X\nC Z".split('\n');
    let mut rounds = Vec::new();
    for line in input {
        rounds.push(ORound::from_str(line)?);
    }
    assert_eq!(
        vec![
            ORound(Play::Rock, Outcome::Draw),
            ORound(Play::Paper, Outcome::Loose),
            ORound(Play::Scissors, Outcome::Win),
        ],
        rounds
    );
    let rounds: Vec<Round> = rounds.into_iter().map(|f| f.into()).collect();
    assert_eq!(
        vec![
            Round(Play::Rock, Play::Rock),
            Round(Play::Paper, Play::Rock),
            Round(Play::Scissors, Play::Rock),
        ],
        rounds
    );
    assert_eq!(12u32, rounds.iter().map(|f| f.right_score()).sum::<u32>());
    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ORound(Play, Outcome);

impl FromStr for ORound {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').expect("invalid play line");
        Ok(Self(left.parse()?, right.parse()?))
    }
}

impl From<ORound> for Play {
    fn from(val: ORound) -> Self {
        match (val.0, val.1) {
            (Play::Rock, Outcome::Win) => Play::Paper,
            (Play::Rock, Outcome::Draw) => Play::Rock,
            (Play::Rock, Outcome::Loose) => Play::Scissors,
            (Play::Paper, Outcome::Win) => Play::Scissors,
            (Play::Paper, Outcome::Draw) => Play::Paper,
            (Play::Paper, Outcome::Loose) => Play::Rock,
            (Play::Scissors, Outcome::Win) => Play::Rock,
            (Play::Scissors, Outcome::Draw) => Play::Scissors,
            (Play::Scissors, Outcome::Loose) => Play::Paper,
        }
    }
}

impl From<ORound> for Round {
    fn from(val: ORound) -> Self {
        Round(val.0, val.into())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Round(Play, Play);

impl Round {
    pub fn right_score(&self) -> u32 {
        let outcome_score = match self.1.cmp(&self.0) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 6,
        };
        let item_score = match self.1 {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };
        outcome_score + item_score
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(' ').expect("invalid play line");
        Ok(Self(left.parse()?, right.parse()?))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Draw,
    Loose,
}

impl FromStr for Outcome {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "x" => Outcome::Loose,
            "y" => Outcome::Draw,
            "z" => Outcome::Win,
            _ => return Err(color_eyre::Report::msg("invalid rock-paper-scissor play")),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Play {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Play::*;
        Ok(match s.to_lowercase().as_str() {
            "a" | "x" => Rock,
            "b" | "y" => Paper,
            "c" | "z" => Scissors,
            _ => return Err(color_eyre::Report::msg("invalid rock-paper-scissor play")),
        })
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for Play {}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::{Equal, Greater, Less};
        Some(match (self, other) {
            (Play::Rock, Play::Rock) => Equal,
            (Play::Rock, Play::Paper) => Less,
            (Play::Rock, Play::Scissors) => Greater,
            (Play::Paper, Play::Rock) => Greater,
            (Play::Paper, Play::Paper) => Equal,
            (Play::Paper, Play::Scissors) => Less,
            (Play::Scissors, Play::Rock) => Less,
            (Play::Scissors, Play::Paper) => Greater,
            (Play::Scissors, Play::Scissors) => Equal,
        })
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // SAFETY: partial_cmp never constructs None
        unsafe { self.partial_cmp(other).unwrap_unchecked() }
    }
}
