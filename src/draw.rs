use std::mem::swap;

use image::{self, ImageBuffer, Rgba};

#[derive(Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Triangle<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

pub trait New<T> {
    fn new(a: T, b: T, c: T) -> Self;
}

impl New<Point<u32>> for Triangle<Point<u32>> {
    fn new(a: Point<u32>, b: Point<u32>, c: Point<u32>) -> Self {
        Self { a, b, c }
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
        let y = self.height - 1 - y; //flip on y axis
        self.imgbuf.put_pixel(x, y, self.color);
    }

    pub fn save(&self, file: &str) {
        self.imgbuf.save(file).unwrap();
    }

    pub fn line(&mut self, p1: &Point<u32>, p2: &Point<u32>) {
        let dx = p1.x as i32 - p2.x as i32;
        let dy = p1.y as i32 - p2.y as i32;
        if dx.abs() > dy.abs() {
            if p1.x > p2.x {
                let mut y = p2.y as f32;
                for x in p2.x..=p1.x {
                    self.put_pixel(x, (y + 0.5) as u32);
                    y += dy as f32 / dx as f32;
                }
            } else {
                let mut y = p1.y as f32;
                for x in p1.x..=p2.x {
                    self.put_pixel(x, (y + 0.5) as u32);
                    y += dy as f32 / dx as f32;
                }
            }
        } else if p1.y > p2.y {
            let mut x = p2.x as f32;
            for y in p2.y..=p1.y {
                self.put_pixel((x + 0.5) as u32, y);
                x += dx as f32 / dy as f32;
            }
        } else {
            let mut x = p1.x as f32;
            for y in p1.y..=p2.y {
                self.put_pixel((x + 0.5) as u32, y);
                x += dx as f32 / dy as f32;
            }
        }
    }

    pub fn triangle(&mut self, mut t: Triangle<Point<u32>>) {
        if t.a.y > t.b.y {
            swap(&mut t.a, &mut t.b);
        }
        if t.a.y > t.c.y {
            swap(&mut t.a, &mut t.c);
        }
        if t.b.y > t.c.y {
            swap(&mut t.b, &mut t.c);
        }

        // ab and ac
        let dx1 = t.c.x as i32 - t.a.x as i32;
        let dy1 = t.c.y - t.a.y;
        let dx2 = t.c.x as i32 - t.b.x as i32;
        let dy2 = dy1;
        let dx3 = t.b.x as i32 - t.c.x as i32;
        let dy3 = t.c.y - (t.b.y - t.a.y);
        let mut x1 = t.a.x as f32;
        let mut x2 = t.a.x as f32;
        let mut x3 = t.a.x as f32;
        for y in t.a.y..=t.b.y {
            self.put_pixel((x1 + 0.5) as u32, y);
            self.put_pixel((x2 + 0.5) as u32, y);
            x1 += dx1 as f32 / dy1 as f32;
            x2 += dx2.abs() as f32 / dy2 as f32;
        }
        // for y in t.b.y..=t.c.y {
        //     // draw other side
        //     self.put_pixel((x1 + 0.5) as u32, y);
        //     self.put_pixel((x3 + 0.5) as u32, y);
        //     x1 += dx1 as f32 / dy1 as f32;
        //     x3 += dx3 as f32 / dy3 as f32;
        // }

        println!("a={:?}, b={:?}, c={:?}", t.a, t.b, t.c);
    }
}
