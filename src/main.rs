use image::{self, ImageBuffer};

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

    let a = Point::new(7, 3);
    let b = Point::new(12, 37);
    let c = Point::new(62, 53);

    let points = [&a, &b, &c];

    let ab = line(&a, &b);
    let ca = line(&c, &a);
    let bc = line(&b, &c);
    let ac = line(&a, &c);

    let black: [u8; 4] = [0, 0, 0, 255];
    let white: [u8; 4] = [255, 255, 255, 255];
    let red: [u8; 4] = [255, 0, 0, 255];
    let green: [u8; 4] = [0, 255, 0, 255];
    let blue: [u8; 4] = [0, 0, 255, 255];
    let yellow: [u8; 4] = [255, 200, 0, 255];

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgba(black);
        let current_point = Point::new(x, y);
        if ab.contains(&current_point) {
            *pixel = image::Rgba(red);
        }
        if ca.contains(&current_point) {
            *pixel = image::Rgba(yellow);
        }
        if bc.contains(&current_point) {
            *pixel = image::Rgba(green);
        }
        if ac.contains(&current_point) {
            *pixel = image::Rgba(blue);
        }
        for point in points {
            if current_point == *point {
                *pixel = image::Rgba(white);
            }
        }
    }

    imgbuf.save("lines.tga").unwrap();
}
