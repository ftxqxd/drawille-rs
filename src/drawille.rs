#![crate_type = "lib"]
#![crate_name = "drawille"]

//! drawille—a terminal graphics library for Rust. The `braille` module is based on the Python
//! library [drawille](https://github.com/asciimoo/drawille).
//!
//! # Example
//!
//! ```
//! extern crate drawille;
//!
//! use drawille::braille::Canvas;
//!
//! fn main() {
//!     let mut canvas = Canvas::new(10, 10);
//!     canvas.set(5, 4);
//!     canvas.line(2, 2, 8, 8);
//!     println!("{}", canvas.frame());
//! }
//! ```

pub mod braille;
pub mod block;
