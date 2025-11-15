mod boxchar;
mod buffer;
mod cell;
mod constraints;
mod direction;
mod dirpack;
mod pt;
mod scaffold;

use std::io::{Result, Stdout, Write as _};
use std::time::Duration;

use crossterm::terminal::{
    self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{QueueableCommand as _, event};
use rand::seq::SliceRandom as _;

use crate::buffer::Buffer;
use crate::cell::Cell;
use crate::pt::Pt;
use crate::scaffold::Scaffold;

fn main() -> Result<()> {
    Scaffold::new(std::io::stdout())
        .layer(|_| enable_raw_mode(), |_| disable_raw_mode())
        .layer(
            |s| s.queue(EnterAlternateScreen).map(|_| ()),
            |s| s.queue(LeaveAlternateScreen).map(|_| ()),
        )
        .layer(
            |s| s.queue(EnterAlternateScreen).map(|_| ()),
            |s| s.queue(LeaveAlternateScreen).map(|_| ()),
        )
        .call(raw_mode_main)?;

    println!("Bye!");
    Ok(())
}

fn raw_mode_main(stdout: &mut Stdout) -> Result<()> {
    let mut rng = rand::rng();
    let mut buf = Buffer::new(terminal::size()?);

    let mut blanks: Vec<Pt> = buf.size().iter_area().collect();
    blanks.shuffle(&mut rng);

    buf.redraw_screen(stdout)?;
    while !event::poll(Duration::from_millis(100))? {
        if let Some(pt) = blanks.pop() {
            let constraints = buf.get_constraints(pt);
            let cell = Cell::from(constraints.random_boxchar(&mut rng));
            buf[pt] = cell;

            stdout
                .queue(pt.move_to())?
                .queue(cell.print_styled_content())?
                .flush()?;
        } else {
            return Err(std::io::Error::other("none left"));
        }
    }

    Ok(())
}
