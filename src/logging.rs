//! Log errors that occur
use etcetera::BaseStrategy as _;
use std::io::Write as _;

/// Log to the log file, or stdout if log file is not available
pub fn initialize_logging() {
    let log_to_file = || -> Result<(), Box<dyn std::error::Error>> {
        env_logger::builder()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}:{} {} [{}] - {}",
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                    record.level(),
                    record.args()
                )
            })
            .target(env_logger::Target::Pipe(Box::new(std::fs::File::create(
                etcetera::choose_base_strategy()?
                    .cache_dir()
                    .join("ferrishot.log"),
            )?)))
            .filter(None, log::LevelFilter::Error)
            .init();

        Ok(())
    };

    log_to_file().unwrap_or_else(|err| {
        env_logger::builder().init();
        log::warn!("Failed to open the log file: {err}");
    });
}
