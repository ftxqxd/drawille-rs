extern crate drawille;

use drawille::{PixelColor, PixelColor::TrueColor, Turtle};

fn main() {
    // Rainbow using true colors 
    let mut turtle_true = Turtle::new(0., 0.);
    let colors1 = vec![
        TrueColor{r: 255, g: 0,   b: 0},
        TrueColor{r: 255, g: 127, b:0},
        TrueColor{r: 255, g: 255, b: 0},
        TrueColor{r: 0,   g: 255, b:0},
        TrueColor{r: 0,   g: 0,   b:255},
        TrueColor{r: 46,  g: 43,  b:95},
        TrueColor{r: 139, g: 0,   b:255},
    ];
    
    for (cn, &color) in colors1.iter().enumerate() {
        turtle_true.up();
        turtle_true.teleport(0.+(cn as f32)*3., 50.);
        turtle_true.rotation = -90.;
        turtle_true.down();
        turtle_true.color(color);
        for _ in 0..150 {
            turtle_true.forward(1.-(cn as f32)/16.);
            turtle_true.right(180./150.);
        }
    }
    println!("{}", turtle_true.frame());

    // Rainbow using adaptive colors
    let mut turtle_buildin= Turtle::new(0., 0.);
    let colors2 = vec![
       PixelColor::Red,
       PixelColor::Yellow,
       PixelColor::Green,
       PixelColor::Blue,
       PixelColor::Magenta, 
    ];
    
    for (cn, &color) in colors2.iter().enumerate() {
        turtle_buildin.up();
        turtle_buildin.teleport(0.+(cn as f32)*3., 50.);
        turtle_buildin.rotation = -90.;
        turtle_buildin.down();
        turtle_buildin.color(color);
        for _ in 0..150 {
            turtle_buildin.forward(1.-(cn as f32)/16.);
            turtle_buildin.right(180./150.);
        }
    }
    println!("{}", turtle_buildin.frame());
}
