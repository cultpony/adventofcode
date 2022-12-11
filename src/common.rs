pub mod matrix;

use std::{borrow::Cow, pin::Pin};

pub use color_eyre::eyre::eyre as report;
use color_eyre::Report;
pub use color_eyre::{eyre::Context, Result};
pub use rayon::prelude::*;
pub use tokio_stream::StreamExt;
pub use tracing::{debug, error, info, trace, warn};

use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio_stream::Stream;

#[tracing::instrument]
pub async fn read_file_lines(filename: &str) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>> {
    let file = tokio::fs::File::open(filename).await?;
    let lines = tokio::io::BufReader::new(file);
    let lines = tokio_stream::wrappers::LinesStream::new(lines.lines());
    Ok(Box::pin(lines.map(|f| f.unwrap())))
}

#[tracing::instrument]
pub async fn read_file_chars<'a>(filename: &str) -> Result<Cow<'a, [u8]>> {
    let file = tokio::fs::File::open(filename).await?;
    let mut chars = tokio::io::BufReader::new(file);
    let mut buf = Vec::new();
    chars.read_to_end(&mut buf).await?;
    let buf = Cow::from(buf);
    Ok(buf)
}

pub async fn read_file_chunks(filename: &str, chunk_start: &str) -> Result<Vec<String>> {
    let mut chunk_buf: Vec<String> = Vec::new();
    let mut chunks: Vec<String> = Vec::new();
    let mut lines = read_file_lines(filename).await?;
    while let Some(line) = lines.next().await {
        if line.starts_with(chunk_start) {
            chunk_buf.clear();
            chunk_buf.push(line);
            while let Some(line) = lines.next().await {
                if line.is_empty() {
                    chunks.push(chunk_buf.join("\n"));
                    break;
                }
                chunk_buf.push(line);
            }
        }
    }
    if !chunk_buf.is_empty() {
        chunks.push(chunk_buf.join("\n"));
        chunk_buf.clear();
    }
    Ok(chunks)
}

pub fn skip_empty_lines<F: Stream<Item = String> + 'static + Send>(
    f: F,
) -> Pin<Box<dyn Stream<Item = String> + Send>> {
    Box::pin(f.filter(|f| !f.is_empty()))
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskResult {
    String(String),
    Usize(usize),
    Isize(isize),
    I128(i128),
    U128(u128),
    I64(i64),
    U64(u64),
    I32(i32),
    U32(u32),
    I16(i16),
    U16(u16),
    I8(i8),
    U8(u8),
    /// Task not yet implemented
    Todo,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reportable {
    pub(crate) year: i16,
    pub(crate) day: i8,
    pub(crate) part: TaskPart,
    pub(crate) result: TaskResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPart {
    Part1,
    Part2,
}

impl From<u8> for TaskPart {
    fn from(u: u8) -> Self {
        match u {
            1 => TaskPart::Part1,
            2 => TaskPart::Part2,
            _ => panic!("invalid task part"),
        }
    }
}

impl From<TaskPart> for u8 {
    fn from(val: TaskPart) -> Self {
        match val {
            TaskPart::Part1 => 1,
            TaskPart::Part2 => 2,
        }
    }
}

impl std::str::FromStr for TaskPart {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(TaskPart::Part1),
            "2" => Ok(TaskPart::Part2),
            v => Err(report!("Invalid task part {v:?}")),
        }
    }
}

impl std::fmt::Display for TaskPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPart::Part1 => f.write_str("part 1"),
            TaskPart::Part2 => f.write_str("part 2"),
        }
    }
}

#[derive(clap::Parser, Debug)]
pub struct TaskConfig {
    /// Which day to run, if not specified runs all days
    #[arg(short, long)]
    pub(crate) day: Option<u8>,
    /// Which parts to run. If this is specified without day, runs the given parts of all days
    #[arg(short, long)]
    pub(crate) part: Option<TaskPart>,
}

impl TaskConfig {
    pub fn is(&self, day: u8, part: u8) -> bool {
        let is_day = match self.day {
            None => true,
            Some(v) => v == day,
        };
        let is_part = match self.part {
            None => true,
            Some(v) => {
                let v: u8 = v.into();
                v == part
            }
        };
        is_day && is_part
    }
}
