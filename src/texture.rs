extern crate image;
use image::{ImageReader, Pixel, DynamicImage, GenericImageView};
use std::fmt;
use crate::color::Color;

#[derive(Clone)]
pub struct Texture {
    image: DynamicImage,
    pub width: usize,
    pub height: usize,
    color_array: Vec<Color>,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut texture = Texture {
            image: img,
            width,
            height,
            color_array: vec![Color::black(); width * height],
        };
        texture.load_color_array();
        texture
    }

    fn load_color_array(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = self.image.get_pixel(x as u32, y as u32).to_rgb();
                let color = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
                self.color_array[y * self.width + x] = Color::from_hex(color);
            }
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let width = self.width as f32;
        let height = self.height as f32;

        
        let u = u.fract();
        let v = v.fract();

        
        let u = if u < 0.0 { u + 1.0 } else { u };
        let v = if v < 0.0 { v + 1.0 } else { v };

        
        let x = (u * width) as usize % self.width;
        let y = ((1.0 - v) * height) as usize % self.height; 

        let index = y * self.width + x;

        if x >= self.width || y >= self.height {
            Color::from_hex(0xFF00FF)
        } else {
            self.color_array[index]
        }
    }
}

impl fmt::Debug for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Texture")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
