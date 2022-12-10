use color_eyre::Report;
use itertools::Itertools;
use std::str::FromStr;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let mut input = read_file_lines("aoc2022/day9.txt").await?;

    let mut moves: Vec<Move> = Vec::new();
    while let Some(line) = input.next().await {
        moves.push(line.parse()?);
    }

    let mut rope = Rope::default();

    for mov in moves {
        trace!("Making move {mov:?}");
        rope.move_head(mov);
    }

    let pos_list = rope.tail_pos.clone();
    let pos_list = pos_list.into_iter().unique().collect_vec();

    Ok(Reportable {
        year: 2022,
        day: 9,
        part: 1.into(),
        result: TaskResult::Usize(pos_list.len() + 1),
    })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let mut input = read_file_lines("aoc2022/day9.txt").await?;

    let mut moves: Vec<Move> = Vec::new();
    while let Some(line) = input.next().await {
        moves.push(line.parse()?);
    }

    let mut rope = vec![Rope::default(); 10];

    for mov in moves {
        debug!("Making move {mov:?}");
        let (rope, rrope) = rope.split_first_mut().unwrap();
        rope.move_head_rec(mov, rrope);
    }

    let pos_list = rope.last().unwrap().tail_pos.clone();
    let pos_list = pos_list.into_iter().unique().collect_vec();

    Ok(Reportable {
        year: 2022,
        day: 9,
        part: 2.into(),
        result: TaskResult::Usize(pos_list.len() + 1),
    })
}

#[derive(Clone, Default)]
pub struct Rope {
    head: (isize, isize),
    prev_head: (isize, isize),
    tail: (isize, isize),
    tail_pos: Vec<(isize, isize)>,
}

impl std::fmt::Debug for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rope")
            .field("head", &self.head)
            .field("prev_head", &self.prev_head)
            .field("tail", &self.tail)
            .field("tail_pos.len()", &self.tail_pos.len())
            .finish()
    }
}

impl Rope {
    fn move_head_rec(&mut self, dirm: Move, rrope: &mut [Rope]) {
        trace!(
            "Doing recursive move {dirm:?} against {} ropes",
            rrope.len()
        );
        let dirs = dirm.into_single_step();
        for (pos, dir) in dirs.iter().enumerate() {
            let knot_before = self.tail;
            let ress = self.move_head_single(*dir);
            let knot_after = self.tail;
            let res = Move::from_xy((knot_after.0 - knot_before.0, knot_after.1 - knot_before.1))
                .into_opt();
            trace!("Got RES={res:?} RESS={ress:?}, {knot_before:?} -> {knot_after:?}, {rrope:?}");
            assert_eq!(ress, res, "Suggested move matches delta move");
            if !rrope.is_empty() {
                trace!("Splitting rope into first and rest");
                if let Some((nrp, nrrp)) = rrope.split_first_mut() {
                    trace!("Split needed; got {nrp:?} as head of rope");
                    if let Some(res) = res {
                        trace!("Making submove {res:?}");
                        nrp.move_head_rec(res, nrrp);
                        if !nrrp.is_empty() && pos == dirs.len() - 1 {
                            trace!(
                                "Submove {}/{} done on subrope: bottom {nrp:?} top {self:?}",
                                pos + 1,
                                dirs.len()
                            );
                            //assert!(nrrp[0].head == nrp.tail, "Tail is detached: {:?} -> {:?}", nrp.tail, nrrp[0].head);
                        }
                    }
                }
            }
        }
    }
    fn move_head(&mut self, dirm: Move) -> Vec<Move> {
        let dirl = dirm.into_single_step();
        let mut tail_moves = Vec::new();
        for dir in dirl {
            if let Some(v) = self.move_head_single(dir) {
                tail_moves.push(v);
            }
        }
        tail_moves
    }

    fn move_head_single(&mut self, dir: Move) -> Option<Move> {
        let (deltax, deltay) = dir.into_delta();
        trace!("Moving head delta x{deltax} y{deltay}");
        let (nhx, nhy) = (self.head.0 + deltax, self.head.1 + deltay);
        trace!("Got new head position : {nhx} {nhy}");
        self.prev_head = self.head;
        self.head.0 = nhx;
        self.head.1 = nhy;
        assert!(
            (self.head.0 - self.prev_head.0).abs() <= 1,
            "Excessive X head movement: {:?} -> {:?}",
            self.prev_head,
            self.head
        );
        assert!(
            (self.head.1 - self.prev_head.1).abs() <= 1,
            "Excessive Y head movement: {:?} -> {:?}",
            self.prev_head,
            self.head
        );
        trace!(
            "Prev Head: {:?}, Head: {:?}, Tail: {:?}",
            self.prev_head,
            self.head,
            self.tail
        );
        let thd = THDelta::from_delta(self.head, self.tail);
        trace!("Got tail-head delta {thd:?}");
        let td = thd.into_adj();
        trace!("Computed thd adj delta {td:?}");
        if td.0 != 0 || td.1 != 0 {
            if !self.tail_pos.contains(&self.tail) {
                self.tail_pos.push(self.tail);
            }
            self.tail.0 += td.0;
            self.tail.1 += td.1;
            trace!(
                "Adjusting Tail: {:?} -> {:?}",
                self.tail_pos.last().unwrap(),
                self.tail
            );
            Some(Move::from_xy((td.0, td.1)))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum THDelta {
    XN2YP2,
    XN1YP2,
    X00YP2,
    XP1YP2,
    XP2YP2,
    XN2YP1,
    XN1YP1,
    X00YP1,
    XP1YP1,
    XP2YP1,
    XN2Y00,
    XN1Y00,
    X00Y00,
    XP1Y00,
    XP2Y00,
    XN2YN1,
    XN1YN1,
    X00YN1,
    XP1YN1,
    XP2YN1,
    XN2YN2,
    XN1YN2,
    X00YN2,
    XP1YN2,
    XP2YN2,
}

impl THDelta {
    pub fn from_delta(head: (isize, isize), tail: (isize, isize)) -> Self {
        let delta = (tail.0 - head.0, head.1 - tail.1);
        assert!(delta.0 >= -2 && delta.0 <= 2, "X Delta is > 2: {delta:?}");
        assert!(delta.1 >= -2 && delta.1 <= 2, "Y Delta is > 2: {delta:?}");
        trace!("Checking for delta against {delta:?}");
        let deltae = match delta {
            (-2, -2) => THDelta::XN2YP2,
            (-1, -2) => THDelta::XN1YP2,
            (0, -2) => THDelta::X00YP2,
            (1, -2) => THDelta::XP1YP2,
            (2, -2) => THDelta::XP2YP2,
            (-2, -1) => THDelta::XN2YP1,
            (-1, -1) => THDelta::XN1YP1,
            (0, -1) => THDelta::X00YP1,
            (1, -1) => THDelta::XP1YP1,
            (2, -1) => THDelta::XP2YP1,
            (-2, 0) => THDelta::XN2Y00,
            (-1, 0) => THDelta::XN1Y00,
            (0, 0) => THDelta::X00Y00,
            (1, 0) => THDelta::XP1Y00,
            (2, 0) => THDelta::XP2Y00,
            (-2, 1) => THDelta::XN2YN1,
            (-1, 1) => THDelta::XN1YN1,
            (0, 1) => THDelta::X00YN1,
            (1, 1) => THDelta::XP1YN1,
            (2, 1) => THDelta::XP2YN1,
            (-2, 2) => THDelta::XN2YN2,
            (-1, 2) => THDelta::XN1YN2,
            (0, 2) => THDelta::X00YN2,
            (1, 2) => THDelta::XP1YN2,
            (2, 2) => THDelta::XP2YN2,
            _ => unreachable!(),
        };
        trace!("Got delta enum: {deltae:?}");
        deltae
    }
    pub fn into_adj(self) -> (isize, isize) {
        match self {
            THDelta::XN2YP2 => (1, -1),
            THDelta::XN1YP2 => (1, -1),
            THDelta::X00YP2 => (0, -1),
            THDelta::XP1YP2 => (-1, -1),
            THDelta::XP2YP2 => (-1, -1),
            THDelta::XN2YP1 => (1, -1),
            THDelta::XN1YP1 => (0, 0),
            THDelta::X00YP1 => (0, 0),
            THDelta::XP1YP1 => (0, 0),
            THDelta::XP2YP1 => (-1, -1),
            THDelta::XN2Y00 => (1, 0),
            THDelta::XN1Y00 => (0, 0),
            THDelta::X00Y00 => (0, 0),
            THDelta::XP1Y00 => (0, 0),
            THDelta::XP2Y00 => (-1, 0),
            THDelta::XN2YN1 => (1, 1),
            THDelta::XN1YN1 => (0, 0),
            THDelta::X00YN1 => (0, 0),
            THDelta::XP1YN1 => (0, 0),
            THDelta::XP2YN1 => (-1, 1),
            THDelta::XN2YN2 => (1, 1),
            THDelta::XN1YN2 => (1, 1),
            THDelta::X00YN2 => (0, 1),
            THDelta::XP1YN2 => (-1, 1),
            THDelta::XP2YN2 => (-1, 1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Move {
    Up(usize),
    UpRight(usize),
    UpLeft(usize),
    Down(usize),
    DownRight(usize),
    DownLeft(usize),
    Left(usize),
    Right(usize),
    None,
}

impl FromStr for Move {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((dir, mag)) = s.split_once(' ') else {
            return Err(report!("invalid direction: {s}"));
        };
        Ok(match dir {
            "R" => Self::Right(mag.parse()?),
            "L" => Self::Left(mag.parse()?),
            "U" => Self::Up(mag.parse()?),
            "D" => Self::Down(mag.parse()?),
            _ => return Err(report!("invalid pattern {dir}")),
        })
    }
}

impl Move {
    pub fn from_xy(xy: (isize, isize)) -> Self {
        match xy {
            (1, 1) => Move::UpRight(1),
            (1, 0) => Move::Right(1),
            (1, -1) => Move::UpLeft(1),
            (-1, 1) => Move::DownRight(1),
            (-1, 0) => Move::Down(1),
            (-1, -1) => Move::DownLeft(1),
            (0, 1) => Move::Right(1),
            (0, -1) => Move::Left(1),
            (0, 0) => Move::None,
            _ => panic!("invalid move from xy"),
        }
    }
    pub fn into_delta(self) -> (isize, isize) {
        match self {
            Move::Up(f) => (f as isize, 0),
            Move::Down(f) => (-(f as isize), 0),
            Move::Left(f) => (0, -(f as isize)),
            Move::Right(f) => (0, f as isize),
            Move::UpRight(f) => (f as isize, f as isize),
            Move::UpLeft(f) => (f as isize, -(f as isize)),
            Move::DownRight(f) => (-(f as isize), f as isize),
            Move::DownLeft(f) => (-(f as isize), -(f as isize)),
            Move::None => (0, 0),
        }
    }
    pub fn magnitude(self) -> isize {
        (match self {
            Move::Up(f) => f,
            Move::Down(f) => f,
            Move::Left(f) => f,
            Move::Right(f) => f,
            Move::UpRight(f) => f,
            Move::UpLeft(f) => f,
            Move::DownRight(f) => f,
            Move::DownLeft(f) => f,
            Move::None => 0,
        }) as isize
    }
    pub fn into_single_step(self) -> Vec<Move> {
        match self {
            Move::Up(f) => vec![Move::Up(1); f],
            Move::Down(f) => vec![Move::Down(1); f],
            Move::Left(f) => vec![Move::Left(1); f],
            Move::Right(f) => vec![Move::Right(1); f],
            Move::UpRight(f) => vec![Move::UpRight(1); f],
            Move::UpLeft(f) => vec![Move::UpRight(1); f],
            Move::DownRight(f) => vec![Move::UpRight(1); f],
            Move::DownLeft(f) => vec![Move::UpRight(1); f],
            Move::None => Vec::new(),
        }
    }
    pub fn into_opt(self) -> Option<Move> {
        match self {
            Move::None => None,
            v => Some(v),
        }
    }
}
