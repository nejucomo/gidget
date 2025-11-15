use std::ops::{Index, IndexMut};

use unicode_box_drawing::{BoxCharacter, Width};

use crate::direction::Direction::{self, Down, Left, Right, Up};
use crate::dirpack::DirPack;

#[derive(Copy, Clone, Debug)]
pub struct BoxChar(DirPack<Width>);

impl BoxChar {
    pub fn char(self) -> char {
        BoxCharacter::from(self).char()
    }
}

impl Default for BoxChar {
    fn default() -> Self {
        BoxChar(DirPack {
            up: Width::None,
            down: Width::None,
            left: Width::None,
            right: Width::None,
        })
    }
}

impl From<BoxChar> for BoxCharacter {
    fn from(bc: BoxChar) -> Self {
        BoxCharacter::new(bc[Up], bc[Right], bc[Down], bc[Left])
    }
}

impl Index<Direction> for BoxChar {
    type Output = Width;

    fn index(&self, d: Direction) -> &Self::Output {
        self.0.index(d)
    }
}

impl IndexMut<Direction> for BoxChar {
    fn index_mut(&mut self, d: Direction) -> &mut Self::Output {
        self.0.index_mut(d)
    }
}
