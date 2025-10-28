use image::{self, ImageBuffer, Rgba};
use rand::prelude::*;

#[derive(PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

fn line(p1: &Point, p2: &Point, imgbuf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, color: &Rgba<u8>) {
    let dx = p1.x as i32 - p2.x as i32;
    let dy = p1.y as i32 - p2.y as i32;
    if dx.abs() > dy.abs() {
        if p1.x > p2.x {
            let mut y = p2.y as f32;
            for x in p2.x..p1.x {
                y += dy as f32 / dx as f32;
                imgbuf.put_pixel(x, y as u32, *color);
            }
        } else {
            let mut y = p1.y as f32;
            for x in p1.x..p2.x {
                y += dy as f32 / dx as f32;
                imgbuf.put_pixel(x, y as u32, *color);
            }
        }
    } else if p1.y > p2.y {
        let mut x = p2.x as f32;
        for y in p2.y..p1.y {
            x += dx as f32 / dy as f32;
            imgbuf.put_pixel(x as u32, y, *color);
        }
    } else {
        let mut x = p1.x as f32;
        for y in p1.y..p2.y {
            x += dx as f32 / dy as f32;
            imgbuf.put_pixel(x as u32, y, *color);
        }
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
}

pub fn random_lines() {
    let width = 64;
    let height = 64;

    let mut imgbuf = ImageBuffer::new(width, height);

    let mut rng = rand::rng();

    for _ in 0..1 << 24 {
        let a = Point::new(rng.random_range(0..width), rng.random_range(0..height));
        let b = Point::new(rng.random_range(0..width), rng.random_range(0..height));

        let color: [u8; 4] = [rand::random(), rand::random(), rand::random(), 255];
        let color = Rgba(color);
        line(&a, &b, &mut imgbuf, &color);
    }

    imgbuf.save("lines.tga").unwrap();
}
