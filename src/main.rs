mod buffer;
mod cell;
mod constraints;
mod pt;
mod scaffold;

use std::io::{Result, Stdout};
use std::time::Duration;

use crossterm::terminal::{
    self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{QueueableCommand as _, event};

use crate::buffer::Buffer;
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
    let buf = Buffer::new(terminal::size()?);

    buf.redraw_screen(stdout)?;
    while !event::poll(Duration::from_millis(100))? {
        let _pos = buf.random_position(&mut rng);
        todo!()
    }

    Ok(())
}
