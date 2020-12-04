
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

    prologue("AOC4");
    aoc4_1()?;

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
            //println!("{} + {} = {}", i, j, i + j);
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
                //println!("{} + {} + {} = {}", i, j, k, i + j + k);
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
        //println!("{} => {}{}", password, letter, count);
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
        //println!("{} => {}={} {}={}", password, pos_a_l, pos_a, pos_b_l, pos_b);
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
    while xpos < input.len() {
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

fn aoc4_1() -> Result<()> {
    let input = read_file_lines("./aoc_4_1.txt")?;
    let mut passport: Vec<(String, String)> = Vec::new();
    let mut passports: Vec<Vec<(String, String)>> = Vec::new();
    for line in input {
        if line == "" {
            passport.sort_by(|a, b| b.0.cmp(&a.0));
            //println!("{:?}", passport);
            passports.push(passport);
            passport = Vec::new();
        } else {
            for fields in line.split(' ') {
                let kv: Vec<&str> = fields.split(':').collect();
                passport.push((kv[0].to_string(), kv[1].to_string()));
            }
        }
    }
    #[derive(Debug)]
    struct HasFields {
        byr: bool,
        iyr: bool,
        eyr: bool,
        hgt: bool,
        hcl: bool,
        ecl: bool,
        pid: bool,
        cid: bool,
    };
    
    let mut valid = 0;
    let mut valid_passports = Vec::new();
    for passport in passports.clone() {
        let mut hf = HasFields{ byr: false, iyr: false, eyr: false, hgt: false , ecl: false, pid: false, cid: false, hcl: false };
        for field in &passport {
            match &*field.0 {
                "byr" => hf.byr = true,
                "iyr" => hf.iyr = true,
                "eyr" => hf.eyr = true,
                "hgt" => hf.hgt = true,
                "hcl" => hf.hcl = true,
                "ecl" => hf.ecl = true,
                "pid" => hf.pid = true,
                "cid" => hf.cid = true,
                _ => (),
            }
        }
        if hf.byr && hf.iyr && hf.eyr && hf.hgt && hf.ecl && hf.pid && hf.hcl {
            //println!("VALID => {:?}", passport);
            valid += 1;
            valid_passports.push(passport);
        } else {
            /*println!("INVALID => {:?}", passport);
            if !hf.byr { println!("\t => MISSING BYR"); } 
            if !hf.iyr { println!("\t => MISSING IYR"); }
            if !hf.eyr { println!("\t => MISSING EYR"); }
            if !hf.hgt { println!("\t => MISSING HGT"); }
            if !hf.ecl { println!("\t => MISSING ECL"); }
            if !hf.pid { println!("\t => MISSING PID"); }
            if !hf.hcl { println!("\t => MISSING HCL"); }
            if !hf.cid { println!("\t => MISSING CID"); }*/
        }
    }
    println!("Valid passports: {}", valid);

    aoc4_2(valid_passports)?;
    Ok(())
}

fn aoc4_2(valid_passports: Vec<Vec<(String, String)>>) -> Result<()> {
    let passports = valid_passports;
    #[derive(Debug)]
    struct HasFields {
        byr: bool,
        iyr: bool,
        eyr: bool,
        hgt: bool,
        hcl: bool,
        ecl: bool,
        pid: bool,
        cid: bool,
    };
    
    let mut valid = 0;
    for passport in passports {
        let mut hf = HasFields{ byr: false, iyr: false, eyr: false, hgt: false , ecl: false, pid: false, cid: false, hcl: false };
        //println!("\t\t\t{:?}", passport);
        for field in &passport {
            match &*field.0 {
                "byr" => {
                    let value = &*field.1;
                    let byr: u16 = value.parse().unwrap_or(0);
                    if byr >= 1920 && byr <= 2002 {
                        hf.byr = true;
                    } else {
                        //println!("\t\tBYR: {}", byr)
                    }
                },
                "iyr" => {
                    let iyr = &*field.1;
                    let iyr: u16 = iyr.parse().unwrap_or(0);
                    if iyr >= 2010 && iyr <= 2020 {
                        hf.iyr = true;
                    } else {
                        //println!("\t\tIYR: {}", iyr)
                    }
                },
                "eyr" => {
                    let eyr = &*field.1;
                    let eyr: u16 = eyr.parse().unwrap_or(0);
                    if eyr >= 2020 &&eyr <= 2030 {
                        hf.eyr = true;
                    } else {
                        //println!("\t\tEYR: {}", eyr)
                    }
                },
                "hgt" => {
                    let hgt = &*field.1;
                    if hgt.contains("in") {
                        let hgt = hgt.trim_end_matches("in");
                        let hgt: u16 = hgt.parse().unwrap_or(0);
                        if hgt >= 59 && hgt <= 76 {
                            hf.hgt = true;
                        } else {
                            //println!("\t\tHGT= {}in", hgt);
                        }
                    } else if hgt.contains("cm") {
                        let hgt = hgt.trim_end_matches("cm");
                        let hgt: u16 = hgt.parse().unwrap_or(0);
                        if hgt >= 150 && hgt <= 193 {
                            hf.hgt = true;
                        } else {
                            //println!("\t\tHGT= {}cm", hgt);
                        }
                    } else {
                        continue;
                    }
                },
                "hcl" => {
                    hf.hcl = regex::Regex::new(r#"#[0-9a-f]{6}"#).unwrap().is_match(&field.1)
                },
                "ecl" => {
                    hf.ecl = match &*field.1 {
                        "amb" => true,
                        "blu" => true,
                        "brn" => true,
                        "gry" => true,
                        "grn" => true,
                        "hzl" => true,
                        "oth" => true,
                        _ => false,
                    }
                },
                "pid" => {
                    hf.pid = regex::Regex::new(r#"\d"#).unwrap().is_match(&field.1) && field.1.len() == 9
                },
                _ => (),
            }
        }
        if hf.byr && hf.iyr && hf.eyr && hf.hgt && hf.ecl && hf.pid && hf.hcl {
            //println!("VALID => {:?}", passport);
            valid += 1;
        } else {
            /*println!("INVALID => {:?}", passport);
            if !hf.byr { println!("\t => INVALID BYR"); } 
            if !hf.iyr { println!("\t => INVALID IYR"); }
            if !hf.eyr { println!("\t => INVALID EYR"); }
            if !hf.hgt { println!("\t => INVALID HGT"); }
            if !hf.ecl { println!("\t => INVALID ECL"); }
            if !hf.pid { println!("\t => INVALID PID"); }
            if !hf.hcl { println!("\t => INVALID HCL"); }
            println!();*/
        }
    }
    println!("Valid passports: {}", valid);
    Ok(())
}