use crate::*;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day13;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[tracing::instrument(skip(tc))]
pub async fn main(tc: TaskConfig) -> Result<()> {
    let start = tokio::time::Instant::now();
    let mut set = tokio::task::JoinSet::new();
    if tc.is(1, 1) {
        set.spawn(day1::part1());
    }
    if tc.is(1, 2) {
        set.spawn(day1::part2());
    }
    if tc.is(2, 1) {
        set.spawn(day2::part1());
    }
    if tc.is(2, 2) {
        set.spawn(day2::part2());
    }
    if tc.is(3, 1) {
        set.spawn(day3::part1());
    }
    if tc.is(3, 2) {
        set.spawn(day3::part2());
    }
    if tc.is(4, 1) {
        set.spawn(day4::part1());
    }
    if tc.is(4, 2) {
        set.spawn(day4::part2());
    }
    if tc.is(5, 1) {
        set.spawn(day5::part1());
    }
    if tc.is(5, 2) {
        set.spawn(day5::part2());
    }
    if tc.is(6, 1) {
        set.spawn(day6::part1());
    }
    if tc.is(6, 2) {
        set.spawn(day6::part2());
    }
    if tc.is(7, 1) {
        set.spawn(day7::part1());
    }
    if tc.is(7, 2) {
        set.spawn(day7::part2());
    }
    if tc.is(8, 1) {
        set.spawn(day8::part1());
    }
    if tc.is(8, 2) {
        set.spawn(day8::part2());
    }
    if tc.is(9, 1) {
        set.spawn(day9::part1());
    }
    if tc.is(9, 2) {
        set.spawn(day9::part2());
    }
    if tc.is(10, 1) {
        set.spawn(day10::part1());
    }
    if tc.is(10, 2) {
        set.spawn(day10::part2());
    }
    if tc.is(11, 1) {
        set.spawn(day11::part1());
    }
    if tc.is(11, 2) {
        set.spawn(day11::part2());
    }
    if tc.is(13, 1) {
        set.spawn(day13::part1());
    }
    if tc.is(13, 2) {
        set.spawn(day13::part2());
    }
    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        let res = res??;
        results.push(res.clone());
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
