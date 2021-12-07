use log::{debug, info, trace, warn};
use std::ops::Add;
use std::str::FromStr;
use std::{convert::TryInto, hint::unreachable_unchecked};

use bit_vec::BitVec;

use crate::*;

pub fn main() -> Result<()> {
    prologue("AOC1");
    time_func!(aoc1_1()?);
    time_func!(aoc1_2()?);

    prologue("AOC2");
    time_func!(aoc2_1()?);
    time_func!(aoc2_2()?);

    prologue("AOC3");
    time_func!(aoc3_1()?);

    prologue("AOC4");
    time_func!(aoc4_1()?);

    prologue("AOC5");
    time_func!(aoc5_1()?);

    prologue("AOC6");
    time_func!(aoc6_1()?);

    prologue("AOC7");
    time_func_rft!("./aoc2021/aoc_7_1.txt", read_file_lines_nenl, aoc7_1, 1000);
    time_func_rft!("./aoc2021/aoc_7_1.txt", read_file_lines_nenl, aoc7_2, 1000);

    epilogue();

    Ok(())
}

fn aoc7_1(input: Vec<String>) -> Result<String> {
    let input: Vec<i64> = input[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let depth_try = |depth: i64, inp: &[i64]| -> i64 {
        let mut fuel_spent = 0;
        for i in inp {
            fuel_spent += (i-depth).abs()
        }
        assert!(fuel_spent > 0, "Fuel spent {} at depth {}", fuel_spent, depth);
        fuel_spent
    };

    let max = input.iter().max().unwrap();
    let min = input.iter().min().unwrap();

    let mut best_try = (0, i64::MAX);
    for i in *min..*max {
        let fuel_spent = depth_try(i as i64, &input);
        if fuel_spent < best_try.1 {
            best_try = (i, fuel_spent)
        }
    }

    Ok(format!("Best solution: {} fuel spent at position {}", best_try.1, best_try.0))
}

fn aoc7_2(input: Vec<String>) -> Result<String> {
    let input: Vec<i64> = input[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let depth_func = |f: i64, depth: i64| -> i64 {
        let r = (f-depth).abs();
        let r = (r*r+r)/2;
        r
    };

    let max = input.iter().max().unwrap();
    let min = input.iter().min().unwrap();

    let depth_try = |depth: i64, inp: &[i64]| -> i64 {
        let mut fuel_spent = 0;
        for i in inp {
            fuel_spent += depth_func(*i, depth);
        }
        assert!(fuel_spent > 0, "Fuel spent {} at depth {}", fuel_spent, depth);
        fuel_spent
    };

    assert_eq!(depth_func(16, 5), 66);
    assert_eq!(depth_func(1, 5), 10);
    assert_eq!(depth_func(2, 5), 6);
    assert_eq!(depth_func(0, 5), 15);
    assert_eq!(depth_func(14, 5), 45);

    let mut best_try = (0, i64::MAX);
    for i in *min..*max {
        let fuel_spent = depth_try(i as i64, &input);
        if fuel_spent < best_try.1 {
            best_try = (i, fuel_spent)
        }
    }

    Ok(format!("Best solution: {} fuel spent at position {}", best_try.1, best_try.0))
}

fn aoc6_1() -> Result<()> {
    let input = read_file_lines_nenl("./aoc2021/aoc_6_1.txt")?;

    type FishBucketType = usize;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(C)]
    struct FishBucket {
        day0: FishBucketType,
        day1: FishBucketType,
        day2: FishBucketType,
        day3: FishBucketType,
        day4: FishBucketType,
        day5: FishBucketType,
        day6: FishBucketType,
        day7: FishBucketType,
        day8: FishBucketType,
        pad: FishBucketType,
    }

    impl FishBucket {
        fn new(
            day0: usize,
            day1: usize,
            day2: usize,
            day3: usize,
            day4: usize,
            day5: usize,
            day6: usize,
        ) -> Self {
            Self {
                day0: day0 as FishBucketType,
                day1: day1 as FishBucketType,
                day2: day2 as FishBucketType,
                day3: day3 as FishBucketType,
                day4: day4 as FishBucketType,
                day5: day5 as FishBucketType,
                day6: day6 as FishBucketType,
                day7: 0,
                day8: 0,
                pad: 0,
            }
        }

        #[inline(always)]
        fn fast_step(&mut self) {
            const SHIFT: usize = std::mem::size_of::<FishBucketType>();
            let p = (self as *mut Self as usize + SHIFT) as *mut Self;
            let p: &mut Self = unsafe { &mut *p };
            p.day8 = self.day0;
            p.day6 += self.day0;
            *self = *p
        }

        fn total(&self) -> FishBucketType {
            self.day0
                + self.day1
                + self.day2
                + self.day3
                + self.day4
                + self.day5
                + self.day6
                + self.day7
                + self.day8
        }
    }

    let inp: Vec<&str> = input[0].split(',').collect();
    let (day0, day1, day2, day3, day4, day5, day6) =
        inp.iter().fold((0, 0, 0, 0, 0, 0, 0), |acc, x| match *x {
            "0" => (acc.0 + 1, acc.1, acc.2, acc.3, acc.4, acc.5, acc.6),
            "1" => (acc.0, acc.1 + 1, acc.2, acc.3, acc.4, acc.5, acc.6),
            "2" => (acc.0, acc.1, acc.2 + 1, acc.3, acc.4, acc.5, acc.6),
            "3" => (acc.0, acc.1, acc.2, acc.3 + 1, acc.4, acc.5, acc.6),
            "4" => (acc.0, acc.1, acc.2, acc.3, acc.4 + 1, acc.5, acc.6),
            "5" => (acc.0, acc.1, acc.2, acc.3, acc.4, acc.5 + 1, acc.6),
            "6" => (acc.0, acc.1, acc.2, acc.3, acc.4, acc.5, acc.6 + 1),
            _ => unsafe { unreachable_unchecked() },
        });
    let ocean_base = FishBucket::new(day0, day1, day2, day3, day4, day5, day6);

    let mut ocean = ocean_base.clone();
    let mut oceanl = ocean_base.clone();
    let start = std::time::Instant::now();

    debug!("Running through fish futures");
    for i in 0..80 {
        debug!("Day {}: {} fish", i, ocean.total());
        ocean.fast_step();
    }

    debug!("Running through fish futures extended edition");
    for i in 0..256 {
        debug!("Day {}: {} fish", i, ocean.total());
        oceanl.fast_step();
    }

    let end = std::time::Instant::now();
    println!("Got fish futures: {} fish cnt", ocean.total());
    println!("Got fish futures: {} fish cnt", oceanl.total());

    let dur = end.checked_duration_since(start).unwrap();

    println!("Took {}µs ({}ns)", dur.as_micros(), dur.as_nanos());

    Ok(())
}

fn aoc5_1() -> Result<()> {
    let input = read_file_lines_nenl("./aoc2021/aoc_5_1.txt")?;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    struct Point {
        x: usize,
        y: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Line {
        Vertical { start: Point, end: Point },
        Horizontal { start: Point, end: Point },
        Angled { start: Point, end: Point },
    }

    impl FromStr for Point {
        type Err = AppError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (left, right) = s.split_once(',').unwrap();
            let left = left.parse().expect(&format!("{} is not a digit", left));
            let right = right.parse().expect(&format!("{} is not a digit", right));
            Ok(Self { x: left, y: right })
        }
    }

    impl FromStr for Line {
        type Err = AppError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (start, end) = s.split_once(" -> ").unwrap();
            let start: Point = start.parse()?;
            let end: Point = end.parse()?;
            if start.x == end.x {
                let (start, end) = {
                    if start.y > end.y {
                        trace!("Had to swap horiz line");
                        (end, start)
                    } else {
                        trace!("Got plain line");
                        (start, end)
                    }
                };
                Ok(Line::Horizontal { start, end })
            } else if start.y == end.y {
                let (start, end) = {
                    if start.x > end.x {
                        trace!("Had to swap vertic line");
                        (end, start)
                    } else {
                        trace!("Got plain line");
                        (start, end)
                    }
                };
                Ok(Line::Vertical { start, end })
            } else {
                let (start, end) = {
                    if start.x > end.x {
                        if start.y > end.y {
                            (start, end)
                        } else {
                            (end, start)
                        }
                    } else {
                        if start.y < end.y {
                            (start, end)
                        } else {
                            (end, start)
                        }
                    }
                };
                Ok(Line::Angled { start, end })
            }
        }
    }

    impl Line {
        fn is_horizontal_or_vertical(&self) -> bool {
            match self {
                Self::Vertical { .. } => true,
                Self::Horizontal { .. } => true,
                _ => false,
            }
        }
        fn start(&self) -> &Point {
            match self {
                Self::Vertical { start, .. } => start,
                Self::Horizontal { start, .. } => start,
                Self::Angled { start, .. } => start,
            }
        }
        fn end(&self) -> &Point {
            match self {
                Self::Vertical { end, .. } => end,
                Self::Horizontal { end, .. } => end,
                Self::Angled { end, .. } => end,
            }
        }
    }

    struct PlayingField<
        T: Eq + Ord + Clone + Copy + Default + std::fmt::Debug + Add<u8, Output = T>,
        const N: usize,
        const M: usize,
    > {
        f: Box<[Box<[T; N]>; M]>,
    }

    impl<
            T: Eq + Ord + Clone + Copy + Default + std::fmt::Debug + Add<u8, Output = T>,
            const N: usize,
            const M: usize,
        > PlayingField<T, N, M>
    {
        fn new() -> Self {
            let mut v = Vec::new();
            for i in 0..N {
                v.push(T::default());
            }
            let r: [T; N] = v.try_into().unwrap();
            let mut v = Vec::new();
            for i in 0..M {
                v.push(Box::new(r.clone()));
            }
            let r: [Box<[T; N]>; M] = v.try_into().unwrap();
            Self { f: Box::new(r) }
        }
        fn get(&self, x: usize, y: usize) -> T {
            assert!(y < N);
            assert!(x < M);
            self.f[y][x]
        }
        fn swap(&mut self, x: usize, y: usize, d: T) -> T {
            assert!(y < N);
            assert!(x < M);
            let v = self.f[y][x];
            self.f[y][x] = d;
            v
        }
        fn apply<R: FnMut(T) -> T>(&mut self, x: usize, y: usize, mut d: R) -> (T, T) {
            assert!(y < N);
            assert!(x < M);
            let v = self.f[y][x];
            let nv = d(v);
            self.f[y][x] = nv;
            (v, nv)
        }
        fn fold<Q, R: FnMut(Q, Q) -> Q>(&self, start: Q, mut d: R) -> Q
        where
            T: Into<Q>,
        {
            let mut acc = start;
            for i in 0..N {
                for j in 0..M {
                    acc = d(acc, self.get(i, j).into())
                }
            }
            acc
        }
        fn print(&self) {
            for j in 0..M {
                for i in 0..N {
                    let v = self.get(i, j);
                    if v == T::default() {
                        print!(".");
                    } else {
                        print!("{:?}", v);
                    }
                }
                println!("");
            }
        }
        fn reset(&mut self) {
            for j in 0..M {
                for i in 0..N {
                    self.swap(i, j, T::default());
                }
            }
        }
        fn apply_lines(&mut self, l: &Vec<Line>) {
            for line in l {
                match line {
                    Line::Angled { start, end } => {
                        trace!("Applying dline: {:?}, {:?}", start, end);
                        if start.x > end.x {
                            if start.y > end.y {
                                trace!("L1");
                                trace!("Moving X from {} - {}", start.x, end.x);
                                trace!("Moving Y from {} - {}", start.y, end.y);
                                for i in 0..=(start.x - end.x) {
                                    trace!("Applying to {:?}+{}x{:?}+{}", start.x, i, start.y, i);
                                    self.apply(start.x - i, start.y - i, |n| n.add(1));
                                }
                            } else {
                                trace!("L2");
                                trace!("Moving x from {} - {}", start.x, end.x);
                                trace!("Moving Y from {} - {}", end.y, start.y);
                                for i in 0..=(start.x - end.x) {
                                    trace!("Applying to {:?}-{}x{:?}+{}", start.x, i, start.y, i);
                                    self.apply(start.x - i, start.y + i, |n| n.add(1));
                                }
                            }
                        } else {
                            if start.y > end.y {
                                trace!("L3");
                                trace!("Moving X from {} - {}", end.x, start.x);
                                trace!("Moving Y from {} - {}", start.y, end.y);
                                for i in 0..=(end.x - start.x) {
                                    trace!("Applying to {:?}+{}x{:?}-{}", start.x, i, start.y, i);
                                    self.apply(start.x + i, start.y - i, |n| n.add(1));
                                }
                            } else {
                                trace!("L4");
                                trace!("Moving X from {} - {}", end.x, start.x);
                                trace!("Moving Y from {} - {}", end.y, start.y);
                                for i in 0..=(end.x - start.x) {
                                    trace!("Applying to {:?}-{}x{:?}-{}", start.x, i, start.y, i);
                                    self.apply(start.x + i, start.y + i, |n| n.add(1));
                                }
                            }
                        }
                    }
                    Line::Horizontal { start, end } => {
                        trace!("Applying hline: {:?}, {:?}", start, end);
                        for i in 0..=(end.y - start.y) {
                            self.apply(start.x, start.y + i, |n| n.add(1));
                        }
                    }
                    Line::Vertical { start, end } => {
                        trace!("Applying vline: {:?}, {:?}", start, end);
                        for i in 0..=(end.x - start.x) {
                            self.apply(start.x + i, start.y, |n| n.add(1));
                        }
                    }
                }
            }
        }
    }

    let input: Vec<Line> = input
        .iter()
        .map(|x| {
            x.parse::<Line>()
                .expect(&format!("could not parse as line: {}", x))
        })
        .collect();

    let hvlines: Vec<Line> = input
        .iter()
        .filter(|x| x.is_horizontal_or_vertical())
        .copied()
        .collect();

    println!("Got {} lines either H or V", hvlines.len());

    let max_x = hvlines
        .iter()
        .map(|l: &Line| l.start().x.max(l.end().x))
        .max()
        .unwrap() as usize;
    let max_y = hvlines
        .iter()
        .map(|l: &Line| l.start().y.max(l.end().y))
        .max()
        .unwrap() as usize;

    assert!(max_x < 1000);
    assert!(max_y < 1000);

    println!("Playing field: {}x{}", max_x, max_y);

    let mut playing_field = PlayingField::<u8, 1000, 1000>::new();
    assert_eq!(0, playing_field.get(max_x, max_y));
    assert_eq!(0, playing_field.get(0, max_y));
    assert_eq!(0, playing_field.get(max_x, 0));
    assert_eq!(0, playing_field.get(0, 0));

    println!("PF Self Test Complete");

    {
        let input: Vec<&str> = r#"0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        1,1 -> 3,3
        9,7 -> 7,9"#
            .split('\n')
            .map(|x| x.trim())
            .collect();
        let mut pf = PlayingField::<u8, 10, 10>::new();
        let input: Vec<Line> = input
            .iter()
            .map(|x| {
                x.parse::<Line>()
                    .expect(&format!("could not parse as line: {}", x))
            })
            .collect();
        let hvlines: Vec<Line> = input
            .iter()
            .filter(|x| x.is_horizontal_or_vertical())
            .copied()
            .collect();

        pf.print();

        println!("Applying test lines");

        pf.apply_lines(&hvlines);

        println!("Applied test lines");

        pf.print();
        pf.reset();

        println!("Applying diagonal lines");

        pf.apply_lines(&input);

        println!("Applied test lines");

        pf.print();
    }

    println!("PF Example Test Complete");

    playing_field.apply_lines(&hvlines);

    println!("Completed HV run, counting overlaps");

    let overlaps: usize = playing_field.fold(0usize, |mut acc: usize, v: usize| {
        if v >= 2 {
            acc += 1;
        }
        acc
    });

    println!("Overlaps: {}", overlaps);

    playing_field.reset();

    playing_field.apply_lines(&input);

    let overlaps: usize = playing_field.fold(0usize, |mut acc: usize, v: usize| {
        if v >= 2 {
            acc += 1;
        }
        acc
    });

    println!("Overlaps: {}", overlaps);

    Ok(())
}

fn aoc4_1() -> Result<()> {
    let mut input = read_file_lines("./aoc2021/aoc_4_1.txt")?;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Board {
        nums: [[(u8, bool); 5]; 5],
    }

    impl Board {
        fn winner(&self) -> bool {
            for i in self.nums {
                if i.iter().all(|x| x.1) {
                    return true;
                }
            }
            for i in 0..self.nums.len() {
                if self.nums[0][i].1
                    && self.nums[1][i].1
                    && self.nums[2][i].1
                    && self.nums[3][i].1
                    && self.nums[4][i].1
                {
                    return true;
                }
            }
            return false;
        }
        fn mark(&mut self, n: u8) {
            self.nums.iter_mut().for_each(|x| {
                x.iter_mut().for_each(|f| {
                    if f.0 == n {
                        (*f).1 = true
                    }
                })
            })
        }
        fn reset(&mut self) {
            self.nums
                .iter_mut()
                .for_each(|x| x.iter_mut().for_each(|f| (*f).1 = false))
        }
        fn sum(&self) -> u32 {
            let mut o = 0;
            for j in self.nums {
                for k in j {
                    if !k.1 {
                        o += k.0 as u32
                    }
                }
            }
            o
        }
    }

    impl FromStr for Board {
        type Err = AppError;
        fn from_str(f: &str) -> Result<Self> {
            let r: Vec<[(u8, bool); 5]> = f
                .split("\n")
                .into_iter()
                .map(|x| x.split_ascii_whitespace())
                .map(|x| x.into_iter().filter(|x| !x.trim().is_empty()))
                .map(|x| {
                    let mut q = [(0u8, false); 5];
                    for (i, f) in x.enumerate() {
                        assert!(i < 5);
                        q[i] = (
                            f.parse().expect(&format!("error in space: {}:{}", i, f)),
                            false,
                        );
                    }
                    q
                })
                .collect();
            let mut r2: [[(u8, bool); 5]; 5] = [
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
            ];
            //println!("Converting board vec->arr");
            for (i, n) in r.into_iter().enumerate() {
                for (j, m) in n.iter().enumerate() {
                    r2[i][j] = *m;
                }
            }
            Ok(Self { nums: r2 })
        }
    }

    {
        let inp = r#"22 13 17 11  0
                      8  2 23  4 24
                     21  9 14 16  7
                      6 10  3 18  5
                1 12 20 15 19"#;
        let mut b = Board::from_str(inp).unwrap();
        assert_eq!(
            Board {
                nums: [
                    [
                        (22, false),
                        (13, false),
                        (17, false),
                        (11, false),
                        (0, false)
                    ],
                    [(8, false), (2, false), (23, false), (4, false), (24, false)],
                    [
                        (21, false),
                        (9, false),
                        (14, false),
                        (16, false),
                        (7, false)
                    ],
                    [(6, false), (10, false), (3, false), (18, false), (5, false)],
                    [
                        (1, false),
                        (12, false),
                        (20, false),
                        (15, false),
                        (19, false)
                    ],
                ],
            },
            b
        );
        b.mark(22);
        assert!(b.nums[0][0].1, "Mark correct number");
        assert!(!b.winner(), "Not winner yet");
        for i in 0..30 {
            b.mark(i);
        }
        assert!(b.winner(), "Must be winner now");
    }
    println!("Passed selftest");

    let nums: Vec<u8> = input
        .remove(0)
        .split(",")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().expect(&format!("error at: {}", x)))
        .collect();

    let mut boards = Vec::new();
    while input.len() > 0 {
        let inp: Vec<&str> = input
            .iter()
            .take_while(|x| !x.is_empty())
            .map(|x| x.as_ref())
            .collect();
        let inps = inp.len();
        let inp = inp.join("\n");
        if inp.is_empty() {
            input.remove(0);
            continue;
        } else {
            for _ in 0..inps {
                input.remove(0);
            }
        }
        //println!("Parsing board {:?}", inp);
        boards.push(
            inp.parse::<Board>()
                .expect(&format!("parsing board {} failed", inp)),
        );
    }

    println!("Running Bingo");
    let mut last_num = 0;
    for num in nums.clone() {
        last_num = num;
        //println!("Num: {}", num);
        boards.iter_mut().for_each(|b| b.mark(num));
        if boards.iter().any(|x| x.winner()) {
            break;
        }
    }
    let winner = boards.iter().find(|x| x.winner()).unwrap();
    println!("Found winner: {:?}", winner);
    println!("Score: {}", winner.sum() * last_num as u32);

    boards.iter_mut().for_each(|x| x.reset());
    println!("Running Bad Bingo");
    let mut last_num = 0;
    let mut losing_board = None;
    for num in nums {
        last_num = num;
        //println!("Num: {}", num);
        {
            boards.iter_mut().for_each(|b| b.mark(num));
            if losing_board.is_some() {
                losing_board
                    .iter_mut()
                    .for_each(|x: &mut Board| x.mark(num));
            }
        }
        if boards.iter().filter(|x| !x.winner()).count() == 1 && losing_board.is_none() {
            losing_board = Some(boards.iter().find(|x| !x.winner()).unwrap().clone());
            println!("Last board determined, waiting for win: {:?}", losing_board);
        }
        if boards.iter().all(|x| x.winner()) {
            break;
        }
    }
    let losing_board = losing_board.unwrap();
    println!("Found looser: {:?}", losing_board);
    println!("Score: {}", losing_board.sum() * last_num as u32);
    Ok(())
}

fn aoc3_1() -> Result<()> {
    let input = read_file_lines_nenl("./aoc2021/aoc_3_1.txt")?;
    assert_eq!(
        22 * 9,
        aoc3_1c(&vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ])
    );
    let res = aoc3_1c(&input);
    println!("Result: PC={}", res);
    Ok(())
}

fn aoc3_1c<S: ToString>(v: &[S]) -> u64 {
    let width = v[0].to_string().chars().count();
    assert!(width < 8 * 8);
    println!("Width: {}", width);
    let mut one_cnt: Vec<usize> = Vec::with_capacity(width);
    let mut zer_cnt: Vec<usize> = Vec::with_capacity(width);
    for _ in 0..width {
        one_cnt.push(0);
        zer_cnt.push(0);
    }
    for q in v {
        let q = q.to_string();
        assert!(q.len() == width);
        for (i, c) in q.chars().enumerate() {
            if c == '1' {
                one_cnt[width - 1 - i] += 1;
            } else if c == '0' {
                zer_cnt[width - 1 - i] += 1;
            } else {
                unreachable!();
            }
        }
    }
    let mut gamma_rate = BitVec::from_elem(width, false);
    let mut epsilon_rate = BitVec::from_elem(width, false);
    for i in 0..width {
        println!("Bit {}: {} 0, {} 1", i, zer_cnt[i], one_cnt[i]);
        if one_cnt[i] > zer_cnt[i] {
            println!("Bit {}: GR", i);
            gamma_rate.set(i, true);
        } else if zer_cnt[i] > one_cnt[i] {
            println!("Bit {}: ER", i);
            epsilon_rate.set(i, true);
        } else {
            unreachable!()
        }
    }
    assert!(
        {
            let mut g = gamma_rate.clone();
            g.negate();
            g
        } == epsilon_rate
    );
    println!("GR: {:?}", gamma_rate);
    let gamma_rate = gamma_rate.to_bytes();
    let gamma_rate = {
        let mut buf = [0; 8];
        for (i, v) in gamma_rate.iter().enumerate() {
            buf[7 - i] = v.reverse_bits();
        }
        u64::from_be_bytes(buf)
    };
    println!("GR: {:#018b}", gamma_rate);
    println!("ER: {:?}", epsilon_rate);
    let epsilon_rate = epsilon_rate.to_bytes();
    let epsilon_rate = {
        let mut buf = [0; 8];
        for (i, v) in epsilon_rate.iter().enumerate() {
            buf[7 - i] = v.reverse_bits();
        }
        u64::from_be_bytes(buf)
    };
    println!("ER: {:#018b}", epsilon_rate);
    gamma_rate * epsilon_rate
}

fn aoc2_1() -> Result<()> {
    enum Direction {
        Horizontal(u32),
        Vertical(i32),
    }
    let input: Vec<String> = read_file_lines_nenl("./aoc2021/aoc_2_1.txt")?;
    let result = input
        .into_iter()
        .map(|x: String| {
            let (a, b) = x.split_once(" ").unwrap();
            match a {
                "forward" => Direction::Horizontal(b.parse().unwrap()),
                "down" => Direction::Vertical(b.parse().unwrap()),
                "up" => Direction::Vertical(-(b.parse::<i32>().unwrap())),
                _ => unreachable!(),
            }
        })
        .fold((0, 0), |x, y| match y {
            Direction::Horizontal(y) => (x.0 + y, x.1),
            Direction::Vertical(y) => (x.0, x.1 + y),
        });
    println!(
        "Result: {} depth, {} distance = {}",
        result.0,
        result.1,
        result.0 as i32 * result.1
    );
    Ok(())
}

fn aoc2_2() -> Result<()> {
    #[derive(Debug)]
    enum Direction {
        Horizontal(i64),
        Vertical(i64),
    }
    let input: Vec<String> = read_file_lines_nenl("./aoc2021/aoc_2_1.txt")?;
    let result = input
        .into_iter()
        .map(|x: String| {
            //println!("X: {}", x);
            let (a, b) = x.split_once(" ").unwrap();
            match a {
                "forward" => Direction::Horizontal(b.parse().unwrap()),
                "down" => Direction::Vertical(b.parse().unwrap()),
                "up" => Direction::Vertical(-(b.parse::<i64>().unwrap())),
                _ => unreachable!(),
            }
        })
        .fold((0, 0, 0), |(aim, x, y), dir| {
            //println!("aim: {}, x: {}, y: {}, dir: {:?}", aim, x, y, dir);
            match dir {
                Direction::Horizontal(dir) => (aim, x + (aim * dir), y + dir),
                Direction::Vertical(dir) => (aim + dir, x, y),
            }
        });
    println!(
        "Result: {} depth, {} distance = {}",
        result.1,
        result.2,
        result.1 * result.2
    );
    Ok(())
}

fn aoc1_1() -> Result<()> {
    let input: Vec<u32> = read_file_lines_nenl("./aoc2021/aoc_1_1.txt")?
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let test = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, aoc1_1c(&test));
    let result = aoc1_1c(&input);
    println!("Number of increases: {}", result);
    Ok(())
}

fn aoc1_1c(f: &[u32]) -> u32 {
    let result: (Option<u32>, u32) = f.iter().fold((None, 0), |x, y| {
        match x.0 {
            Some(x0) => {
                if *y > x0 {
                    //println!("{} > {}", y, x0);
                    (Some(*y), x.1 + 1)
                } else {
                    //println!("{} < {}", y, x0);
                    (Some(*y), x.1)
                }
            }
            None => (Some(*y), 0),
        }
    });
    result.1
}

fn aoc1_2() -> Result<()> {
    let input: Vec<u32> = read_file_lines_nenl("./aoc2021/aoc_1_1.txt")?
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let test = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(5, aoc1_2c(&test));
    let result = aoc1_2c(&input);
    println!("Max slid. Increases: {}", result);
    Ok(())
}

fn aoc1_2c(f: &[u32]) -> u32 {
    let res: Vec<u32> = f
        .iter()
        .zip(f.iter().skip(1))
        .zip(f.iter().skip(2))
        .map(|x| (x.0 .0, x.0 .1, x.1))
        .map(|x| {
            //println!("{} + {} + {}", x.0, x.1, x.2);
            x.0 + x.1 + x.2
        })
        .collect();
    let res = aoc1_1c(&res);
    res
}
