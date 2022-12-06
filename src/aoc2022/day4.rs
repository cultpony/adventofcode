use std::{ops::Range, str::FromStr};

use color_eyre::Report;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let input = read_file_lines("aoc2022/day4.txt").await?;
    let mut input = skip_empty_lines(input);
    let mut range_pairs: Vec<ElfPair> = Vec::new();
    while let Some(line) = input.next().await {
        range_pairs.push(line.parse()?);
    }

    let result = range_pairs
        .par_iter()
        .filter(|x| x.has_full_overlap())
        .count();
    assert!(result < u16::MAX as usize);
    let result = result as u16;

    Ok(Reportable {
        year: 2022,
        day: 4,
        part: 1.into(),
        result: TaskResult::U16(result),
    })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let input = read_file_lines("aoc2022/day4.txt").await?;
    let mut input = skip_empty_lines(input);
    let mut range_pairs: Vec<ElfPair> = Vec::new();
    while let Some(line) = input.next().await {
        range_pairs.push(line.parse()?);
    }

    let result = range_pairs
        .par_iter()
        .filter(|x| x.has_any_overlap())
        .count();
    assert!(result < u16::MAX as usize);
    let result = result as u16;

    Ok(Reportable {
        year: 2022,
        day: 4,
        part: 2.into(),
        result: TaskResult::U16(result),
    })
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ElfPair(ElfRange, ElfRange);

impl ElfPair {
    pub fn has_full_overlap(&self) -> bool {
        self.0.fully_contains(&self.1)
    }
    pub fn has_any_overlap(&self) -> bool {
        self.0.any_overlap(&self.1)
    }
}

impl FromStr for ElfPair {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = match s.split_once(',') {
            None => return Err(Report::msg("missing , delimiter")),
            Some(s) => s,
        };
        let (first, second): (ElfRange, ElfRange) = (first.parse()?, second.parse()?);
        Ok(ElfPair(first, second))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ElfRange(Range<u8>);

impl ElfRange {
    pub fn new(start: u8, end: u8) -> Self {
        assert!(start <= end);
        Self(Range {
            start,
            end: end + 1,
        })
    }
    pub fn any_overlap(&self, other: &Self) -> bool {
        if other.0.start > self.0.end || self.0.start > other.0.end {
            trace!("Excluding range due to start/end exclusion: {self:?} {other:?}");
            return false;
        }
        other.0.clone().any(|x| self.0.contains(&x))
            || self.0.clone().any(|x| other.0.contains(&x))
            || ((other.0.start == other.0.end && self.0.contains(&other.0.start))
                || (self.0.start == self.0.end && other.0.contains(&self.0.start)))
            || (other.0.start == self.0.start && other.0.end == self.0.end)
    }
    pub fn fully_contains(&self, other: &Self) -> bool {
        if other.0.start == self.0.start && other.0.end == self.0.end {
            trace!("Range equal overlap: {self:?} {other:?}");
            return true;
        }
        if other.0.start > self.0.end || self.0.start > other.0.end {
            trace!("Excluding range due to start/end exclusion: {self:?} {other:?}");
            return false;
        }
        (other.0.clone().all(|x| self.0.contains(&x))
            || self.0.clone().all(|x| other.0.contains(&x)))
            || (other.0.start == other.0.end && self.0.contains(&other.0.start))
            || (self.0.start == self.0.end && other.0.contains(&self.0.start))
    }
}

impl FromStr for ElfRange {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = match s.split_once('-') {
            None => return Err(Report::msg("missing - delimiter")),
            Some(s) => s,
        };
        let (start, end): (u8, u8) = (start.parse()?, end.parse()?);
        Ok(ElfRange(Range {
            start,
            end: end + 1,
        }))
    }
}

#[cfg(test)]
#[test]
pub fn test_verify_range_overlapping() {
    use itertools::Itertools;

    let input: Vec<ElfPair> = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ]
    .into_iter()
    .map(|x| x.parse().unwrap())
    .collect_vec();
    assert_eq!(input.iter().filter(|x| x.has_full_overlap()).count(), 2);
    assert_eq!(
        input
            .into_iter()
            .filter(|x| x.has_full_overlap())
            .collect_vec(),
        vec![
            ElfPair(ElfRange::new(2, 8), ElfRange::new(3, 7)),
            ElfPair(ElfRange::new(6, 6), ElfRange::new(4, 6))
        ]
    );
}
