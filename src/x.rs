use image::RgbaImage;

use crate::types::{ScreenshotInterface, ScreenshotType};

pub(crate) struct XScreenshot {
    connection: xcb::Connection,
}

impl XScreenshot {
    pub fn new() -> Self {
        let (connection, _) = xcb::Connection::connect(None).unwrap();
        Self { connection }
    }
}

impl ScreenshotInterface for XScreenshot {
    fn take_screenshot(
        &self,
        screenshot_type: crate::types::ScreenshotType,
    ) -> Result<RgbaImage, String> {
        let setup = self.connection.get_setup();
        let mut screens = setup.roots();

        match screenshot_type {
            ScreenshotType::AllScreens => {
                let screen = screens.nth(0).unwrap();
                let cookie = self.connection.send_request(&xcb::x::GetImage {
                    format: xcb::x::ImageFormat::ZPixmap,
                    drawable: xcb::x::Drawable::Window(screen.root()),
                    x: 0,
                    y: 0,
                    width: screen.width_in_pixels(),
                    height: screen.height_in_pixels(),
                    plane_mask: std::u32::MAX,
                });

                let reply = self
                    .connection
                    .wait_for_reply(cookie)
                    .map_err(|e| e.to_string())?;
                let image = RgbaImage::from_raw(
                    screen.width_in_pixels().into(),
                    screen.height_in_pixels().into(),
                    self.image_data_to_pixels(reply.data()),
                )
                .expect("Unable to create image from raw data");
                Ok(image)
            }
            ScreenshotType::Screen(_) => {
                todo!();
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
        todo!();
    }
}
