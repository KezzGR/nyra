use image::{ImageBuffer, Rgba};

pub struct Texture {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let image = image::open(path)
            .expect("Failed to load texture")
            .to_rgba8();

        let (width, height) = image.dimensions();

        Self {
            image,
            width,
            height,
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            image: ImageBuffer::from_pixel(1, 1, Rgba([255, 255, 255, 255])),
            width: 1,
            height: 1,
        }
    }
}
