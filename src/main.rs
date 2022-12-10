use aoc::TaskConfig;
use clap::Parser;
use tracing_subscriber::prelude::*;

pub mod aoc2022;
pub mod common;
pub use common::*;

#[tracing::instrument]
#[tokio::main]
pub async fn main() -> Result<()> {
    let fmt = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_file(false)
        .with_target(false)
        .with_source_location(false)
        .with_line_number(false)
        .without_time()
        .compact();
    let fmt_layer = tracing_subscriber::fmt::layer()
        .event_format(fmt)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_level(true)
        .with_target(true);
    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .or_else(|_| tracing_subscriber::EnvFilter::try_new("debug"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();

    color_eyre::install()?;

    let tc = TaskConfig::parse();

    aoc2022::main(tc).await?;

    Ok(())
}
