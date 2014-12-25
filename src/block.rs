use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cmp;
use std::default::Default;
use std::fmt::{mod, Show, Formatter};

#[deriving(Copy, Show, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[deriving(Copy, Clone, PartialEq, Eq)]
struct ColorPair(Color, Color);

impl Show for ColorPair {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // TODO: add Windows support if needed
        let ColorPair(first, second) = *self;
        let finit = "\x1b[0;";
        let fend = first as u32;
        let f = format!("{}4{}m", finit, fend);
        let sinit = "\x1b[";
        let send = second as u32;
        let s = format!("{}3{}m", sinit, send);
        try!(write!(fmt, "{}{}", f, s));
        Ok(())
    }
}

#[deriving(Copy, Clone, PartialEq, Eq)]
enum Pixel {
    Char(ColorPair, char),
    Pair(ColorPair),
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel::Char(ColorPair(Color::Black, Color::Black), ' ')
    }
}

impl Show for Pixel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Pixel::Char(cp, a) => try!(write!(f, "{}{}", cp, a)),
            Pixel::Pair(a) => try!(write!(f, "{}▄", a)),
        }
        Ok(())
    }
}

impl IndexMut<uint, Color> for Pixel {
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut Color {
        let cp = match *self {
            Pixel::Pair(ref mut cp) => cp,
            _ => panic!("indexing a text pixel"),
        };
        let ColorPair(ref mut c1, ref mut c2) = *cp;
        match *index {
            0 => c1,
            1 => c2,
            _ => panic!("ColorPair index out of bounds"),
        }
    }
}

impl Pixel {
    fn index(&self, index: uint) -> Color {
        let cp = match *self {
            Pixel::Pair(cp) => cp,
            _ => panic!("indexing a text pixel"),
        };
        let ColorPair(c1, c2) = cp;
        match index {
            0 => c1,
            1 => c2,
            _ => panic!("ColorPair index out of bounds"),
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

    pub fn text<S: Str>(&mut self, x: uint, y: uint, fg: Color, bg: Color, s: S) {
        let (row, col) = (x, y / 2);
        for (i, c) in s.as_slice().chars().enumerate() {
            match self.blocks.entry((row + i, col)) {
                Entry::Occupied(e) => *e.into_mut() = Pixel::Char(ColorPair(bg, fg), c),
                Entry::Vacant(e) => { e.set(Pixel::Char(ColorPair(bg, fg), c)); },
            }
        }
    }

    pub fn set(&mut self, x: uint, y: uint, c: Color) {
        let (row, col) = (x, y / 2);
        let mut block = match self.blocks.entry((row, col)) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.set(Default::default()),
        };
        match block {
            ref mut a @ &Pixel::Char(_, _) => **a = Pixel::Pair(ColorPair(Color::Black, Color::Black)),
            _ => {},
        }

        block[y % 2] = c;
    }

    pub fn unset(&mut self, x: uint, y: uint) {
        let (row, col) = (x, y / 2);
        let mut block = match self.blocks.entry((row, col)) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.set(Default::default()),
        };
        block[y % 2] = Color::Black;
    }

    pub fn get(&self, x: uint, y: uint) -> Color {
        let (col, row) = (x, y / 2);
        let col = self.blocks.get(&(row, col));
        
        match col {
            None => Color::Black,
            Some(c) => c.index(y % 2),
        }
    }

    pub fn rows(&self) -> Vec<String> {
        let maxrow = cmp::max(self.width, self.blocks.keys().map(|&(x, _)| x).max().unwrap_or(0));
        let maxcol = cmp::max(self.height, self.blocks.keys().map(|&(_, y)| y).max().unwrap_or(0));

        let mut result = vec![];
        for y in range(0, maxcol + 1) {
            let mut row = String::new();
            for x in range(0, maxrow + 1) {
                let col = *self.blocks.get(&(x, y)).unwrap_or(&Default::default());
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
