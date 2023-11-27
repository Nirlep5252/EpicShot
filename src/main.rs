use clap::Parser;
use types::{ScreenshotInterface, ScreenshotType};

mod cli;
mod types;
mod x;

fn main() -> Result<(), String> {
    let input = cli::Cli::parse();

    // Determine which screenshot interface to use based on the input or the display server.
    let screenshot_interface: Box<dyn ScreenshotInterface>;
    if input.x11 && input.wayland {
        return Err("You can't use both `--x11` and `--wayland` at the same time.".to_string());
    }
    if input.x11 {
        screenshot_interface = Box::new(x::XScreenshot::new());
    } else if input.wayland {
        todo!("Wayland is not supported yet");
    } else {
        return Err("Auto detection of display server is not supported yet. Please specify a display server using `--x11` or `--wayland`.".to_string());
    }

    let screenshot_type: ScreenshotType;
    if input.all {
        screenshot_type = ScreenshotType::AllScreens;
    } else if input.monitor.is_some() {
        screenshot_type = ScreenshotType::Monitor(input.monitor.unwrap());
    } else if input.window.is_some() {
        todo!("--window is not supported yet");
    } else {
        return Err(
            "You must specify a screenshot type. Use `--help` for more information.".to_string(),
        );
    }

    let screenshot_image = screenshot_interface.take_screenshot(screenshot_type)?;
    println!("Screenshot taken.");

    if input.clipboard {
        screenshot_interface.copy_screenshot(&screenshot_image)?;
        println!("Screenshot copied to clipboard.");
    }
    if input.save.is_some() {
        screenshot_image
            .save(input.save.unwrap())
            .map_err(|e| e.to_string())?;
        println!("Screenshot saved to file.");
    }

    Ok(())
}
