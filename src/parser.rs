use crate::draw::{Canvas, New, Point, Triangle};
use std::{fs::File, io::Read};

impl New<usize> for Triangle<usize> {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a: a - 1,
            b: b - 1,
            c: c - 1,
        }
    }
}

pub fn parse_obj(file: &str, canvas: &Canvas) -> (Vec<Point>, Vec<Triangle<usize>>) {
    let mut model = File::open(file).expect("file should be there");
    let mut buf = String::new();
    _ = model.read_to_string(&mut buf).unwrap();

    let mut vertices: Vec<Point> = Vec::new();
    let mut triangles: Vec<Triangle<usize>> = Vec::new();

    for line in buf.lines() {
        let mut line = line.split_whitespace();
        let data_type = match line.next() {
            Some(data) => data,
            None => continue,
        };
        match data_type {
            "v" => {
                let x: f32 = line.next().unwrap().parse().unwrap();
                let y: f32 = line.next().unwrap().parse().unwrap();
                // I don't use z for now
                let z: f32 = line.next().unwrap().parse().unwrap();

                // -1.0 to 1.0 converted into 0 to canvas.width
                // (0 to 2) / 2 is the same as -1 to 1
                let x = ((x + 1.) * (canvas.width - 1) as f32 / 2.) as u32;
                let y = ((y + 1.) * (canvas.height - 1) as f32 / 2.) as u32;
                let point = Point::new(x, y);
                vertices.push(point);
            }
            "f" => {
                let a: usize = line
                    .next()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let b: usize = line
                    .next()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let c: usize = line
                    .next()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let triangle = Triangle::new(a, b, c);
                triangles.push(triangle);
            }
            _ => (),
        }
    }
    (vertices, triangles)
}
