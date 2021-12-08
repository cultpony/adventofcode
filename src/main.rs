
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

#[cfg(feature = "run_aoc2020")]
mod aoc2020;
#[cfg(feature = "run_aoc2021")]
mod aoc2021;

pub fn prologue<T: ToString>(p: T) {
    println!("==== ==== ====");
    println!("==== {} ====", p.to_string());
}

pub fn epilogue() {
    println!("==== ==== ====\n\n");
}

pub fn read_file_lines(filename: &str) -> Result<Vec<String>> {
    use std::str::FromStr;
    let filename = std::path::PathBuf::from_str(filename).context(Infallible)?;
    let data = std::fs::read_to_string(filename.clone()).context(Io{filename})?;
    let data = data.split("\n").map(|x| x.to_owned()).collect();
    Ok(data)
}

pub fn read_file_lines_nenl(filename: &str) -> Result<Vec<String>> {
    read_file_lines(filename).and_then(|mut x| {
        assert_eq!("", x[x.len()-1]);
        x.pop();
        Ok(x)
    })
}

#[macro_export]
macro_rules! time_func {
    ($c:tt) => {
        let start = std::time::Instant::now();
        $c;
        let end = std::time::Instant::now();
        let dur = end.duration_since(start);
        let dur = chrono::Duration::from_std(dur).unwrap();
        println!("-- Time taken: {:03}µs -- ", dur.num_microseconds().unwrap());
    };
    ($c:expr) => {
        time_func!({$c})
    }
}

#[macro_export]
macro_rules! time_func_rft {
    ($file:tt, $reader:ident, $c:ident, $tries:expr) => {
        let file = $reader($file)?;
        let mut durl = Vec::new();
        let mean = |list: &[chrono::Duration]| -> chrono::Duration {
            let sum: chrono::Duration = list.iter().fold(chrono::Duration::nanoseconds(0), |acc, r| {
                acc + *r
            });
            sum / list.len() as i32
        };
        let median = |list: &[chrono::Duration]| -> chrono::Duration {
            let mut list = list.to_vec();
            list.sort();
            let len = list.len();
            let mid = len / 2;
            if len % 2 == 0 {
                mean(&list[(mid - 1)..(mid + 1)])
            } else {
                list[mid]
            }
        };
        let mut trial = |file: &[String]| {
            let file = file.to_vec();
            let start = std::time::Instant::now();
            let r = {
                $c(file).unwrap()
            };
            let end = std::time::Instant::now();
            let dur = end.duration_since(start);
            let dur = chrono::Duration::from_std(dur).unwrap();
            durl.push(dur);
            r
        };
        let mut r = trial(&file);
        assert!($tries > 0, "Cannot do 0 tries");
        for _ in 0..$tries {
            r = trial(&file);
        }
        let durmax = durl.iter().max().unwrap();
        let durmin = durl.iter().min().unwrap();
        let durmean = mean(&durl);
        let durmed = median(&durl);
        println!("-- Trials: {} -- ", $tries);
        println!("{}", r);
        println!("-- Time taken (MIN): {:03}µs -- ", durmin.num_microseconds().unwrap());
        println!("-- Time taken (AVG): {:03}µs -- ", durmean.num_microseconds().unwrap());
        println!("-- Time taken (MED): {:03}µs -- ", durmed.num_microseconds().unwrap());
        println!("-- Time taken (MAX): {:03}µs -- ", durmax.num_microseconds().unwrap());
    };
    ($ff:expr, $c:expr) => {
        time_func!({$ff}, read_file_lines_nenl, {$c}, 1000)
    }
}


pub fn main() -> Result<()> {
    env_logger::init();

    #[cfg(feature = "run_aoc2020")]
    aoc2020::main()?;
    #[cfg(feature = "run_aoc2021")]
    aoc2021::main()?;
    Ok(())
}