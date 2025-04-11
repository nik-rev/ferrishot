//! Provides methods to set the clipboard with image data

/// An argument that can be passed into the program to signal that it should daemonize itself. This
/// can be anything as long as it is unlikely to be passed in by the user by mistake.
pub const CLIPBOARD_DAEMON_ID: &str = "__ferrishot_clipboard_daemon";
/// In order to pass along image data from this process onto another, the
/// easiest way to do that is to create a temporary file then read it
const CLIPBOARD_BUFFER_FILE: &str = "__ferrishot_clipboard_buffer";

use std::{
    fs::{self, File},
    io::Write,
    process,
};

/// Set the text content of the clipboard
#[expect(dead_code)]
pub fn set_text(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "linux") {
        process::Command::new(std::env::current_exe()?)
            .arg(CLIPBOARD_DAEMON_ID)
            .arg("text")
            .arg(text)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir("/")
            .spawn()?;
    } else {
        arboard::Clipboard::new()?.set_text(text)?;
    }

    Ok(())
}

/// Set the image content of the clipboard
pub fn set_image(image_data: arboard::ImageData) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "linux") {
        let clipboard_buffer_path = std::env::temp_dir().join(CLIPBOARD_BUFFER_FILE);
        let mut clipboard_buffer_file = File::create(&clipboard_buffer_path)?;
        clipboard_buffer_file.write_all(&image_data.bytes)?;
        process::Command::new(std::env::current_exe()?)
            .arg(CLIPBOARD_DAEMON_ID)
            .arg("image")
            .arg(image_data.width.to_string())
            .arg(image_data.height.to_string())
            .arg(clipboard_buffer_path)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir("/")
            .spawn()?;
    } else {
        arboard::Clipboard::new()?.set_image(image_data)?;
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
///
/// # Panics
///
/// Will panic if the daemon was invoked incorrectly. That's fine because
/// it should only be invoked from this app, never from the outside.
///
/// We expect that the daemon receives 4 arguments:
///
/// 1. ID of the daemon
/// 2. copy type: "image" or "text"
///
/// if copy type is "image" we expect:
///     3. width of image
///     4. height of image
///     5. path to bytes of the image
///
///     The image must be of valid width, height and byte amount
/// if copy type is "text" we expect:
///     3. text content which should be copied to the clipboard
pub fn run_clipboard_daemon() -> Result<(), arboard::Error> {
    use arboard::SetExtLinux as _;
    // skip program name
    let mut args = std::env::args().skip(1);

    assert_eq!(
        args.next().as_deref(),
        Some(CLIPBOARD_DAEMON_ID),
        "this function must be invoked from a daemon process"
    );

    match args.next().expect("has copy type").as_str() {
        "image" => {
            let width = args
                .next()
                .expect("width")
                .parse::<usize>()
                .expect("valid image width");
            let height = args
                .next()
                .expect("height")
                .parse::<usize>()
                .expect("valid image height");
            let bytes: std::borrow::Cow<[u8]> = fs::read(args.next().expect("image path"))
                .expect("image contents")
                .into();

            assert_eq!(args.next(), None, "unexpected extra args");
            assert_eq!(
                width * height * 4,
                bytes.len(),
                "every 4 bytes in `bytes` represents a single RGBA pixel"
            );

            arboard::Clipboard::new()?
                .set()
                .wait()
                .image(arboard::ImageData {
                    width,
                    height,
                    bytes,
                })?;
        },
        "text" => {
            let text = args.next().expect("text");
            assert_eq!(args.next(), None, "unexpected extra args");
            arboard::Clipboard::new()?.set().wait().text(text)?;
        },
        _ => panic!("invalid copy type, expected `image` or `text`"),
    }
    Ok(())
}
