//! Provides methods to set the clipboard with image data

/// An argument that can be passed into the program to signal that it should daemonize itself. This
/// can be anything as long as it is unlikely to be passed in by the user by mistake.
pub const CLIPBOARD_DAEMON_ID: &str = "__ferrishot_clipboard_daemon";

use std::process;

/// Set the text content of the clipboard
pub fn set_text() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "linux") {
        process::Command::new(std::env::current_exe()?)
            .arg(CLIPBOARD_DAEMON_ID)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir("/")
            .spawn()?;
    } else {
        arboard::Clipboard::new()?.set_text("Hello, world!")?;
    }

    Ok(())
}

/// Runs a process in the background that provides clipboard access,
/// until the user copies something else into their clipboard.
///
/// # Errors
///
/// - Could not create a clipboard
/// - Could not set the clipboard text
pub fn run_clipboard_daemon() -> Result<(), arboard::Error> {
    use arboard::SetExtLinux as _;
    arboard::Clipboard::new()?
        .set()
        .wait()
        .text("Lol, world!")?;
    Ok(())
}
