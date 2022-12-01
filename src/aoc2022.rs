
use color_eyre::eyre::ContextCompat;
use rayon::prelude::IntoParallelRefIterator;

use crate::*;

#[tracing::instrument]
pub async fn main() -> Result<()> {
    let (
        day1_part1,
        day1_part2,
    ) = tokio::join!(
        aoc2022_day1_part1(),
        aoc2022_day1_part2(),
    );
    day1_part1?;
    day1_part2?;
    Ok(())
}

#[tracing::instrument]
pub async fn aoc2022_day1_part2() -> Result<()> {
    let mut elves: Vec<i32> = Vec::new();

    let mut input = read_file_lines("aoc2022/day1_1.txt").await?;

    let mut cur_elf = 0;
    while let Some(v) = input.next().await {
        if v.is_empty() {
            trace!("Elf carries {cur_elf} calories, backpack concluded, pushing on list");
            elves.push(cur_elf);
            cur_elf = 0;
        } else {
            let v: i32 = v.parse().context("not a number on line")?;
            cur_elf += v;
        }
    }

    debug!("Done calculating list");

    let max_elves = 3;
    let mut max_elves_list = Vec::new();

    for _ in 0..max_elves {
        let (max_elf_idx, max_elf) = elves.par_iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).context("no maximum in list")?;
        max_elves_list.push(*max_elf);
        elves.remove(max_elf_idx);
    }

    let max_elf: i32 = max_elves_list.iter().sum();

    info!("Biggest calorie take is {max_elf}");

    Ok(())
}

#[tracing::instrument]
pub async fn aoc2022_day1_part1() -> Result<()> {
    let mut elves: Vec<i32> = Vec::new();

    let mut input = read_file_lines("aoc2022/day1_1.txt").await?;

    let mut cur_elf = 0;
    while let Some(v) = input.next().await {
        if v.is_empty() {
            trace!("Elf carries {cur_elf} calories, backpack concluded, pushing on list");
            elves.push(cur_elf);
            cur_elf = 0;
        } else {
            let v: i32 = v.parse().context("not a number on line")?;
            cur_elf += v;
        }
    }

    debug!("Done calculating list");

    let max_elf = elves.par_iter().max().context("no maximum in list")?;

    info!("Biggest calorie take is {max_elf}");

    Ok(())
}