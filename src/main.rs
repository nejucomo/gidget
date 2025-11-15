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
    println!("Hello.");
    let r = main_inner();
    println!("Goodbye?");
    println!("Result: {:#?}", &r);
    r
}

fn main_inner() -> Result<()> {
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
        .call(main_raw_mode)?;

    std::io::stdout().flush()?;
    println!("Bye!");
    Ok(())
}

fn main_raw_mode(stdout: &mut Stdout) -> Result<()> {
    let mut rng = rand::rng();
    let mut buf = Buffer::new(terminal::size()?);

    let mut blanks: BTreeSet<Pt> = buf.size().iter_area().collect();
    let mut sprouts: Vec<Pt> = vec![];

    let mut dbglog = "".to_string();

    buf.redraw_screen(stdout)?;
    while !event::poll(params::INTERVAL)? && !blanks.is_empty() {
        if dbglog.lines().count() > 2 {
            return Err(std::io::Error::other(dbglog));
        }

        let pt: Pt = until_some::<_, Pt>(|| {
            if sprouts.is_empty() || rng.random_ratio(1, u32::try_from(sprouts.len()).unwrap() + 1)
            {
                // Generate a seed:
                let pt = *blanks.iter().choose(&mut rng).unwrap();
                sprouts.push(pt);
                dbglog += &format!("sprouts: {:?}\n", &sprouts);

                Some(pt)
            } else {
                // Attempt to grow a sprout:
                let sprix = rng.random_range(..sprouts.len());
                if let Some(pt) =
                    (sprouts[sprix] + rng.random::<Direction>()).and_then(|pt| buf.size().clip(pt))
                {
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
