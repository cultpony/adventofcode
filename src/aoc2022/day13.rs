use color_eyre::Report;
use either::Either;
use itertools::{FoldWhile, Itertools};
use regex::Regex;
use std::{convert::identity, str::FromStr};

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let input = read_file_chunks("aoc2022/day13.txt", "").await?;

    let mut packet_pairs = Vec::new();

    for line in input {
        let (a_value, b_value) = line.split_once('\n').unwrap();
        //debug!("a_value: {a_value:?}");
        let a_value: serde_json::Value = serde_json::from_str(a_value)?;
        let a_value: ProtoList = serde_json::from_value(a_value)?;
        let a_value: Packet = a_value.into();
        //debug!("a_value: {a_value:?}");
        //debug!("b_value: {b_value:?}");
        let b_value: serde_json::Value = serde_json::from_str(b_value)?;
        let b_value: ProtoList = serde_json::from_value(b_value)?;
        let b_value: Packet = b_value.into();
        //debug!("b_value: {b_value:?}");
        packet_pairs.push((a_value, b_value));
    }

    let pair_sum: usize = packet_pairs
        .into_iter()
        .enumerate()
        .flat_map(|(idx, (a, b))| {
            debug!("Checking pair {}", idx + 1);
            match a.correct_order(b) {
                PacketOrdering::Correct => {
                    debug!("{} correct", idx + 1);
                    Some(idx + 1)
                }
                PacketOrdering::Bad => None,
                PacketOrdering::Undecided => unreachable!(),
            }
        })
        .sum();

    Ok(Reportable {
        year: 2022,
        day: 13,
        part: 1.into(),
        result: TaskResult::Usize(pair_sum),
    })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let input = read_file_chunks("aoc2022/day13.txt", "").await?;

    Ok(Reportable {
        year: 2022,
        day: 13,
        part: 2.into(),
        result: TaskResult::Todo,
    })
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct ProtoList(#[serde(with = "either::serde_untagged")] Either<usize, Vec<ProtoList>>);

#[derive(Debug, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Data(usize),
}

impl From<ProtoList> for Packet {
    fn from(v: ProtoList) -> Self {
        match v.0 {
            Either::Left(v) => Self::Data(v),
            Either::Right(v) => Self::List(v.into_iter().map(|x| x.into()).collect_vec()),
        }
    }
}

pub enum PacketOrdering {
    Correct,
    Undecided,
    Bad,
}

impl Packet {
    fn correct_order(self, other: Self) -> PacketOrdering {
        match (self, other) {
            (Self::Data(l0), Self::Data(r0)) => {
                debug!("Comparing {l0} and {r0}");
                match l0.cmp(&r0) {
                    std::cmp::Ordering::Less => {
                        debug!("Left side smaller, ordering correct");
                        PacketOrdering::Correct
                    }
                    std::cmp::Ordering::Equal => {
                        debug!("Could not determine order");
                        PacketOrdering::Undecided
                    }
                    std::cmp::Ordering::Greater => {
                        debug!("Right side smaller, ordering bad");
                        PacketOrdering::Bad
                    }
                }
            }
            (Self::Data(l0), Self::List(r0)) => {
                Self::List(vec![Self::Data(l0)]).correct_order(Self::List(r0))
            }
            (Self::List(l0), Self::Data(r0)) => {
                Self::List(l0).correct_order(Self::List(vec![Self::Data(r0)]))
            }
            (Self::List(l0), Self::List(r0)) => {
                let max_items = l0.len().max(r0.len());
                l0.into_iter()
                    .map(Option::Some)
                    .chain(std::iter::repeat(None))
                    .zip(
                        r0.into_iter()
                            .map(Option::Some)
                            .chain(std::iter::repeat(None)),
                    )
                    .take(max_items)
                    .map(|(l0, r0)| {
                        let (l0, r0) = match (l0, r0) {
                            (Some(l0), Some(r0)) => (l0, r0),
                            (None, None) => {
                                debug!("Lists terminated, undecided on this");
                                return PacketOrdering::Undecided;
                            }
                            (Some(_), None) => {
                                debug!("Rightside ran out of items, incorrect");
                                return PacketOrdering::Bad;
                            }
                            (None, Some(_)) => {
                                debug!("Leftside ran out of items, correct");
                                return PacketOrdering::Correct;
                            }
                        };
                        debug!("Need to compare list value");
                        l0.correct_order(r0)
                    })
                    .fold_while(PacketOrdering::Undecided, |out, item| match (out, item) {
                        (PacketOrdering::Bad, _) => FoldWhile::Done(PacketOrdering::Bad),
                        (PacketOrdering::Correct, _) => FoldWhile::Done(PacketOrdering::Correct),
                        (_, PacketOrdering::Bad) => FoldWhile::Done(PacketOrdering::Bad),
                        (_, PacketOrdering::Correct) => FoldWhile::Done(PacketOrdering::Correct),
                        (PacketOrdering::Undecided, v) => FoldWhile::Continue(v),
                    })
                    .into_inner()
            }
        }
    }
}
