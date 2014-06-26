use std::collections::HashMap;
use std::cmp;
use std::default::Default;
use std::fmt::{Show, Formatter, FormatError};

#[deriving(Show, Clone, PartialEq, Eq)]
pub enum Color {
    Bright(Hue),
    Normal(Hue),
    Dim(Hue),
}

#[deriving(Show, Clone, PartialEq, Eq)]
pub enum Hue {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[deriving(Clone, PartialEq, Eq)]
struct ColorPair(Color, Color);

impl Show for ColorPair {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        // TODO: add Windows support if needed
        let ColorPair(first, second) = *self;
        let finit = match first {
            Bright(_) => "\x1b[0;1;",
            Normal(_) => "\x1b[0;",
            Dim(_) => "\x1b[0;2;",
        };
        let fend = match first {
            Bright(hue) | Normal(hue) | Dim(hue) =>
                hue as u32,
        };
        let f = format!("{}4{}m", finit, fend);
        let sinit = match second {
            Bright(_) => "\x1b[1;",
            Normal(_) => "\x1b[",
            Dim(_) => "\x1b[2;",
        };
        let send = match second {
            Bright(hue) | Normal(hue) | Dim(hue) =>
                hue as u32,
        };
        let s = format!("{}3{}m", sinit, send);
        try!(write!(fmt, "{}{}", f, s));
        Ok(())
    }
}

impl Index<uint, Color> for ColorPair {
    fn index(&self, index: &uint) -> Color {
        let ColorPair(c1, c2) = *self;
        match *index {
            0 => c1,
            1 => c2,
            _ => fail!("ColorPair index out of bounds"),
        }
    }
}

#[deriving(Clone, PartialEq, Eq)]
enum Pixel {
    Char(ColorPair, char),
    Pair(ColorPair),
}

impl Default for Pixel {
    fn default() -> Pixel {
        Char(ColorPair(Normal(Black), Normal(Black)), ' ')
    }
}

impl Show for Pixel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        match *self {
            Char(cp, a) => try!(write!(f, "{}{}", cp, a)),
            Pair(a) => try!(write!(f, "{}▄", a)),
        }
        Ok(())
    }
}

impl Pixel {
    fn index_mut<'a>(&'a mut self, index: uint) -> &'a mut Color {
        let cp = match *self {
            Pair(ref mut cp) => cp,
            _ => fail!("indexing a text pixel"),
        };
        let ColorPair(ref mut c1, ref mut c2) = *cp;
        match index {
            0 => c1,
            1 => c2,
            _ => fail!("ColorPair index out of bounds"),
        }
    }
}

impl Index<uint, Color> for Pixel {
    fn index(&self, index: &uint) -> Color {
        let cp = match *self {
            Pair(cp) => cp,
            _ => fail!("indexing a text pixel"),
        };
        let ColorPair(c1, c2) = cp;
        match *index {
            0 => c1,
            1 => c2,
            _ => fail!("ColorPair index out of bounds"),
        }
    }
}

#[deriving(Clone, Show, PartialEq, Eq)]
pub struct Canvas {
    blocks: HashMap<(uint, uint), Pixel>,
    width:  uint,
    height: uint,
}

impl Canvas {
    pub fn new(width: uint, height: uint) -> Canvas {
        Canvas {
            blocks: HashMap::new(),
            width: width / 2,
            height: height / 4,
        }
    }

    pub fn clear(&mut self) {
        self.blocks.clear();
    }

    pub fn text<S: Str>(&mut self, x: uint, y: uint, fg: Hue, bg: Hue, s: S) {
        let (row, col) = (x, y / 2);
        for (i, c) in s.as_slice().chars().enumerate() {
            let block = self.blocks.find_or_insert((row + i, col), Default::default());
            *block = Char(ColorPair(Normal(bg), Normal(fg)), c);
        }
    }

    pub fn set(&mut self, x: uint, y: uint, c: Color) {
        let (row, col) = (x, y / 2);
        let mut block = self.blocks.find_or_insert((row, col), Default::default());
        match block {
            ref mut a @ &Char(_, _) => **a = Pair(ColorPair(Normal(Black), Normal(Black))),
            _ => {},
        }

        *block.index_mut(y % 2) = c;
    }

    pub fn unset(&mut self, x: uint, y: uint) {
        let (row, col) = (x, y / 2);
        *self.blocks.find_or_insert((row, col), Default::default()).index_mut(y % 2) = Normal(Black);
    }

    pub fn get(&self, x: uint, y: uint) -> Color {
        let (col, row) = (x, y / 2);
        let col = self.blocks.find(&(row, col));
        
        match col {
            None => Normal(Black),
            Some(c) => c[y % 2],
        }
    }

    pub fn rows(&self) -> Vec<String> {
        let maxrow = cmp::max(self.width, self.blocks.keys().map(|&(x, _)| x).max().unwrap_or(0));
        let maxcol = cmp::max(self.height, self.blocks.keys().map(|&(_, y)| y).max().unwrap_or(0));

        let mut result = vec![];
        for y in range(0, maxcol + 1) {
            let mut row = String::new();
            for x in range(0, maxrow + 1) {
                let col = *self.blocks.find(&(x, y)).unwrap_or(&Default::default());
                row.push_str((format!("{}", col)).as_slice());
            }
            result.push(format!("{}\x1b[0m", row));
        }
        result
    }

    pub fn frame(&self) -> String {
        self.rows().connect("\n")
    }

    pub fn line_vec(&self, x1: uint, y1: uint, x2: uint, y2: uint) -> Vec<(uint, uint)> {
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

    pub fn line(&mut self, x1: uint, y1: uint, x2: uint, y2: uint, c: Color) {
        for &(x, y) in self.line_vec(x1, y1, x2, y2).iter() {
            self.set(x, y, c);
        }
    }
}
