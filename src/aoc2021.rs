use crate::*;

pub fn main() -> Result<()> {
    prologue("AOC1");
    time_func!(aoc1_1()?);
    time_func!(aoc1_2()?);

    epilogue();

    Ok(())
}

fn aoc1_1() -> Result<()> {
    let input: Vec<u32> = read_file_lines_nenl("./aoc2021/aoc_1_1.txt")?.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let test = vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263
    ];
    assert_eq!(7, aoc1_1c(&test));
    let result = aoc1_1c(&input);
    println!("Number of increases: {}", result);
    Ok(())
}

fn aoc1_1c(f: &[u32]) -> u32 {
    let result: (Option<u32>, u32) = f.iter()
        .fold((None, 0), |x, y| {
            match x.0 {
                Some(x0) => {
                    if *y > x0 {
                        //println!("{} > {}", y, x0);
                        (Some(*y), x.1 + 1)
                    } else {
                        //println!("{} < {}", y, x0);
                        (Some(*y), x.1)
                    }
                },
                None => {
                    (Some(*y), 0)
                }
            }
        });
    result.1
}

fn aoc1_2() -> Result<()> {
    let input: Vec<u32> = read_file_lines_nenl("./aoc2021/aoc_1_1.txt")?.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let test = vec! [
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263
    ];
    assert_eq!(5, aoc1_2c(&test));
    let result = aoc1_2c(&input);
    println!("Max slid. Increases: {}", result);
    Ok(())
}

fn aoc1_2c(f: &[u32]) -> u32 {
    let res: Vec<u32> = f.iter().zip(f.iter().skip(1)).zip(f.iter().skip(2))
        .map(|x| (x.0.0, x.0.1, x.1))
        .map(|x| {
            //println!("{} + {} + {}", x.0, x.1, x.2);
            x.0 + x.1 + x.2
        })
        .collect();
    let res = aoc1_1c(&res);
    res
}