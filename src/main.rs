use image::{self, ImageBuffer};
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

fn line(p1: &Point, p2: &Point) -> Vec<Point> {
    let dx = p1.x.abs_diff(p2.x);
    let dy = p1.y.abs_diff(p2.y);
    let mut res: Vec<Point> = vec![];
    if dx > dy {
        if p1.x > p2.x {
            for x in p2.x..p1.x {
                let y = p2.y + ((x - p2.x) as f32 * (dy as f32 / dx as f32)) as u32;
                res.push(Point::new(x, y));
            }
        } else {
            for x in p1.x..p2.x {
                let y = p1.y + ((x - p1.x) as f32 * (dy as f32 / dx as f32)) as u32;
                res.push(Point::new(x, y));
            }
        }
    } else if p1.y > p2.y {
        for y in p2.y..p1.y {
            let x = p2.x + ((y - p2.y) as f32 * (dx as f32 / dy as f32)) as u32;
            res.push(Point::new(x, y));
        }
    } else {
        for y in p1.y..p2.y {
            let x = p1.x + ((y - p1.y) as f32 * (dx as f32 / dy as f32)) as u32;
            res.push(Point::new(x, y));
        }
    }

    res
}

fn main() {
    // draw lines
    let width = 64;
    let height = 64;

    let mut imgbuf = ImageBuffer::new(width, height);

    let mut rng = rand::rng();

    for _ in 0..1 << 16 {
        let a = Point::new(rng.random_range(0..64), rng.random_range(0..64));
        let b = Point::new(rng.random_range(0..64), rng.random_range(0..64));

        let line = line(&a, &b);
        let color: [u8; 4] = [rand::random(), rand::random(), rand::random(), 255];

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let current_point = Point::new(x, y);
            if line.contains(&current_point) {
                *pixel = image::Rgba(color);
            }
        }
    }

    imgbuf.save("lines.tga").unwrap();
}
