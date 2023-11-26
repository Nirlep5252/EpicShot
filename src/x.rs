use image::ImageEncoder;
use image::RgbaImage;
use std::io::Write;

use crate::types::{ScreenshotInterface, ScreenshotType};

pub(crate) struct XScreenshot {
    connection: xcb::Connection,
}

impl XScreenshot {
    pub fn new() -> Self {
        let (connection, _) = xcb::Connection::connect(None).unwrap();
        Self { connection }
    }

    fn get_screenshot_image(
        &self,
        screen: &xcb::x::Screen,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> RgbaImage {
        let cookie = self.connection.send_request(&xcb::x::GetImage {
            format: xcb::x::ImageFormat::ZPixmap,
            drawable: xcb::x::Drawable::Window(screen.root()),
            x,
            y,
            width,
            height,
            plane_mask: std::u32::MAX,
        });

        let reply = self
            .connection
            .wait_for_reply(cookie)
            .map_err(|e| e.to_string())
            .unwrap();
        RgbaImage::from_raw(
            width.into(),
            height.into(),
            self.image_data_to_pixels(reply.data()),
        )
        .expect("Unable to create image from raw data")
    }
}

impl ScreenshotInterface for XScreenshot {
    fn take_screenshot(
        &self,
        screenshot_type: crate::types::ScreenshotType,
    ) -> Result<RgbaImage, String> {
        let setup = self.connection.get_setup();
        let screen = setup.roots().nth(0).ok_or("No screen found")?;

        match screenshot_type {
            ScreenshotType::AllScreens => Ok(self.get_screenshot_image(
                screen,
                0,
                0,
                screen.width_in_pixels(),
                screen.height_in_pixels(),
            )),
            ScreenshotType::Monitor(monitor_num) => {
                // Get all the monitors
                let res_cookie = self.connection.send_request(&xcb::randr::GetMonitors {
                    window: screen.root(),
                    get_active: true,
                });
                let res_reply = self
                    .connection
                    .wait_for_reply(res_cookie)
                    .map_err(|e| e.to_string())?;
                let monitors = res_reply.monitors().collect::<Vec<_>>();

                if monitor_num >= monitors.len() {
                    return Err(format!(
                        "Monitor {} does not exist. There are only {} monitors.",
                        monitor_num,
                        monitors.len()
                    ));
                }
                Ok(self.get_screenshot_image(
                    screen,
                    monitors[monitor_num].x(),
                    monitors[monitor_num].y(),
                    monitors[monitor_num].width(),
                    monitors[monitor_num].height(),
                ))
            }
            ScreenshotType::Window(_) => {
                todo!();
            }
            ScreenshotType::Selection {
                x: _,
                y: _,
                width: _,
                height: _,
            } => {
                todo!();
            }
        }
    }

    fn copy_screenshot(&self, screenshot_image: &RgbaImage) -> Result<(), String> {
        let mut png_data = vec![];
        image::codecs::png::PngEncoder::new(&mut png_data)
            .write_image(
                screenshot_image,
                screenshot_image.width(),
                screenshot_image.height(),
                image::ColorType::Rgba8,
            )
            .map_err(|e| e.to_string())?;

        // store using command line: xclip
        let mut child = std::process::Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg("-t")
            .arg("image/png")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(&png_data)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
