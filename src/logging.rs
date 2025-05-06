//! Initialize ferrishot logging

/// Initialize logging
pub fn initialize(cli: &crate::Cli) {
    if cli.log_stderr {
        env_logger::builder()
            .filter_module(cli.log_filter.as_deref().unwrap_or(""), cli.log_level)
            .init();
    } else {
        use std::io::Write as _;

        match std::fs::File::create(std::path::PathBuf::from(&*cli.log_file)) {
            Ok(file) => env_logger::Builder::new()
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "[{time} {level} {module}] {message}",
                        time = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                        level = record.level(),
                        module = record.module_path().unwrap_or("unknown"),
                        message = record.args(),
                    )
                })
                .target(env_logger::Target::Pipe(Box::new(file)))
                .filter(cli.log_filter.as_deref(), cli.log_level)
                .init(),
            Err(err) => {
                env_logger::builder().filter_level(cli.log_level).init();
                log::error!("Failed to create log file: {err}");
            }
        }
    }
}
