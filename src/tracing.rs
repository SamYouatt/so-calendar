use std::fs;

use color_eyre::eyre::Result;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn initialise_tracing() -> Result<()> {
    let log_dir = dirs_next::data_dir()
        .expect("unable to find data directory for logging")
        .join("so-calendar")
        .join("logs");
    fs::create_dir_all(&log_dir)?;

    let log_file_path = log_dir.join("tracing");
    let log_file = fs::File::create(log_file_path)?;

    // Try and read RUST_LOG for log level filter or default to info
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new("socal".into(), log_file);

    let tracing_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber)
        .with(ErrorLayer::default())
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

    Ok(())
}
