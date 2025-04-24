//! Initialize ferrishot logging

use crate::CLI;

/// Initialize logging
pub fn initialize() -> bool {
    if CLI.print_log_file_path {
        println!("{}", crate::config::DEFAULT_LOG_FILE_PATH.display());
        true
    } else if CLI.log_stdout {
        env_logger::builder().init();
        false
    } else {
        use std::io::Write as _;
        match std::fs::File::create(std::path::PathBuf::from(&*CLI.log_file)) {
            Ok(file) => env_logger::Builder::new()
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
                .target(env_logger::Target::Pipe(Box::new(file)))
                .filter(None, log::LevelFilter::Error)
                .init(),
            Err(err) => {
                env_logger::builder().init();
                log::error!("Failed to create log file: {err}");
            }
        }
        false
    }
}
