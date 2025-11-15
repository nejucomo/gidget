use derive_more::From;
use derive_new::new;
use rand::distr::Distribution;

use crate::pt::Pt;

#[derive(Copy, Clone, Debug, From, new)]
pub struct Area {
    width: u16,
    height: u16,
}

impl Area {
    pub fn width(self) -> u16 {
        self.width
    }

    pub fn height(self) -> u16 {
        self.height
    }

    pub fn cell_count(self) -> usize {
        let (x, y) = self.usizes();
        x * y
    }

    pub fn points(self) -> impl Iterator<Item = Pt> {
        (0..self.cell_count()).map(move |ix| self.ix_to_pt(ix))
    }

    pub fn ix_to_pt(self, ix: usize) -> Pt {
        let (w, _) = self.usizes();
        let col = ix % w;
        let row = ix / w;
        (col, row).try_into().unwrap()
    }

    pub fn pt_to_ix(self, pt: Pt) -> usize {
        let (w, _) = self.usizes();
        let (col, row) = pt.into();
        usize::from(row) * w + usize::from(col)
    }

    pub fn clip(self, opt: Option<Pt>) -> Option<Pt> {
        opt.and_then(|pt| {
            if pt.col() < self.width() && pt.row() < self.height() {
                Some(pt)
            } else {
                None
            }
        })
    }

    fn usizes(self) -> (usize, usize) {
        let Area { width, height } = self;
        (usize::from(width), usize::from(height))
    }
}

impl Distribution<Pt> for Area {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Pt {
        self.ix_to_pt(rng.random_range(0..self.cell_count()))
    }
}
