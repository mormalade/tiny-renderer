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
            for x in p2.x..p1.x {
                let y = p2.y + ((x - p2.x) as f32 * (dy as f32 / dx as f32)) as u32;
                imgbuf.put_pixel(x, y, *color);
            }
        } else {
            for x in p1.x..p2.x {
                let y = p1.y + ((x - p1.x) as f32 * (dy as f32 / dx as f32)) as u32;
                imgbuf.put_pixel(x, y, *color);
            }
        }
    } else if p1.y > p2.y {
        for y in p2.y..p1.y {
            let x = p2.x + ((y - p2.y) as f32 * (dx as f32 / dy as f32)) as u32;
            imgbuf.put_pixel(x, y, *color);
        }
    } else {
        for y in p1.y..p2.y {
            let x = p1.x + ((y - p1.y) as f32 * (dx as f32 / dy as f32)) as u32;
            imgbuf.put_pixel(x, y, *color);
        }
    }
}

fn main() {
    // draw lines
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
