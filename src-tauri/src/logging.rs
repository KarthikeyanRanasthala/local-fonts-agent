use std::path::PathBuf;

use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn setup(log_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(log_dir)?;

    // Daily rotation with the log file named "app.log"
    let file_appender = rolling::daily(log_dir, "app.log");

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(file_appender).with_ansi(false))
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .init();

    tracing::info!("Logging initialized at: {:?}", log_dir);

    Ok(())
}
