//! Terminal graphics using Braille characters
//!
//! This module provides an interface for utilising Braille characters to draw a picture to a
//! terminal, allowing for much smaller pixels but losing proper colour support.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::num::{Float, FloatMath};
use std::char;
use std::cmp;

static PIXEL_MAP: [[int, ..2], ..4] = [[0x01, 0x08],
                                       [0x02, 0x10],
                                       [0x04, 0x20],
                                       [0x40, 0x80]];

/// A canvas object that can be used to draw to the terminal using Braille characters.
#[deriving(Clone, Show, PartialEq, Eq)]
pub struct Canvas {
    chars: HashMap<(uint, uint), int>,
    width:  uint,
    height: uint,
}

impl Canvas {
    /// Creates a new `Canvas` with the given width and height.
    ///
    /// Note that the `Canvas` can still draw outside the given dimensions (expanding the canvas)
    /// if a pixel is set outside the dimensions.
    pub fn new(width: uint, height: uint) -> Canvas {
        Canvas {
            chars: HashMap::new(),
            width: width / 2,
            height: height / 4,
        }
    }

    /// Clears the canvas.
    pub fn clear(&mut self) {
        self.chars.clear();
    }

    /// Sets a pixel at the specified coordinates.
    pub fn set(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        match self.chars.entry((row, col)) {
            Entry::Occupied(_) => {},
            Entry::Vacant(e) => { e.set(0); },
        }
        self.chars[(row, col)] |= PIXEL_MAP[y % 4][x % 2];
    }

    /// Deletes a pixel at the specified coordinates.
    pub fn unset(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        match self.chars.entry((row, col)) {
            Entry::Occupied(_) => {},
            Entry::Vacant(e) => { e.set(0); },
        }
        self.chars[(row, col)] &= !PIXEL_MAP[y % 4][x % 2];
    }

    /// Toggles a pixel at the specified coordinates.
    pub fn toggle(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        match self.chars.entry((row, col)) {
            Entry::Occupied(_) => {},
            Entry::Vacant(e) => { e.set(0); },
        }
        self.chars[(row, col)] ^= PIXEL_MAP[y % 4][x % 2];
    }

    /// Detects whether the pixel at the given coordinates is set.
    pub fn get(&self, x: uint, y: uint) -> bool {
        let dot_index = PIXEL_MAP[y % 4][x % 2];
        let (col, row) = (x / 2, y / 4);
        let char = self.chars.get(&(row, col));
        
        match char {
            None => false,
            Some(c) => *c & dot_index != 0,
        }
    }

    /// Returns a `Vec` of each row of the `Canvas`.
    ///
    /// Note that each row is actually four pixels high due to the fact that a single Braille
    /// character spans two by four pixels.
    pub fn rows(&self) -> Vec<String> {
        let maxrow = cmp::max(self.width, self.chars.keys().map(|&(x, _)| x).max().unwrap_or(0));
        let maxcol = cmp::max(self.height, self.chars.keys().map(|&(_, y)| y).max().unwrap_or(0));

        let mut result = vec![];
        for y in range(0, maxcol + 1) {
            let mut row = String::new();
            for x in range(0, maxrow + 1) {
                let char = *self.chars.get(&(x, y)).unwrap_or(&0);
                row.push(if char == 0 {
                    ' '
                } else {
                    char::from_u32((0x2800 + char) as u32).unwrap()
                })
            }
            result.push(row);
        }
        result
    }

    /// Draws the canvas to a `String` and returns it.
    pub fn frame(&self) -> String {
        self.rows().into_iter().collect::<Vec<String>>().connect("\n")
    }

    fn line_vec(&self, x1: uint, y1: uint, x2: uint, y2: uint) -> Vec<(uint, uint)> {
        let xdiff = cmp::max(x1, x2) - cmp::min(x1, x2);
        let ydiff = cmp::max(y1, y2) - cmp::min(y1, y2);
        let xdir = if x1 <= x2 { 1 } else { -1 };
        let ydir = if y1 <= y2 { 1 } else { -1 };

        let r = cmp::max(xdiff, ydiff);

        let mut result = vec![];
        for i in range(0, r + 1) {
            let mut x = x1;
            let mut y = y1;

            if ydiff != 0 {
                y += (i * ydiff) / r * ydir;
            }
            if xdiff != 0 {
                x += (i * xdiff) / r * xdir;
            }

            result.push((x, y));
        }
        result
    }

    /// Draws a line from `(x1, y1)` to `(x2, y2)` onto the `Canvas`.
    pub fn line(&mut self, x1: uint, y1: uint, x2: uint, y2: uint) {
        for &(x, y) in self.line_vec(x1, y1, x2, y2).iter() {
            self.set(x, y);
        }
    }
}

/// A ‘turtle’ that can walk around a canvas drawing lines.
pub struct Turtle {
    pub x: f32,
    pub y: f32,
    pub brush: bool,
    pub rotation: f32,
    pub cvs: Canvas,
}

impl Turtle {
    /// Create a new `Turtle`, starting at the given coordinates.
    ///
    /// The turtle starts with its brush down, facing right.
    pub fn new(x: f32, y: f32) -> Turtle {
        Turtle {
            cvs: Canvas::new(0, 0),
            x: x,
            y: y,
            brush: true,
            rotation: 0.0,
        }
    }

    /// Creates a new `Turtle` with the provided `Canvas`, starting at the given coordinates.
    ///
    /// The turtle starts with its brush down, facing right.
    pub fn from_canvas(x: f32, y: f32, cvs: Canvas) -> Turtle {
        Turtle {
            cvs: cvs,
            x: x,
            y: y,
            brush: true,
            rotation: 0.0,
        }
    }

    /// Sets the width of a `Turtle`’s `Canvas`, and return it for use again.
    pub fn width(mut self, width: uint) -> Turtle {
        self.cvs.width = width;
        self
    }

    /// Sets the height of a `Turtle`’s `Canvas`, and return it for use again.
    pub fn height(mut self, height: uint) -> Turtle {
        self.cvs.height = height;
        self
    }

    /// Lifts the `Turtle`’s brush.
    pub fn up(&mut self) {
        self.brush = false;
    }

    /// Puts down the `Turtle`’s brush.
    pub fn down(&mut self) {
        self.brush = true;
    }

    /// Toggles the `Turtle`’s brush.
    pub fn toggle(&mut self) {
        self.brush = !self.brush;
    }

    /// Moves the `Turtle` forward by `dist` steps.
    pub fn forward(&mut self, dist: f32) {
        let x = self.x + self.rotation.to_radians().cos()*dist;
        let y = self.y + self.rotation.to_radians().sin()*dist;
        self.teleport(x, y);
    }

    /// Moves the `Turtle` backward by `dist` steps.
    pub fn back(&mut self, dist: f32) {
        self.forward(-dist);
    }

    /// Teleports the `Turtle` to the given coordinates.
    ///
    /// Note that this draws a line between the old position and the new one if the `Turtle`’s
    /// brush is down.
    pub fn teleport(&mut self, x: f32, y: f32) {
        if self.brush {
            self.cvs.line(cmp::max(0, self.x.round() as int) as uint,
                          cmp::max(0, self.y.round() as int) as uint,
                          cmp::max(0, x.round() as int) as uint,
                          cmp::max(0, y.round() as int) as uint);
        }

        self.x = x;
        self.y = y;
    }

    /// Turns the `Turtle` right (clockwise) by `angle` degrees.
    pub fn right(&mut self, angle: f32) {
        self.rotation += angle;
    }

    /// Turns the `Turtle` left (clockwise) by `angle` degrees.
    pub fn left(&mut self, angle: f32) {
        self.rotation -= angle;
    }

    /// Writes the `Turtle`’s `Canvas` to a `String` and returns it.
    pub fn frame(&self) -> String {
        self.cvs.frame()
    }
}
