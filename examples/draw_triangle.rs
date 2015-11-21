extern crate drawille;

use drawille::Canvas;

fn main() {
    let mut canvas = Canvas::new(100, 100);
    canvas.line(2, 2, 80, 80);
    canvas.line(2, 80, 80, 80);
    canvas.line(2, 2, 2, 80);
    println!("{}", canvas.frame());
}
