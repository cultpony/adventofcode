
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum AppError {
    #[snafu(display("Could not open file {}: {}", filename.display(), source))]
    Io {
        filename: std::path::PathBuf,
        source: std::io::Error,
    },
    Infallible {
        source: std::convert::Infallible,
    },
    Regex {
        source: regex::Error,
    },
    ParseIntError {
        source: std::num::ParseIntError,
    },
    #[snafu(display("Unknown error"))]
    Unknown,
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;

fn main() -> Result<()> {
    prologue("AOC1");
    aoc1_1()?;
    aoc1_2()?;
    
    prologue("AOC2");
    aoc2_1()?;
    aoc2_2()?;

    prologue("AOC3");
    aoc3_1()?;
    aoc3_2()?;

    epilogue();
    Ok(())
}

fn prologue<T: ToString>(p: T) {
    println!("==== ==== ====");
    println!("==== {} ====", p.to_string());
}

fn epilogue() {
    println!("==== ==== ====\n\n");
}

fn read_file_lines(filename: &str) -> Result<Vec<String>> {
    use std::str::FromStr;
    let filename = std::path::PathBuf::from_str(filename).context(Infallible)?;
    let data = std::fs::read_to_string(filename.clone()).context(Io{filename})?;
    let data = data.split("\n").map(|x| x.to_owned()).collect();
    Ok(data)
}

fn aoc1_1() -> Result<()> {
    let input = read_file_lines("./aoc_1_1.txt")?;
    let input: Vec<i32> = input.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    for i in input.clone() {
        for j in input.iter().filter(|x| **x != i) {
            if i + j == 2020 {
                println!("{} + {} = 2020, {} * {} = {}", i, j, i, j, i * j);
                return Ok(())
            }
        }
    }
    Ok(())
}

fn aoc1_2() -> Result<()> {
    let input = read_file_lines("./aoc_1_1.txt")?;
    let input: Vec<i32> = input.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    for i in input.clone() {
        for j in input.clone().iter().filter(|x| **x != i) {
            for k in input.iter().filter(|y| **y != i && **y != *j) {
                if i + j + k == 2020 {
                    println!("{} + {} + {} = 2020, {} * {} * {} = {}", i, j, k, i, j, k, i * j * k);
                    return Ok(())
                }
            }
        }
    }
    Ok(())
}

fn aoc2_1() -> Result<()> {
    let rgx = r#"^(\d+)-(\d+)\s(\w):\s(.*)$"#;
    let input = read_file_lines("./aoc_2_1.txt")?;
    let re = regex::Regex::new(rgx).context(Regex{})?;
    let mut valid = 0;
    let total = input.len();
    for i in input {
        if i == "" {
            continue;
        }
        let cap = re.captures(&i);
        let cap = match cap {
            None => panic!("{} does not match {}", i, rgx),
            Some(cap) => cap,
        };
        let min: i32 = cap[1].parse().context(ParseIntError{})?;
        let max: i32 = cap[2].parse().context(ParseIntError{})?;
        let letter = &cap[3].chars().next().unwrap();
        let password = &cap[4];
        let count = password.chars().fold(0, |acc, x| {
            if x == *letter {
                acc + 1
            } else {
                acc
            }
        });
        if count <= max && count >= min {
            valid += 1;
        }
    }
    println!("Valid passwords: {} of {}", valid, total);
    Ok(())
}


fn aoc2_2() -> Result<()> {
    let rgx = r#"^(\d+)-(\d+)\s(\w):\s(.*)$"#;
    let input = read_file_lines("./aoc_2_1.txt")?;
    let re = regex::Regex::new(rgx).context(Regex{})?;
    let mut valid = 0;
    let total = input.len();
    for i in input {
        if i == "" {
            continue;
        }
        let cap = re.captures(&i);
        let cap = match cap {
            None => panic!("{} does not match {}", i, rgx),
            Some(cap) => cap,
        };
        let idx1: usize = cap[1].parse().context(ParseIntError{})?;
        let idx2: usize = cap[2].parse().context(ParseIntError{})?;
        let letter = &cap[3].chars().next().unwrap();
        let password = &cap[4];
        let pos_a_l = password.chars().nth(idx1 - 1).unwrap_or_default();
        let pos_a = pos_a_l == *letter;
        let pos_b_l = password.chars().nth(idx2 - 1).unwrap_or_default();
        let pos_b = pos_b_l == *letter;
        if (pos_a && !pos_b) || (!pos_a && pos_b) {
            valid += 1;
        }
    }
    println!("Valid passwords: {} of {}", valid, total);
    Ok(())
}

fn aoc3_1() -> Result<()> {
    let tree_count = aoc3_1_slope(1, 3)?;
    println!("Trees: {}", tree_count);
    Ok(())
}

fn aoc3_1_slope(xslope: usize, yslope: usize) -> Result<usize> {
    let input = read_file_lines("./aoc_3_1.txt")?;
    let mut ypos = 0;
    let mut xpos = 0;
    let tree = '#';
    let mut tree_count = 0;
    while (xpos < input.len()) {
        let line = input.iter().cloned().nth(xpos).unwrap_or("".to_string());
        let ch = line.chars().nth(ypos).unwrap_or_default();
        //println!("Walking: {} at {}/{}", ch, xpos, ypos);
        if ch == tree {
            tree_count += 1;
        }
        ypos = (ypos + yslope) % line.len();
        xpos = xpos + xslope;
   }
   Ok(tree_count)
}

fn aoc3_2() -> Result<()> {
    let tree_count: usize = aoc3_1_slope(1, 1)?;
    let tree_count = tree_count * aoc3_1_slope(1, 3)?;
    let tree_count = tree_count * aoc3_1_slope(1, 5)?;
    let tree_count = tree_count * aoc3_1_slope(1, 7)?;
    let tree_count = tree_count * aoc3_1_slope(2, 1)?;
    println!("Result: {}", tree_count);
    Ok(())
}