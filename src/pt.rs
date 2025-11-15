#[cfg(test)]
mod tests;

use crossterm::cursor::MoveTo;
use derive_more::{From, Into};
use derive_new::new;

#[derive(Copy, Clone, Debug, From, Into, Eq, Ord, PartialEq, PartialOrd, new)]
pub struct Pt {
    col: u16,
    row: u16,
}

impl Pt {
    pub fn col(self) -> u16 {
        self.col
    }

    pub fn row(self) -> u16 {
        self.row
    }

    pub fn usizes(self) -> (usize, usize) {
        let Pt { col, row } = self;
        (usize::from(col), usize::from(row))
    }

    pub fn move_to(self) -> MoveTo {
        let Pt { col, row } = self;
        MoveTo(col, row)
    }
}

impl TryFrom<(usize, usize)> for Pt {
    type Error = <u16 as TryFrom<usize>>::Error;

    fn try_from((asize, bsize): (usize, usize)) -> Result<Self, Self::Error> {
        let a16 = u16::try_from(asize)?;
        let b16 = u16::try_from(bsize)?;
        Ok((a16, b16).into())
    }
}
