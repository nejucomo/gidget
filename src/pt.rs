use crossterm::cursor::MoveTo;
use derive_more::{From, Into};

#[derive(Copy, Clone, Debug, From, Into)]
pub struct Pt(pub u16, pub u16);

impl Pt {
    pub fn area(self) -> usize {
        let Pt(x, y) = self;

        usize::from(x) * usize::from(y)
    }

    pub fn ix_to_pt(self, ix: usize) -> Pt {
        let Pt(w, h) = self;
        let col = ix % usize::from(w);
        let row = ix / usize::from(h);
        (col, row).try_into().unwrap()
    }

    pub fn move_to(self) -> MoveTo {
        let Pt(x, y) = self;
        MoveTo(x, y)
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
