
use std::collections::HashMap;

use crate::*;

#[cfg(feature = "run_aoc2020")]
pub fn main() -> Result<()> {
    //vess_puzzle();

    #[cfg(any(feature = "aoc2020-day1-part1", feature = "aoc2020-day1-part2"))]
    prologue("AOC1");
    #[cfg(feature = "aoc2020-day1-part1")]
    aoc1_1()?;
    #[cfg(feature = "aoc2020-day1-part2")]
    aoc1_2()?;
    
    #[cfg(any(feature = "aoc2020-day2-part1", feature = "aoc2020-day2-part2"))]
    prologue("AOC2");
    #[cfg(feature = "aoc2020-day2-part1")]
    aoc2_1()?;
    #[cfg(feature = "aoc2020-day2-part2")]
    aoc2_2()?;

    #[cfg(any(feature = "aoc2020-day3-part1", feature = "aoc2020-day3-part2"))]
    prologue("AOC3");
    #[cfg(feature = "aoc2020-day3-part1")]
    aoc3_1()?;
    #[cfg(feature = "aoc2020-day3-part2")]
    aoc3_2()?;

    #[cfg(any(feature = "aoc2020-day4-part1", feature = "aoc2020-day4-part2"))]
    prologue("AOC4");
    #[cfg(any(feature = "aoc2020-day4-part1", feature = "aoc2020-day4-part2"))]
    aoc4_1()?;

    #[cfg(any(feature = "aoc2020-day5-part1", feature = "aoc2020-day5-part2"))]
    prologue("AOC5");
    #[cfg(any(feature = "aoc2020-day5-part1", feature = "aoc2020-day5-part2"))]
    aoc5_1_2()?;

    #[cfg(any(feature = "aoc2020-day6-part1", feature = "aoc2020-day6-part2"))]
    prologue("AOC6");
    #[cfg(any(feature = "aoc2020-day6-part1", feature = "aoc2020-day6-part2"))]
    aoc6_1_2()?;

    #[cfg(any(feature = "aoc2020-day7-part1", feature = "aoc2020-day7-part2"))]
    prologue("AOC7");
    #[cfg(any(feature = "aoc2020-day7-part1", feature = "aoc2020-day7-part2"))]
    aoc7_1_2()?;

    #[cfg(feature = "vess_puzzles")]
    prologue("VESS");
    #[cfg(feature = "vess_puzzles")]
    vess_puzzle();

    epilogue();
    Ok(())
}

#[cfg(feature = "vess_puzzles")]
fn vess_puzzle() {
    let _start = 1075;
    //let start = 1;
    let width = 200;
    //let width = 3;
    let cnt = (width * (width + 1)) / 2 + ((width - 1) * ((width - 1 + 1)) / 2);
    let mut c_width = 1;
    let sol1 = 11582;
    let sol2 = 6882;
    let mut ans1 = 0;
    let mut ans2 = 0;
    println!("cnt: {}", cnt);
    let mut i = 0;
    for k in 0..width*2 {
        for j in 0..(c_width -1) {
            i += 1;
            if j == 0 || j == c_width-2 {
                print!("o{}o", i);
                ans1 = (ans1 + i) % 15356;
            } else {
                print!("i{}i", i);
                ans2 = (ans2 + i) % 15356;
            }
        }
        if k+2 > cnt / 2 {
            c_width -= 1;
        } else {
            c_width += 1;
        }
        println!("\n");
    }
    //assert_eq!(sol1, ans1);
    //assert_eq!(sol2, ans2);
}

#[cfg(feature = "aoc2020-day1-part1")]
fn aoc1_1() -> Result<()> {
    let input = read_file_lines("./aoc2020/aoc_1_1.txt")?;
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

#[cfg(feature = "aoc2020-day1-part2")]
fn aoc1_2() -> Result<()> {
    let input = read_file_lines("./aoc2020/aoc_1_1.txt")?;
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

#[cfg(feature = "aoc2020-day2-part1")]
fn aoc2_1() -> Result<()> {
    let rgx = r#"^(\d+)-(\d+)\s(\w):\s(.*)$"#;
    let input = read_file_lines("./aoc2020/aoc_2_1.txt")?;
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

#[cfg(feature = "aoc2020-day2-part2")]
fn aoc2_2() -> Result<()> {
    let rgx = r#"^(\d+)-(\d+)\s(\w):\s(.*)$"#;
    let input = read_file_lines("./aoc2020/aoc_2_1.txt")?;
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

#[cfg(feature = "aoc2020-day3-part1")]
fn aoc3_1() -> Result<()> {
    let tree_count = aoc3_1_slope(1, 3)?;
    println!("Trees: {}", tree_count);
    Ok(())
}

#[cfg(any(feature = "aoc2020-day3-part1", feature = "aoc2020-day3-part2"))]
fn aoc3_1_slope(xslope: usize, yslope: usize) -> Result<usize> {
    let input = read_file_lines("./aoc2020/aoc_3_1.txt")?;
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

#[cfg(feature = "aoc2020-day3-part2")]
fn aoc3_2() -> Result<()> {
    let tree_count: usize = aoc3_1_slope(1, 1)?;
    let tree_count = tree_count * aoc3_1_slope(1, 3)?;
    let tree_count = tree_count * aoc3_1_slope(1, 5)?;
    let tree_count = tree_count * aoc3_1_slope(1, 7)?;
    let tree_count = tree_count * aoc3_1_slope(2, 1)?;
    println!("Result: {}", tree_count);
    Ok(())
}

#[cfg(any(feature = "aoc2020-day4-part1", feature = "aoc2020-day4-part2"))]
fn aoc4_1() -> Result<()> {
    let input = read_file_lines("./aoc2020/aoc_4_1.txt")?;
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
    }
    
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

#[cfg(any(feature = "aoc2020-day4-part1", feature = "aoc2020-day4-part2"))]
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
        _cid: bool,
    }
    
    let mut valid = 0;
    for passport in passports {
        let mut hf = HasFields{ byr: false, iyr: false, eyr: false, hgt: false , ecl: false, pid: false, _cid: false, hcl: false };
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

#[cfg(any(feature = "aoc2020-day5-part1", feature = "aoc2020-day5-part2"))]
fn aoc5_1_2() -> Result<()> {
    let bsp_idx = |m: i32, u: char, l: char, x: &str| -> i32 {
        let required_steps = (m as f64).log10() as usize; 
        let present_steps = x.chars().filter(|x| *x == u || *x == l).count();
        assert!(present_steps > required_steps, "require {} steps, got {}", required_steps, present_steps);
        let mut idx_lo = 0;
        let mut idx_hi = m - 1;
        for dir in x.chars() {
            if dir == u {
                idx_hi = idx_lo + ((idx_hi - idx_lo) / 2);
                idx_lo = idx_lo;
            } else if dir == l {
                idx_lo = idx_lo + ((idx_hi - idx_lo) / 2 + 1);
                idx_hi = idx_hi;
            } else {
                continue;
            }
            //println!("{} => idx_lo {}, idx_hi {}", dir, idx_lo, idx_hi);
        }
        //println!("finish BSP");
        assert!(idx_lo == idx_hi, "idx_lo {} == idx_hi {}", idx_lo, idx_hi);
        idx_lo
    };
    assert_eq!(44, bsp_idx(128, 'F', 'B', "FBFBBFF"), "Check Seat 44 Code");
    assert_eq!(45, bsp_idx(128, 'F', 'B', "FBFBBFB"), "Check Seat 45 Code");
    assert_eq!(109, bsp_idx(128, 'F', 'B', "BBFBBFB"), "Check Seat 109 Code");
    let row_idx = |x: &str| bsp_idx(128, 'F', 'B', x);
    let col_idx = |x: &str| bsp_idx(8, 'L', 'R', x);
    let seat_id = |x: &str| -> i32 {
        //println!("Running: {}", x);
        let row = row_idx(x);
        let col = col_idx(x);
        row * 8 + col
    };
    assert_eq!(567, seat_id("BFFFBBFRRR"));
    assert_eq!(119, seat_id("FFFBBBFRRR"));
    assert_eq!(820, seat_id("BBFFBBFRLL"));

    let input = read_file_lines_nenl("./aoc2020/aoc_5_1.txt")?;
    let mut highest_seat_id = 0;
    for line in  input.clone() {
        let seat_id = seat_id(&line);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }
    println!("Highest Seat ID: {}", highest_seat_id);

    let mut seats: Vec<bool> = Vec::new();
    for _ in 0..highest_seat_id+1 {
        seats.push(false);
    }
    for line in input.clone() {
        let seat_id = seat_id(&line) as usize;
        seats[seat_id] = true;
    }
    for i in 0..input.len() {
        let cur_seat = seats[i];
        let prv_seat = seats[(i.saturating_sub(1)).max(0)];
        let nxt_seat = seats[(i+1).min(input.len()-1)];
        if !cur_seat && nxt_seat && prv_seat {
            println!("Your seat: {}", i);
            break;
        }
    }
    Ok(())
}

#[cfg(any(feature = "aoc2020-day6-part1", feature = "aoc2020-day6-part2"))]
fn aoc6_1_2() -> Result<()> {
    let input = read_file_lines("./aoc2020/aoc_6_1.txt")?;
    let mut groups : Vec<HashMap<char, u16>> = Vec::new();
    let mut group: HashMap<char, u16> = HashMap::new();
    let mut group_size = 0;
    for line in input {
        if line == "" {
            group.insert('_', group_size);
            group_size = 0; 
            //println!("{:?}", group);
            groups.push(group);
            group = HashMap::new();
        } else {
            group_size += 1;
            for ch in line.chars() {
                if group.contains_key(&ch) {
                    group.entry(ch).and_modify(|x| {
                        *x += 1;
                    });
                } else {
                    group.insert(ch, 1);
                }
            }
        }
    }
    let mut key_count = 0;
    for group in groups.iter() {
        key_count += group.keys().len() - 1;
    }  
    println!("Question types answererd: {}", key_count);
    let mut sum_count = 0; 
    for group in groups {
        let group_size = *group.get(&'_').unwrap();
        let answers: Vec<(&char, &u16)> = group.iter().filter(|(ch, c)| **c >= group_size && **ch != '_').collect();
        let mut sum_group= 0; 
        for _ in answers {
            //println!("answers: {} -> {}", answer.0, answer.1);
            sum_group += 1;
        }
        //println!("Sum: {}", sum_group);
        sum_count += sum_group;
    }
    println!("Unique questions answered: {}", sum_count);
    Ok(())
}

#[cfg(any(feature = "aoc2020-day7-part1", feature = "aoc2020-day7-part2"))]
fn aoc7_1_2() -> Result<()> {
    let _input = read_file_lines_nenl("aoc2020/aoc_7_1.txt")?;
    Ok(())
}