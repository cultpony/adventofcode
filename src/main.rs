
use std::pin::Pin;

pub use color_eyre::{Result, eyre::Context};
pub use tracing::{trace, debug, info, warn, error};
pub use rayon::prelude::*;
use tokio::io::AsyncBufReadExt;
use tokio_stream::{Stream, StreamExt};

mod aoc2022;

#[tracing::instrument]
pub async fn read_file_lines(filename: &str) -> Result<Pin<Box<dyn Stream<Item = String>>>> {
    let file = tokio::fs::File::open(filename).await?;
    let lines = tokio::io::BufReader::new(file);
    let lines = tokio_stream::wrappers::LinesStream::new(lines.lines());
    Ok(Box::pin(lines.map(|f| f.unwrap())))
}

pub async fn skip_empty_lines<F: Stream<Item = String> + 'static>(f: F) -> Pin<Box<dyn Stream<Item = String>>> {
    Box::pin(f.filter(|f| !f.is_empty()))
}

#[tracing::instrument]
#[tokio::main]
pub async fn main() -> Result<()> {    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    color_eyre::install()?;

    aoc2022::main().await?;

    Ok(())
}