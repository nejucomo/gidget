mod area;
mod boxchar;
mod buffer;
mod cell;
mod constraints;
mod direction;
mod dirpack;
mod params;
mod pt;
mod scaffold;

use std::io::{Result, Stdout, Write as _};

use crossterm::terminal::{
    self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{QueueableCommand as _, cursor, event};
use rand::Rng as _;
use rand::seq::IteratorRandom as _;

use crate::buffer::Buffer;
use crate::cell::Cell;
use crate::direction::Direction;
use crate::pt::Pt;
use crate::scaffold::Scaffold;

fn main() -> Result<()> {
    let r = Scaffold::new(std::io::stdout())
        .layer(|_| Ok(()), |s| s.flush())
        .layer(|_| enable_raw_mode(), |_| disable_raw_mode())
        .layer(
            |s| s.queue(EnterAlternateScreen).map(|_| ()),
            |s| s.queue(LeaveAlternateScreen).map(|_| ()),
        )
        .layer(
            |s| s.queue(cursor::Hide).map(|_| ()),
            |s| s.queue(cursor::Show).map(|_| ()),
        )
        .call(main_raw_mode);

    match r {
        Ok(mut stdout) => {
            writeln!(stdout, "Bye!")?;
            Ok(())
        }
        Err(e) => {
            eprintln!("Error:\n{e}");
            Err(e)
        }
    }
}

fn main_raw_mode(stdout: &mut Stdout) -> Result<()> {
    let mut rng = rand::rng();
    let mut buf = Buffer::new(terminal::size()?);

    let mut seeds = 1;
    let mut q: Vec<Pt> = vec![rng.sample(buf.area())];

    buf.redraw_screen(stdout)?;
    while !event::poll(params::INTERVAL)? {
        if !q.is_empty() {
            let pt = q.swap_remove(rng.random_range(0..q.len()));

            if buf[pt].is_empty() {
                // queue up all it's neighbors:
                q.extend(Direction::each().filter_map(|d| buf.area().clip(pt + d)));

                let constraints = buf.get_constraints(pt);
                let cell = Cell::from(constraints.random_boxchar(&mut rng));
                buf[pt] = cell;

                stdout
                    .queue(pt.move_to())?
                    .queue(cell.print_styled_content())?
                    .flush()?;
            }

            if rng.random_ratio(1, seeds) {
                // plant a new seed:
                if let Some(pt) = buf
                    .iter()
                    .filter(|(_, c)| c.is_empty())
                    .map(|(pt, _)| pt)
                    .choose(&mut rng)
                {
                    q.push(pt);
                    seeds += 1;
                }
            }
        }
    }

    Ok(())
}
