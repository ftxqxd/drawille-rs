extern crate drawille;

use drawille::Turtle;

fn main() {
    let mut turtle = Turtle::new(50., 0.);
    //turtle.up();
    turtle.down();
    for n in 0..100 {
        turtle.forward(10. - (n as f32)/10.);
        turtle.right(10.);
    }
    println!("{}", turtle.frame());
}
