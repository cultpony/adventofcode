use std::pin::Pin;

pub use color_eyre::{eyre::Context, Result};
pub use tracing::{debug, error, info, trace, warn};
pub use rayon::prelude::*;
pub use tokio_stream::StreamExt;

use tokio::io::AsyncBufReadExt;
use tokio_stream::Stream;

#[tracing::instrument]
pub async fn read_file_lines(filename: &str) -> Result<Pin<Box<dyn Stream<Item = String> + Send>>> {
    let file = tokio::fs::File::open(filename).await?;
    let lines = tokio::io::BufReader::new(file);
    let lines = tokio_stream::wrappers::LinesStream::new(lines.lines());
    Ok(Box::pin(lines.map(|f| f.unwrap())))
}

pub fn skip_empty_lines<F: Stream<Item = String> + 'static + Send>(
    f: F,
) -> Pin<Box<dyn Stream<Item = String> + Send>> {
    Box::pin(f.filter(|f| !f.is_empty()))
}