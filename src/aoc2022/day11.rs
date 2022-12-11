use color_eyre::Report;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let input = read_file_chunks("aoc2022/day11.txt", "Monkey ").await?;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in input {
        trace!("Monkey: {line:?}");
        let monkey = line.parse()?;
        debug!("Monkey: {monkey:?}");
        monkeys.push(monkey);
    }

    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            let monkey = monkeys.get_mut(monkey).unwrap();
            let (true_list, false_list) = monkey.inspect();
            let monkey = monkey.clone();
            let true_monkey = monkeys.get_mut(monkey.if_true_to).unwrap();
            true_monkey.items.extend_from_slice(&true_list);
            let false_monkey = monkeys.get_mut(monkey.if_false_to).unwrap();
            false_monkey.items.extend_from_slice(&false_list);
        }
    }

    let most_active_monkey = {
        let Some((id, _)) = monkeys.iter().enumerate().max_by(|(_, monkey_a), (_, monkey_b)| {
            monkey_a.items_inspected.cmp(&monkey_b.items_inspected)
        }) else { panic!("could not find best monkey") };
        monkeys.remove(id)
    };
    let other_most_active_monkey = {
        let Some((id, _)) = monkeys.iter().enumerate().max_by(|(_, monkey_a), (_, monkey_b)| {
            monkey_a.items_inspected.cmp(&monkey_b.items_inspected)
        }) else { panic!("could not find best monkey") };
        monkeys.remove(id)
    };

    Ok(Reportable {
        year: 2022,
        day: 11,
        part: 1.into(),
        result: TaskResult::U128(
            most_active_monkey.items_inspected * other_most_active_monkey.items_inspected,
        ),
    })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let input = read_file_chunks("aoc2022/day11.txt", "Monkey ").await?;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in input {
        trace!("Monkey: {line:?}");
        let monkey = line.parse()?;
        debug!("Monkey: {monkey:?}");
        monkeys.push(monkey);
    }

    let common_modulus = monkeys
        .iter()
        .map(|x| match x.test {
            Test::DivBy(v) => v as u128,
        })
        .fold(1, |f, m| f * m);

    for _ in 0..10000 {
        for monkey in 0..monkeys.len() {
            let monkey = monkeys.get_mut(monkey).unwrap();
            let (true_list, false_list) = monkey.extreme_inspect(common_modulus);
            let monkey = monkey.clone();
            let true_monkey = monkeys.get_mut(monkey.if_true_to).unwrap();
            true_monkey.items.extend_from_slice(&true_list);
            let false_monkey = monkeys.get_mut(monkey.if_false_to).unwrap();
            false_monkey.items.extend_from_slice(&false_list);
        }
    }

    let most_active_monkey = {
        let Some((id, _)) = monkeys.iter().enumerate().max_by(|(_, monkey_a), (_, monkey_b)| {
            monkey_a.items_inspected.cmp(&monkey_b.items_inspected)
        }) else { panic!("could not find best monkey") };
        monkeys.remove(id)
    };
    let other_most_active_monkey = {
        let Some((id, _)) = monkeys.iter().enumerate().max_by(|(_, monkey_a), (_, monkey_b)| {
            monkey_a.items_inspected.cmp(&monkey_b.items_inspected)
        }) else { panic!("could not find best monkey") };
        monkeys.remove(id)
    };

    Ok(Reportable {
        year: 2022,
        day: 11,
        part: 2.into(),
        result: TaskResult::U128(
            most_active_monkey.items_inspected * other_most_active_monkey.items_inspected,
        ),
    })
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<u128>,
    operation: Operation,
    test: Test,
    if_true_to: usize,
    if_false_to: usize,
    items_inspected: u128,
}

impl Monkey {
    fn inspect(&mut self) -> (Vec<u128>, Vec<u128>) {
        trace!("Monkey {}", self.id);
        let mut true_list = Vec::new();
        let mut false_list = Vec::new();
        while let Some(item) = self.items.pop() {
            self.items_inspected += 1;
            trace!("Monkey inspects an item with a worry level of {item}");
            let item = ((match self.operation {
                Operation::Add(v) => item + v,
                Operation::AddSelf => item * 2,
                Operation::Mul(v) => item * v,
                Operation::MulSelf => item * item,
            } as f64)
                / 3.0)
                .floor() as u128;
            trace!("Worry Level changed to {item}");
            match self.test {
                Test::DivBy(v) => {
                    if item % v == 0 {
                        trace!("Monkey throws item to {}", self.if_true_to);
                        true_list.push(item);
                    } else {
                        trace!("Monkey throws item to {}", self.if_false_to);
                        false_list.push(item);
                    }
                }
            }
        }
        (true_list, false_list)
    }
    fn extreme_inspect(&mut self, common_modulus: u128) -> (Vec<u128>, Vec<u128>) {
        trace!("Monkey {}", self.id);
        let mut true_list = Vec::new();
        let mut false_list = Vec::new();
        while let Some(item) = self.items.pop() {
            self.items_inspected += 1;
            trace!("Monkey inspects an item with a worry level of {item}");
            let item = match self.operation {
                Operation::Add(v) => item + v,
                Operation::AddSelf => item * 2,
                Operation::Mul(v) => item * v,
                Operation::MulSelf => item * item,
            } % common_modulus;
            trace!("Worry Level changed to {item}");
            match self.test {
                Test::DivBy(v) => {
                    if item % v == 0 {
                        trace!("Monkey throws item to {}", self.if_true_to);
                        true_list.push(item);
                    } else {
                        trace!("Monkey throws item to {}", self.if_false_to);
                        false_list.push(item);
                    }
                }
            }
        }
        (true_list, false_list)
    }
}

impl FromStr for Monkey {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: [&str; 6] = s.split('\n').collect_vec().try_into().unwrap();
        let id: usize = Regex::new("Monkey (?P<id>\\d+):")
            .unwrap()
            .captures(lines[0])
            .unwrap()
            .name("id")
            .unwrap()
            .as_str()
            .parse()?;
        trace!("monkey id: {id:?}");
        let items = Regex::new("Starting items: (?P<items>(\\d+(, )?)+)")
            .unwrap()
            .captures(lines[1])
            .unwrap()
            .name("items")
            .unwrap()
            .as_str();
        let items: Vec<u128> = items.split(", ").map(|x| x.parse().unwrap()).collect_vec();
        trace!("monkey items: {items:?}");
        let op = Regex::new("Operation: new = old (?P<op>[+*]) (?P<val>(\\d+|old))")
            .unwrap()
            .captures(lines[2])
            .unwrap();
        let (operation, op_val): (&str, &str) = (
            op.name("op").unwrap().as_str(),
            op.name("val").unwrap().as_str(),
        );
        let operation = match operation {
            "+" => match op_val {
                "old" => Operation::AddSelf,
                v => Operation::Add(v.parse()?),
            },
            "*" => match op_val {
                "old" => Operation::MulSelf,
                v => Operation::Mul(v.parse()?),
            },
            _ => unreachable!(),
        };
        trace!("monkey op: {operation:?}");
        let test: u128 = Regex::new("Test: divisible by (?P<val>\\d+)")
            .unwrap()
            .captures(lines[3])
            .unwrap()
            .name("val")
            .unwrap()
            .as_str()
            .parse()?;
        let test = Test::DivBy(test);
        trace!("monkey test: {test:?}");
        let if_true_to: usize = Regex::new("If true: throw to monkey (?P<monkey>\\d+)")
            .unwrap()
            .captures(lines[4])
            .unwrap()
            .name("monkey")
            .unwrap()
            .as_str()
            .parse()?;
        trace!("monkey test true: {if_true_to:?}");
        let if_false_to: usize = Regex::new("If false: throw to monkey (?P<monkey>\\d+)")
            .unwrap()
            .captures(lines[5])
            .unwrap()
            .name("monkey")
            .unwrap()
            .as_str()
            .parse()?;
        trace!("monkey test false: {if_false_to:?}");
        Ok(Self {
            id,
            items,
            operation,
            test,
            if_false_to,
            if_true_to,
            items_inspected: 0,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Add(u128),
    AddSelf,
    Mul(u128),
    MulSelf,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Test {
    DivBy(u128),
}
