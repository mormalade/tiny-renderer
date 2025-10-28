mod draw;

use draw::Canvas;
use std::{fs::File, io::Read};

#[derive(Debug)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            a: a - 1,
            b: b - 1,
            c: c - 1,
        }
    }
}

fn main() {
    let mut model = File::open("src/diablo3_pose.obj").expect("file should be there");
    let mut buf = String::new();
    _ = model.read_to_string(&mut buf).unwrap();

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();
    // make a hashmap??
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
                let z: f32 = line.next().unwrap().parse().unwrap();
                let vertex = Vertex::new(x, y, z);
                vertices.push(vertex);
            }
            "f" => {
                // divide by / and then push to faces
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
    // draw points
    let mut canvas = Canvas::new(1024, 1024);
    canvas.set_color(255, 0, 0);

    for vertex in vertices {
        // -1.0 to 1.0 converted into 0 to 64
        // (0 to 2) / 2 is the same
        let x = ((vertex.x + 1.) * (canvas.width - 1) as f32 / 2.) as u32;
        let y = ((vertex.y + 1.) * (canvas.height - 1) as f32 / 2.) as u32;
        canvas.put_pixel(x, canvas.height - 1 - y);
    }

    canvas.save("diablo.tga");
}
