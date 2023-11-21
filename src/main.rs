use clap::Parser;
use types::{ScreenshotInterface, ScreenshotType};

mod cli;
mod types;
mod x;

fn main() -> Result<(), String> {
    let input = cli::Cli::parse();
    let screenshot_interface: Box<dyn ScreenshotInterface>;

    if input.x11 && input.wayland {
        eprintln!("You can't use both `--x11` and `--wayland` at the same time.");
        std::process::exit(1);
    }
    if input.x11 {
        screenshot_interface = Box::new(x::XScreenshot::new());
    } else if input.wayland {
        todo!("Wayland is not supported yet");
    } else {
        eprintln!("Auto detection of display server is not supported yet. Please specify a display server using `--x11` or `--wayland`.");
        std::process::exit(1);
    }

    let screenshot_type: ScreenshotType;
    if input.all {
        screenshot_type = ScreenshotType::AllScreens;
    } else if input.window.is_some() {
        todo!("--window is not supported yet");
    } else if input.screen.is_some() {
        todo!("--screen is not supported yet");
    } else {
        eprintln!("You must specify a screenshot type. Use `--help` for more information.");
        std::process::exit(1);
    }

    let screenshot_image = screenshot_interface
        .take_screenshot(screenshot_type)
        .expect("Failed to take screenshot");
    println!("Screenshot taken.");

    if input.clipboard {
        screenshot_interface.copy_screenshot(&screenshot_image)?;
    }
    if input.save.is_some() {
        screenshot_image
            .save(input.save.unwrap())
            .map_err(|e| e.to_string())?;
        println!("Screenshot saved to file.");
    }

    Ok(())
}
