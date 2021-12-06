
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

#[cfg(feature = "aoc2020")]
mod aoc2020;
#[cfg(feature = "aoc2021")]
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
        let r = {
            $c
        };
        let end = std::time::Instant::now();
        let dur = end.duration_since(start);
        let dur = chrono::Duration::from_std(dur).unwrap();
        println!("-- Time taken: {:03}µs -- ", dur.num_microseconds().unwrap());
        r
    };
    ($c:expr) => {
        time_func!({$c})
    }
}

pub fn main() -> Result<()> {
    env_logger::init();

    #[cfg(feature = "aoc2020")]
    aoc2020::main()?;
    #[cfg(feature = "aoc2021")]
    aoc2021::main()?;
    Ok(())
}