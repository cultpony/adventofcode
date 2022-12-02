use tracing::Instrument;

use crate::*;

mod day1;
mod day2;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskResult {
    I128(i128),
    U128(u128),
    I64(i64),
    U64(u64),
    I32(i32),
    U32(u32),
    /// Task not yet implemented
    Todo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reportable {
    year: i16,
    day: i8,
    part: TaskPart,
    result: TaskResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPart {
    Part1,
    Part2,
}

impl From<u8> for TaskPart {
    fn from(u: u8) -> Self {
        match u {
            1 => TaskPart::Part1,
            2 => TaskPart::Part2,
            _ => panic!("invalid task part"),
        }
    }
}

impl std::fmt::Display for TaskPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPart::Part1 => f.write_str("part 1"),
            TaskPart::Part2 => f.write_str("part 2"),
        }
    }
}

#[tracing::instrument]
pub async fn main() -> Result<()> {
    let start = tokio::time::Instant::now();
    let mut set = tokio::task::JoinSet::new();
    set.spawn(day1::part1().instrument(tracing::info_span!("aoc2022d1p1")));
    set.spawn(day1::part2().instrument(tracing::info_span!("aoc2022d1p2")));
    set.spawn(day2::part1().instrument(tracing::info_span!("aoc2022d2p1")));
    set.spawn(day2::part2().instrument(tracing::info_span!("aoc2022d2p2")));
    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        let res = res??;
        results.push(res);
        info!(
            "Finished Task aoc{}/day {}/{}: {:?}",
            res.year, res.day, res.part, res.result
        );
    }
    results.sort();
    for res in results {
        info!(
            "Result aoc{}/day {}/{}: {:?}",
            res.year, res.day, res.part, res.result
        );
    }
    let time_taken = start.elapsed();
    info!("Took {:.5} secs", time_taken.as_secs_f64());
    Ok(())
}
