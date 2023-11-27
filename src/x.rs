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
        window: xcb::x::Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<RgbaImage, ()> {
        let cookie = self.connection.send_request(&xcb::x::GetImage {
            format: xcb::x::ImageFormat::ZPixmap,
            drawable: xcb::x::Drawable::Window(window),
            x,
            y,
            width,
            height,
            plane_mask: std::u32::MAX,
        });

        let reply = self
            .connection
            .wait_for_reply(cookie)
            .map_err(|e| error!("Unable to get image: {e}"))?;
        Ok(RgbaImage::from_raw(
            width.into(),
            height.into(),
            self.image_data_to_pixels(reply.data()),
        )
        .expect("Unable to create image from raw data"))
    }
}

impl ScreenshotInterface for XScreenshot {
    fn take_screenshot(
        &self,
        screenshot_type: crate::types::ScreenshotType,
    ) -> Result<RgbaImage, ()> {
        let setup = self.connection.get_setup();
        let screen = setup
            .roots()
            .nth(0)
            .ok_or("No screen found")
            .map_err(|e| error!("{e}"))?;

        match screenshot_type {
            ScreenshotType::AllScreens => self.get_screenshot_image(
                screen.root(),
                0,
                0,
                screen.width_in_pixels(),
                screen.height_in_pixels(),
            ),
            ScreenshotType::Monitor(monitor_num) => {
                // Get all the monitors
                let res_cookie = self.connection.send_request(&xcb::randr::GetMonitors {
                    window: screen.root(),
                    get_active: true,
                });
                let res_reply = self
                    .connection
                    .wait_for_reply(res_cookie)
                    .map_err(|e| error!("Unable to get monitors: {e}"))?;

                if monitor_num >= res_reply.n_outputs() as usize {
                    error!(
                        "Monitor {} does not exist. There are only {} monitors. (0-indexed)",
                        monitor_num,
                        res_reply.n_outputs()
                    );
                    return Err(());
                }
                let monitor = res_reply.monitors().nth(monitor_num).unwrap();
                self.get_screenshot_image(
                    screen.root(),
                    monitor.x(),
                    monitor.y(),
                    monitor.width(),
                    monitor.height(),
                )
            }
            ScreenshotType::Window(window_id) => {
                let window_id = u32::from_str_radix(window_id.trim_start_matches("0x"), 16)
                    .map_err(|_| error!("Invalid window ID (it starts with \"0x\")"))?;
                let window = unsafe { std::mem::transmute::<u32, xcb::x::Window>(window_id) };
                let geometry_cookie = self.connection.send_request(&xcb::x::GetGeometry {
                    drawable: xcb::x::Drawable::Window(window),
                });
                let geometry = self
                    .connection
                    .wait_for_reply(geometry_cookie)
                    .map_err(|e| error!("Unable to get geometry: {e}"))?;
                debug!("{:?}", geometry);
                self.get_screenshot_image(window, 0, 0, geometry.width(), geometry.height())
            }
            ScreenshotType::Selection {
                x,
                y,
                width,
                height,
            } => self.get_screenshot_image(screen.root(), x, y, width, height),
        }
    }

    fn copy_screenshot(&self, screenshot_image: &RgbaImage) -> Result<(), ()> {
        let mut png_data = vec![];
        image::codecs::png::PngEncoder::new(&mut png_data)
            .write_image(
                screenshot_image,
                screenshot_image.width(),
                screenshot_image.height(),
                image::ColorType::Rgba8,
            )
            .map_err(|e| error!("Unable to encode image to PNG: {e}"))?;

        // store using command line: xclip
        let mut child = std::process::Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg("-t")
            .arg("image/png")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| error!("Unable to copy to xclip[0]: {e}"))?;
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(&png_data)
            .map_err(|e| error!("Unable to copy to xclip[1]: {e}"))?;
        Ok(())
    }
}
