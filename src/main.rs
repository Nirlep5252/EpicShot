use clap::Parser;
use types::{ScreenshotInterface, ScreenshotType};

mod cli;
mod types;
mod x;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() -> Result<(), ()> {
    pretty_env_logger::init();
    let input = cli::Cli::parse();

    // Determine which screenshot interface to use based on the input or the display server.
    let screenshot_interface: Box<dyn ScreenshotInterface>;
    if input.x11 && input.wayland {
        error!("You can't use both `--x11` and `--wayland` at the same time.");
        return Err(());
    }
    if input.x11 {
        info!("Using X11");
        screenshot_interface = Box::new(x::XScreenshot::new());
    } else if input.wayland {
        todo!("Wayland is not supported yet");
    } else {
        todo!("Auto detection of display server is not supported yet. Please specify a display server using `--x11` or `--wayland`.");
    }

    let screenshot_type: ScreenshotType;
    if input.all {
        info!("Taking screenshot of all screens");
        screenshot_type = ScreenshotType::AllScreens;
    } else if input.monitor.is_some() {
        info!("Taking screenshot of monitor {}", input.monitor.unwrap());
        screenshot_type = ScreenshotType::Monitor(input.monitor.unwrap());
    } else if input.window.is_some() {
        info!("Taking screenshot of window {:?}", input.window);
        screenshot_type = ScreenshotType::Window(input.window.unwrap());
    } else {
        error!("You must specify a screenshot type. Use `--help` for more information.");
        return Err(());
    }

    let screenshot_image = screenshot_interface.take_screenshot(screenshot_type)?;
    info!("Screenshot taken.");

    if input.clipboard {
        screenshot_interface.copy_screenshot(&screenshot_image)?;
        info!("Screenshot copied to clipboard.");
    }
    if input.save.is_some() {
        screenshot_image
            .save(input.save.unwrap())
            .map_err(|e| error!("{e}"))?;
        info!("Screenshot saved to file.");
    }

    Ok(())
}
