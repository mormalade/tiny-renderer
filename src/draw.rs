use image::{self, ImageBuffer, Rgba};
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    color: Rgba<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let mut imgbuf = ImageBuffer::new(width, height);
        let color: [u8; 4] = [0, 0, 0, 255];
        let color = Rgba(color);
        for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = color;
        }
        Self {
            width,
            height,
            imgbuf,
            color,
        }
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        let color: [u8; 4] = [r, g, b, 255];
        let color = Rgba(color);
        self.color = color;
    }

    pub fn put_pixel(&mut self, x: u32, y: u32) {
        self.imgbuf.put_pixel(x, y, self.color);
    }

    pub fn save(&self, file: &str) {
        self.imgbuf.save(file).unwrap();
    }
    pub fn line(&mut self, p1: &Point, p2: &Point) {
        let dx = p1.x as i32 - p2.x as i32;
        let dy = p1.y as i32 - p2.y as i32;
        if dx.abs() > dy.abs() {
            if p1.x > p2.x {
                let mut y = p2.y as f32;
                for x in p2.x..p1.x {
                    y += dy as f32 / dx as f32;
                    self.imgbuf
                        .put_pixel(x, self.height - 1 - y as u32, self.color);
                }
            } else {
                let mut y = p1.y as f32;
                for x in p1.x..p2.x {
                    y += dy as f32 / dx as f32;
                    self.imgbuf
                        .put_pixel(x, self.height - 1 - y as u32, self.color);
                }
            }
        } else if p1.y > p2.y {
            let mut x = p2.x as f32;
            for y in p2.y..p1.y {
                x += dx as f32 / dy as f32;
                self.imgbuf
                    .put_pixel(x as u32, self.height - 1 - y, self.color);
            }
        } else {
            let mut x = p1.x as f32;
            for y in p1.y..p2.y {
                x += dx as f32 / dy as f32;
                self.imgbuf
                    .put_pixel(x as u32, self.height - 1 - y, self.color);
            }
        }
    }
}
