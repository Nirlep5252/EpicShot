use image::RgbaImage;

pub(crate) enum ScreenshotType {
    AllScreens,
    Monitor(usize),
    Window(String),
    Selection {
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    },
}

pub(crate) trait ScreenshotInterface {
    fn take_screenshot(&self, screenshot_type: ScreenshotType) -> Result<RgbaImage, ()>;
    fn copy_screenshot(&self, screenshot_image: &RgbaImage) -> Result<(), ()>;

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
