pub mod matrix;

use std::{borrow::Cow, pin::Pin};

pub use color_eyre::eyre::eyre as report;
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

pub fn skip_empty_lines<F: Stream<Item = String> + 'static + Send>(
    f: F,
) -> Pin<Box<dyn Stream<Item = String> + Send>> {
    Box::pin(f.filter(|f| !f.is_empty()))
}
