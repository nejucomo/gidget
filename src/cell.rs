use crossterm::style::{PrintStyledContent, StyledContent, Stylize as _};
use derive_more::From;
use unicode_box_drawing::Width;

use crate::boxchar::BoxChar;
use crate::direction::Direction;

#[derive(Copy, Clone, Debug, Default, From)]
#[from(BoxChar, Option<BoxChar>)]
pub struct Cell(Option<BoxChar>);

impl Cell {
    pub fn is_empty(self) -> bool {
        self.0.is_none()
    }

    pub fn width(self, d: Direction) -> Option<Width> {
        self.0.map(|bc| bc[d])
    }

    pub fn print_styled_content(self) -> PrintStyledContent<char> {
        PrintStyledContent(self.styled_content())
    }

    fn styled_content(self) -> StyledContent<char> {
        match self.0 {
            Some(bc) => bc.char().dark_green().on_black(),
            None => '▒'.dark_yellow().dim(),
        }
    }
}
