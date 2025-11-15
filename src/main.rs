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

use std::collections::BTreeSet;
use std::io::{Result, Stdout, Write as _};

use crossterm::terminal::{
    self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{QueueableCommand as _, event};
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
            |s| s.queue(EnterAlternateScreen).map(|_| ()),
            |s| s.queue(LeaveAlternateScreen).map(|_| ()),
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

    let mut blanks: BTreeSet<Pt> = buf.area().points().collect();
    let mut sprouts: Vec<Pt> = vec![];

    buf.redraw_screen(stdout)?;
    while !event::poll(params::INTERVAL)? && !blanks.is_empty() {
        let pt: Pt = until_some::<_, Pt>(|| {
            let denom = u32::try_from(sprouts.len()).unwrap() + 1;
            let gen_sprout = rng.random_ratio(1, denom);
            if gen_sprout {
                // Generate a seed:
                let newsprout = *blanks.iter().choose(&mut rng).unwrap();
                sprouts.push(newsprout);
                Some(newsprout)
            } else {
                // Attempt to grow a sprout:
                let sprix = rng.random_range(..sprouts.len());
                if let Some(pt) = buf.area().clip(sprouts[sprix] + rng.random::<Direction>()) {
                    sprouts[sprix] = pt;
                    Some(pt)
                } else {
                    None
                }
            }
        });

        assert!(blanks.remove(&pt));
        let constraints = buf.get_constraints(pt);
        let cell = Cell::from(constraints.random_boxchar(&mut rng));
        buf[pt] = cell;

        stdout
            .queue(pt.move_to())?
            .queue(cell.print_styled_content())?
            .flush()?;
    }

    Ok(())
}

fn until_some<F, T>(mut f: F) -> T
where
    F: FnMut() -> Option<T>,
{
    loop {
        if let Some(v) = f() {
            return v;
        }
    }
}
