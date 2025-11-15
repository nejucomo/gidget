#[cfg(test)]
mod tests;

use crossterm::cursor::MoveTo;
use derive_more::{From, Into};

#[derive(Copy, Clone, Debug, From, Into)]
pub struct Pt(pub u16, pub u16);

impl Pt {
    pub fn area(self) -> usize {
        let Pt(x, y) = self;

        usize::from(x) * usize::from(y)
    }

    pub fn iter_area(self) -> impl Iterator<Item = Pt> {
        (0..self.area()).map(move |ix| self.ix_to_pt(ix))
    }

    pub fn ix_to_pt(self, ix: usize) -> Pt {
        let (w, _): (usize, usize) = self.into();
        let col = ix % w;
        let row = ix / w;
        (col, row).try_into().unwrap()
    }

    pub fn pt_to_ix(self, pt: Pt) -> usize {
        let (w, _): (usize, usize) = self.into();
        let (col, row): (usize, usize) = pt.into();
        row * w + col
    }

    pub fn clip(self, area: Pt) -> Option<Pt> {
        let Pt(x, y) = self;
        let Pt(w, h) = area;
        if x < w && y < h { Some(self) } else { None }
    }

    pub fn move_to(self) -> MoveTo {
        let Pt(x, y) = self;
        MoveTo(x, y)
    }
}

impl From<Pt> for (usize, usize) {
    fn from(pt: Pt) -> Self {
        let Pt(x, y) = pt;
        (usize::from(x), usize::from(y))
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
