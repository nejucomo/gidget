use std::io::{Result, Stdout};

use crossterm::QueueableCommand as _;
use crossterm::style::{PrintStyledContent, StyledContent, Stylize as _};
use unicode_box_drawing::BoxCharacter;

#[derive(Copy, Clone, Debug, Default)]
pub struct Cell(Option<BoxCharacter>);

impl Cell {
    pub fn print(self, stdout: &mut Stdout) -> Result<&mut Stdout> {
        stdout.queue(PrintStyledContent(self.styled_content()))
    }

    fn styled_content(self) -> StyledContent<char> {
        match self.0 {
            Some(bc) => bc.char().dark_green().on_black(),
            None => '▒'.dark_yellow().dim(),
        }
    }
}
