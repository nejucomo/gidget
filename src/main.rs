mod boxchar;
mod buffer;
mod cell;
mod constraints;
mod direction;
mod dirpack;
mod params;
mod pt;
mod scaffold;

use std::collections::VecDeque;
use std::io::{Result, Stdout, Write as _};

use crossterm::terminal::{
    self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{QueueableCommand as _, event};
use rand::distr::{Bernoulli, Distribution as _};
use rand::seq::SliceRandom as _;

use crate::buffer::Buffer;
use crate::cell::Cell;
use crate::params::NEW_SEED_PROBABILITY;
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
    let seedcoin = Bernoulli::new(NEW_SEED_PROBABILITY).unwrap();
    let mut rng = rand::rng();
    let mut buf = Buffer::new(terminal::size()?);

    let mut blanks = VecDeque::from({
        let mut v: Vec<Pt> = buf.size().iter_area().collect();
        v.shuffle(&mut rng);
        v
    });
    let mut first = true;

    buf.redraw_screen(stdout)?;
    while !event::poll(params::INTERVAL)? {
        if let Some(pt) = blanks.pop_front() {
            let constraints = buf.get_constraints(pt);
            if !(first || constraints.is_constrained() || seedcoin.sample(&mut rng)) {
                // We should not create a new seed:
                blanks.push_back(pt);
                continue;
            }
            first = false;

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
