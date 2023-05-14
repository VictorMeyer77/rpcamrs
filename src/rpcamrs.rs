use std::error::Error;
use std::process::{Command, ExitStatus};

pub fn single_capture(
    output: &str,
    encoding: Option<&str>,
    width: Option<u16>,
    height: Option<u16>,
) -> Result<ExitStatus, Box<dyn Error>> {
    Ok(Command::new("sh")
        .arg("-c")
        .arg("libcamera-still")
        .arg("-v")
        .arg("0")
        .arg("-n")
        .arg("--width")
        .arg(width.unwrap_or(0).to_string())
        .arg("--height")
        .arg(height.unwrap_or(0).to_string())
        .arg("-o")
        .arg(output)
        .arg("-e")
        .arg(encoding.unwrap_or("jpg"))
        .status()?)
}
