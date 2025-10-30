mod draw;
mod parser;

use draw::Canvas;
use parser::parse_obj;

use crate::draw::{New, Point, Triangle};

fn main() {
    let mut canvas = Canvas::new(128, 128);

    let t1 = Triangle::new(Point::new(7, 45), Point::new(35, 100), Point::new(45, 60));
    let t2 = Triangle::new(Point::new(120, 35), Point::new(90, 5), Point::new(45, 110));
    let t3 = Triangle::new(Point::new(115, 83), Point::new(80, 90), Point::new(85, 120));

    // triangles are weird
    canvas.set_color(255, 0, 0);
    canvas.triangle(t1);

    canvas.set_color(255, 255, 255);
    canvas.triangle(t2);

    canvas.set_color(0, 255, 0);
    canvas.triangle(t3);

    canvas.save("triangles.tga");
}
