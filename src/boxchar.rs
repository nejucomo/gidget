use std::ops::{Index, IndexMut};

use unicode_box_drawing::{BoxCharacter, Width};

use crate::direction::Direction::{self, Down, Left, Right, Up};
use crate::dirpack::DirPack;

#[derive(Copy, Clone, Debug)]
pub struct BoxChar {
    dp: DirPack<Width>,
    rounded: bool,
}

impl BoxChar {
    pub fn new(rounded: bool) -> Self {
        BoxChar {
            dp: DirPack {
                up: Width::None,
                down: Width::None,
                left: Width::None,
                right: Width::None,
            },
            rounded,
        }
    }

    pub fn char(self) -> char {
        let bc = BoxCharacter::from(self);
        if self.rounded {
            bc.char_round()
        } else {
            bc.char()
        }
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
        self.dp.index(d)
    }
}

impl IndexMut<Direction> for BoxChar {
    fn index_mut(&mut self, d: Direction) -> &mut Self::Output {
        self.dp.index_mut(d)
    }
}
