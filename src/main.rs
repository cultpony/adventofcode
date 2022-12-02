use std::pin::Pin;

pub use color_eyre::{eyre::Context, Result};
pub use rayon::prelude::*;
use tokio::io::AsyncBufReadExt;
use tokio_stream::{Stream, StreamExt};
use tracing_error::prelude::*;
pub use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::prelude::*;

mod aoc2022;

#[tracing::instrument]
pub async fn read_file_lines(filename: &str) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>> {
    let file = tokio::fs::File::open(filename).await.in_current_span()?;
    let lines = tokio::io::BufReader::new(file);
    let lines = tokio_stream::wrappers::LinesStream::new(lines.lines());
    Ok(Box::pin(lines.map(|f| f.unwrap())))
}

pub fn skip_empty_lines<F: Stream<Item = String> + 'static + Send>(
    f: F,
) -> Pin<Box<dyn Stream<Item = String> + Send>> {
    Box::pin(f.filter(|f| !f.is_empty()))
}

#[tracing::instrument]
#[tokio::main]
pub async fn main() -> Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new("debug"))?;
    
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();

    color_eyre::install()?;

    aoc2022::main().await?;

    Ok(())
}
