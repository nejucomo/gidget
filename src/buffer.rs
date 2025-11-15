use std::io::{Result, Stdout, Write as _};

use crossterm::QueueableCommand as _;
use crossterm::terminal::{Clear, ClearType::All};
use rand::Rng;

use crate::cell::Cell;
use crate::pt::Pt;

#[derive(Debug)]
pub struct Buffer {
    size: Pt,
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn new<V: Into<Pt>>(size: V) -> Self {
        let size = size.into();
        Buffer {
            size,
            cells: vec![Cell::default(); size.area()],
        }
    }

    pub fn random_position<R: Rng>(&self, rng: &mut R) -> Pt {
        (
            rng.random_range(..self.size.0),
            rng.random_range(..self.size.1),
        )
            .into()
    }

    pub fn redraw_screen(&self, stdout: &mut Stdout) -> Result<()> {
        stdout.queue(Clear(All))?;

        let lastrow = None;
        for (pt, c) in self.iter() {
            if lastrow != Some(pt.1) {
                // First column on this row; move cursor:
                stdout.queue(pt.move_to())?;
            }
            c.print(stdout)?;
        }
        stdout.flush()?;
        Ok(())
    }

    fn iter(&self) -> impl Iterator<Item = (Pt, Cell)> {
        let size = self.size;
        self.cells
            .iter()
            .enumerate()
            .map(move |(ix, &c)| (size.ix_to_pt(ix), c))
    }
}
