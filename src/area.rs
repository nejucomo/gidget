use crate::pt::Pt;

#[derive(Copy, Clone, Debug)]
pub struct Area(Pt);

impl From<(u16, u16)> for Area {
    fn from((w, h): (u16, u16)) -> Self {
        Area::new(w, h)
    }
}

impl Area {
    pub fn new(w: u16, h: u16) -> Self {
        Area(Pt::new(w, h))
    }

    pub fn width(self) -> u16 {
        self.0.col()
    }

    pub fn height(self) -> u16 {
        self.0.row()
    }

    pub fn cell_count(self) -> usize {
        let (x, y) = self.0.usizes();
        x * y
    }

    pub fn points(self) -> impl Iterator<Item = Pt> {
        (0..self.cell_count()).map(move |ix| self.ix_to_pt(ix))
    }

    pub fn ix_to_pt(self, ix: usize) -> Pt {
        let (w, _) = self.0.usizes();
        let col = ix % w;
        let row = ix / w;
        (col, row).try_into().unwrap()
    }

    pub fn pt_to_ix(self, pt: Pt) -> usize {
        let (w, _) = self.0.usizes();
        let (col, row) = pt.usizes();
        row * w + col
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
}
