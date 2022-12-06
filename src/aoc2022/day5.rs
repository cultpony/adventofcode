use std::str::FromStr;

use color_eyre::Report;
use itertools::Itertools;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let structure = read_file_lines("aoc2022/day5_1.txt").await?;
    let input = read_file_lines("aoc2022/day5_2.txt").await?;
    let mut input = skip_empty_lines(input);
    let mut structure: Harbor = structure.collect::<Vec<String>>().await.try_into()?;
    let mut moves: Vec<Move> = Vec::new();
    while let Some(line) = input.next().await {
        moves.push(line.parse()?);
    }

    let moves: Vec<Vec<SingleMove>> = moves
        .into_par_iter()
        .map(|x| -> Vec<SingleMove> { x.into() })
        .collect();
    let moves: Vec<SingleMove> = moves.into_iter().flatten().collect_vec();

    structure.all_moves(moves);

    Ok(Reportable {
        year: 2022,
        day: 5,
        part: 1.into(),
        result: TaskResult::String(structure.msg()),
    })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let structure = read_file_lines("aoc2022/day5_1.txt").await?;
    let input = read_file_lines("aoc2022/day5_2.txt").await?;
    let mut input = skip_empty_lines(input);
    let mut structure: Harbor = structure.collect::<Vec<String>>().await.try_into()?;
    let mut moves: Vec<Move> = Vec::new();
    while let Some(line) = input.next().await {
        moves.push(line.parse()?);
    }

    structure.all_moves_v9001(moves);

    Ok(Reportable {
        year: 2022,
        day: 5,
        part: 2.into(),
        result: TaskResult::String(structure.msg()),
    })
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regexp =
            regex::Regex::new(r#"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)"#).unwrap();
        let captures = regexp.captures(s).unwrap();
        let (count, from, to): (usize, usize, usize) = (
            captures.name("count").unwrap().as_str().parse()?,
            captures.name("from").unwrap().as_str().parse()?,
            captures.name("to").unwrap().as_str().parse()?,
        );
        Ok(Self { count, from, to })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SingleMove {
    pub from: usize,
    pub to: usize,
}

impl SingleMove {
    /// # Safety
    ///
    /// Must not be called twice on single move, this adjusts offsets to be zero indexed
    pub unsafe fn idx_offset(self) -> Self {
        Self {
            from: self.from - 1,
            to: self.to - 1,
        }
    }
}

impl From<Move> for Vec<SingleMove> {
    fn from(val: Move) -> Self {
        let mut o = Vec::new();
        for _ in 0..(val.count) {
            o.push(SingleMove {
                from: val.from,
                to: val.to,
            })
        }
        o
    }
}

#[derive(Debug)]
pub struct Harbor(Vec<Stack>);

impl<S: Into<String>> TryFrom<Vec<S>> for Harbor {
    type Error = Report;

    fn try_from(s: Vec<S>) -> Result<Self, Self::Error> {
        let lines: Vec<String> = s.into_iter().map(|x| x.into()).collect_vec();
        assert!(
            lines.iter().all(|x| (x.len() + 1) % 4 == 0),
            "invalid harbor format: {lines:?}"
        );

        let harbor_width = lines[0].len() / 4 + 1;

        let mut stack = vec![Stack::default(); harbor_width];

        for line in lines {
            trace!("Loading line {line:?}");
            for (idx, chunk) in line.chars().chunks(4).into_iter().enumerate() {
                let ind = chunk.collect_vec()[1];
                if !ind.is_whitespace() && !ind.is_ascii_digit() {
                    trace!("inserting {ind} chunk at idx {idx}");
                    stack[idx].push(ind);
                }
            }
        }

        let stack = stack.into_iter().map(|x| x.reverse()).collect_vec();

        let harbor = Harbor(stack);
        trace!("Got harbor struct: {harbor:?}");
        Ok(harbor)
    }
}

impl Harbor {
    pub fn single_move(&mut self, a_move: SingleMove) {
        let a_move = unsafe { a_move.idx_offset() };
        assert!(self.0.len() > a_move.from);
        assert!(self.0.len() > a_move.to);
        assert!(a_move.to != a_move.from);
        trace!("Executing move: {a_move:?}");
        let popped = self.0.get_mut(a_move.from).unwrap().pop();
        self.0.get_mut(a_move.to).unwrap().push(popped)
    }

    pub fn all_moves(&mut self, moves: Vec<SingleMove>) {
        for mov in moves {
            self.single_move(mov)
        }
    }

    pub fn all_moves_v9001(&mut self, moves: Vec<Move>) {
        for mov in moves {
            trace!("Executing 9001 move {mov:?}");
            assert!(self.0.len() > mov.from - 1);
            assert!(self.0.len() > mov.to - 1);
            let mut out = Vec::new();
            let stack = self.0.get_mut(mov.from - 1).unwrap();
            for _ in 0..mov.count {
                out.push(stack.pop());
            }
            out.reverse();
            let stack = self.0.get_mut(mov.to - 1).unwrap();
            for out in out {
                stack.push(out);
            }
        }
    }

    pub fn msg(self) -> String {
        let mut s = String::new();
        for m in self.0 {
            s.push(m.read_out());
        }
        s
    }
}

#[derive(Clone, Debug)]
pub struct Stack(Vec<char>);

impl Stack {
    pub fn default() -> Self {
        Self(Vec::new())
    }
    pub fn reverse(mut self) -> Self {
        self.0.reverse();
        self
    }
    pub fn pop(&mut self) -> char {
        assert!(!self.0.is_empty(), "tried to pop from empty stack");
        self.0.pop().unwrap()
    }
    pub fn push(&mut self, new: char) {
        self.0.push(new)
    }
    /// final read of stack
    pub fn read_out(mut self) -> char {
        self.pop()
    }
}
