#![crate_type = "lib"]
#![crate_id = "drawille"]

//! drawille—a terminal graphics library for Rust, based on the Python library
//! [drawille](https://github.com/asciimoo/drawille).

extern crate term;

pub mod braille;
pub mod block;
