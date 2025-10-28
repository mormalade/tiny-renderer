mod draw;
mod parser;

use draw::Canvas;
use parser::parse_obj;

fn main() {
    let mut canvas = Canvas::new(64, 64);
    canvas.set_color(255, 0, 0);

    let (vertices, triangles) = parse_obj("src/diablo3_pose.obj", &canvas);

    // draw triangles
    for trinagle in triangles {
        let p1 = &vertices[trinagle.a];
        let p2 = &vertices[trinagle.b];
        let p3 = &vertices[trinagle.c];

        canvas.line(p1, p2);
        canvas.line(p2, p3);
        canvas.line(p1, p3);
    }

    canvas.save("triangles.tga");
}
