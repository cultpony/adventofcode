use std::{str::FromStr, collections::HashSet};

use itertools::Itertools;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let input = read_file_lines("aoc2022/day3.txt").await?;
    let mut input = skip_empty_lines(input);
    let mut backpacks: Vec<Backpack> = Vec::new();
    while let Some(line) = input.next().await {
        backpacks.push(line.parse()?);
    }

    let result = backpacks.par_iter().map(|b| b.intersection())
        .map(|items| {
            items.iter().take(1).map(|x| x.priority() as u16).sum::<u16>()
        }).sum::<u16>();

    Ok(Reportable { year: 2022, day: 3, part: 1.into(), result: TaskResult::U16(result) })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let input = read_file_lines("aoc2022/day3.txt").await?;
    let mut input = skip_empty_lines(input);
    let mut backpacks: Vec<Backpack> = Vec::new();
    while let Some(line) = input.next().await {
        backpacks.push(line.parse()?);
    }

    let result = backpacks.par_iter().chunks(3)
        .map(|x| -> (&Backpack, &Backpack, &Backpack) {
            assert!(x.len() == 3);
            (x[0], x[1], x[2])
        })
        .map(|(b1, b2, b3)| {
            let b1hs = b1.union_hs();
            let b2hs = b2.union_hs();
            let b3hs = b3.union_hs();
            let group = b1hs.intersection(&HashSet::from_iter(b2hs.intersection(&b3hs).copied())).copied().collect_vec();
            assert!(group.len() == 1);
            trace!("Group badge {:?}", group[0]);
            group[0]
        })
        .map(|group| {
            group.priority() as u16
        })
        .sum::<u16>();

    Ok(Reportable { year: 2022, day: 3, part: 2.into(), result: TaskResult::U16(result) })
}

pub struct Backpack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Backpack {
    pub fn intersection(&self) -> Vec<Item> {
        self.first_compartment.intersection(&self.second_compartment)
    }
    /// All unique items
    pub fn union_hs(&self) -> HashSet<Item> {
        HashSet::from_iter(self.first_compartment.union(&self.second_compartment).into_iter())
    }
}

impl FromStr for Backpack {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() % 2 == 0, "Must have dividable string size but size if odd: {s}");
        let first = &s[0..(s.len() / 2)];
        let second = &s[(s.len() / 2)..];
        assert!(first.len() == second.len());
        Ok(Backpack {
            first_compartment: first.parse()?,
            second_compartment: second.parse()?,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Compartment {
    items: HashSet<Item>,
}

impl FromStr for Compartment {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.is_ascii(), "must be ascii");
        let set = HashSet::from_iter(s.chars().map(|x| -> Item { x.into() }));
        Ok(Self{
            items: set,
        })
    }
}

impl Compartment {
    pub fn intersection(&self, other: &Self) -> Vec<Item> {
        self.items.intersection(&other.items).copied().collect_vec()
    }
    pub fn union(&self, other: &Self) -> Vec<Item> {
        self.items.union(&other.items).copied().collect_vec()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct Item(char);

impl Item {
    pub fn priority(&self) -> u8 {
        assert!(self.0.is_ascii(), "must be ascii input");
        let cha = self.0 as u8;
        if self.0.is_ascii_lowercase() {
            cha - ('a' as u8) + 1
        } else {
            cha - ('A' as u8) + 27
        }
    }
}

impl From<char> for Item {
    fn from(a: char) -> Self {
        assert!(a.is_ascii(), "must be ascii");
        Item(a)
    }
}

#[cfg(test)]
#[test]
pub fn test_verify_item_priority() {
    assert_eq!(Item::from('a').priority(), 1);
    assert_eq!(Item::from('m').priority(), 13);
    assert_eq!(Item::from('z').priority(), 26);
    assert_eq!(Item::from('A').priority(), 27);
    assert_eq!(Item::from('M').priority(), 39);
    assert_eq!(Item::from('Z').priority(), 52);
}

#[cfg(test)]
#[test]
pub fn test_compartment_parsing() -> Result<()> {
    let comp: Compartment = "vJrwpWtwJgWr".parse()?;
    let mut hs = HashSet::new();
    hs.insert(Item('r'));
    hs.insert(Item('g'));
    hs.insert(Item('w'));
    hs.insert(Item('v'));
    hs.insert(Item('J'));
    hs.insert(Item('p'));
    hs.insert(Item('t'));
    hs.insert(Item('W'));
    assert_eq!(comp.items, hs);
    Ok(())
}

#[cfg(test)]
#[test]
pub fn test_backpack_intersection() -> Result<()> {
    assert_eq!(Backpack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp")?.intersection(), vec![Item('p')]);
    assert_eq!(Backpack::from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")?.intersection(), vec![Item('L')]);
    assert_eq!(Backpack::from_str("PmmdzqPrVvPwwTWBwg")?.intersection(), vec![Item('P')]);
    assert_eq!(Backpack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")?.intersection(), vec![Item('v')]);
    assert_eq!(Backpack::from_str("ttgJtRGJQctTZtZT")?.intersection(), vec![Item('t')]);
    assert_eq!(Backpack::from_str("CrZsJsPPZsGzwwsLwLmpwMDw")?.intersection(), vec![Item('s')]);

    assert_eq!(vec![
        Item('p'), Item('L'), Item('P'), Item('v'), Item('t'), Item('s'),
    ].iter().map(|x| x.priority() as u16).sum::<u16>(), 157u16);
    Ok(())
}