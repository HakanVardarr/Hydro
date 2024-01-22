use std::path::Path;

use glium::{glutin::surface::WindowSurface, Display, Texture2d};
use image::{DynamicImage, Rgba, RgbaImage};

pub struct Texture;

impl Texture {
    pub fn new(path: &str, display: &Display<WindowSurface>) -> Texture2d {
        // TODO: REMOVE UNWRAP
        let image = convert_rgb_to_rgba(path);
        let image = image.as_rgba8().unwrap();

        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.as_raw(), image_dimensions);
        glium::texture::Texture2d::new(display, image).unwrap()
    }
}

fn convert_rgb_to_rgba(image_path: &str) -> DynamicImage {
    let im = image::open(Path::new(image_path)).expect("Failed to open image");
    let rgb_image = im.to_rgb8();

    // Create a new RGBA image with the same dimensions
    let mut rgba_image = RgbaImage::new(rgb_image.width(), rgb_image.height());

    // Copy RGB values from the original image to the RGBA image, setting alpha to 255 (fully opaque)
    for (x, y, pixel) in rgba_image.enumerate_pixels_mut() {
        let rgb_pixel = rgb_image.get_pixel(x, y);
        *pixel = Rgba([rgb_pixel[0], rgb_pixel[1], rgb_pixel[2], 255]);
    }

    DynamicImage::ImageRgba8(rgba_image)
}
