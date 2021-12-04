use std::convert::TryInto;
use std::str::FromStr;

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

    epilogue();

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
            self.nums.iter().all(|x| x.iter().all(|y| y.1))
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
                        q[i] = (f.parse().expect(&format!("error in space: {}:{}", i, f)), false);
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
                        (00, false)
                    ],
                    [
                        (08, false),
                        (02, false),
                        (23, false),
                        (04, false),
                        (24, false)
                    ],
                    [
                        (21, false),
                        (09, false),
                        (14, false),
                        (16, false),
                        (07, false)
                    ],
                    [
                        (06, false),
                        (10, false),
                        (03, false),
                        (18, false),
                        (05, false)
                    ],
                    [
                        (01, false),
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

    let nums: Vec<u8> = input.remove(0)
        .split(",")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().expect(&format!("error at: {}", x)))
        .collect();

    let mut boards = Vec::new();
    while input.len() > 0 {
        let inp: Vec<&str> = input.iter().take_while(|x| !x.is_empty()).map(|x| x.as_ref()).collect();
        let inp = inp.join("\n");
        boards.push(inp.parse::<Board>().expect(&format!("parsing board {} failed", inp)));
    }

    for num in nums {
        boards.iter_mut().for_each(|b| b.mark(num));
        if boards.iter().any(|x| x.winner()) {
            break;
        }
    }
    println!("Found winner");

    todo!();
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
