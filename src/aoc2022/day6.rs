use std::collections::HashSet;

use itermore::IterArrayWindows;
use itertools::Itertools;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let input = read_file_chars("aoc2022/day6.txt").await?;

    Ok(Reportable {
        year: 2022,
        day: 6,
        part: 1.into(),
        result: TaskResult::Usize(find_preamble::<4>(&input)),
    })
}

fn find_preamble<const PREAMBLE_SIZE: usize>(input: &[u8]) -> usize {
    let input: Vec<(usize, [&u8; PREAMBLE_SIZE])> = input
        .iter()
        .take_while(|x| (**x as char).is_alphabetic())
        .array_windows()
        .enumerate()
        .collect_vec();

    input
        .iter()
        .find(|(aidx, n)| {
            let is_only_uniq =
                HashSet::<u8>::from_iter(n.iter().map(|x| **x)).len() == PREAMBLE_SIZE;
            trace!("Considering tuple {aidx} {n:?} == {is_only_uniq}");
            is_only_uniq
        })
        .map(|(aidx, _)| *aidx)
        .expect("must have preamble")
        + PREAMBLE_SIZE // preamble size + 4
}

#[cfg(test)]
#[test]
#[tracing_test::traced_test]
fn test_preamble_finding() {
    assert_eq!(find_preamble::<4>(b"bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(find_preamble::<4>(b"nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(find_preamble::<4>(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(find_preamble::<4>(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let input = read_file_chars("aoc2022/day6.txt").await?;

    Ok(Reportable {
        year: 2022,
        day: 6,
        part: 2.into(),
        result: TaskResult::Usize(find_preamble::<14>(&input)),
    })
}
