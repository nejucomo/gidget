use std::io::{Result, Stdout, Write as _};
use std::ops::{Index, IndexMut};

use crossterm::QueueableCommand as _;
use crossterm::terminal::{Clear, ClearType::All};

use crate::area::Area;
use crate::cell::Cell;
use crate::constraints::Constraints;
use crate::direction::Direction;
use crate::pt::Pt;

#[derive(Debug)]
pub struct Buffer {
    area: Area,
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn new<V: Into<Area>>(area: V) -> Self {
        let area = area.into();
        Buffer {
            area,
            cells: vec![Cell::default(); area.cell_count()],
        }
    }

    // Accessors
    pub fn area(&self) -> Area {
        self.area
    }

    pub fn iter(&self) -> impl Iterator<Item = (Pt, Cell)> {
        let size = self.area;
        self.cells
            .iter()
            .enumerate()
            .map(move |(ix, &c)| (size.ix_to_pt(ix), c))
    }

    pub fn get_constraints(&self, pt: Pt) -> Constraints {
        assert!(self[pt].is_empty());

        let mut cons = Constraints::default();

        for dir in Direction::each() {
            if let Some(neighbor) = self.area().clip(pt + dir) {
                cons[dir] = self[neighbor].width(dir.opposite());
            }
        }

        cons
    }

    // Rendering
    pub fn redraw_screen(&self, stdout: &mut Stdout) -> Result<()> {
        stdout.queue(Clear(All))?;

        let mut lastrow = None;
        for (pt, c) in self.iter() {
            if lastrow != Some(pt.row()) {
                // First column on this row; move cursor:
                stdout.queue(pt.move_to())?;
                lastrow = Some(pt.row());
            }
            stdout.queue(c.print_styled_content())?;
        }
        stdout.flush()?;
        Ok(())
    }
}

impl Index<Pt> for Buffer {
    type Output = Cell;

    fn index(&self, pt: Pt) -> &Self::Output {
        &self.cells[self.area.pt_to_ix(pt)]
    }
}

impl IndexMut<Pt> for Buffer {
    fn index_mut(&mut self, pt: Pt) -> &mut Self::Output {
        &mut self.cells[self.area.pt_to_ix(pt)]
    }
}
