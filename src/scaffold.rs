use std::io::Result;

use derive_new::new;

type DynOp<T> = Box<dyn FnOnce(&mut T) -> Result<()>>;

#[derive(new)]
pub struct Scaffold<T> {
    state: T,
    #[new(default)]
    layers: Vec<(DynOp<T>, DynOp<T>)>,
}

impl<T> Scaffold<T> {
    pub fn layer<S, C>(mut self, setup: S, cleanup: C) -> Self
    where
        S: FnOnce(&mut T) -> Result<()> + 'static,
        C: FnOnce(&mut T) -> Result<()> + 'static,
    {
        self.layers.push((Box::new(setup), Box::new(cleanup)));
        self
    }

    pub fn call<F>(self, f: F) -> Result<T>
    where
        F: FnOnce(&mut T) -> Result<()> + 'static,
    {
        // f is actually a setup with a no-op cleanup:
        let Self { mut state, layers } = self.layer(f, |_| Ok(()));

        // setup phase:
        let mut cleanups = vec![];
        let mut firsterr = None; // latch on first Err

        for (setup, cleanup) in layers {
            cleanups.push(cleanup);

            let res = setup(&mut state);

            // latch first err:
            firsterr = firsterr.or(res.err());

            // begin cleanup on first error:
            if firsterr.is_some() {
                break;
            }
        }

        // cleanup phase:
        while let Some(cleanup) = cleanups.pop() {
            let res = cleanup(&mut state);

            // latch first err, and continue cleanup even if there's an error:
            firsterr = firsterr.or(res.err());
        }

        // Return the first error, if any, else ok:
        firsterr.map(Err).unwrap_or(Ok(state))
    }
}
