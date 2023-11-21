use image::RgbaImage;

pub(crate) enum ScreenshotType {
    AllScreens,
    Screen(u32),
    Window(u32),
    Selection {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
}

pub(crate) trait ScreenshotInterface {
    fn take_screenshot(&self, screenshot_type: ScreenshotType) -> Result<RgbaImage, String>;
    fn copy_screenshot(&self, screenshot_image: &RgbaImage) -> Result<(), String>;

    fn image_data_to_pixels(&self, image_data: &[u8]) -> Vec<u8> {
        let mut pixels = Vec::new();
        for chunk in image_data.chunks(4) {
            pixels.push(chunk[2]);
            pixels.push(chunk[1]);
            pixels.push(chunk[0]);
            pixels.push(chunk[3]);
        }
        pixels
    }
}
