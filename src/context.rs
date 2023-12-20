mod index;
mod single;

use std::path::PathBuf;

pub use index::*;
pub use single::*;

pub trait Parse {
    fn parse<T: AsRef<PathBuf>>(&self, path: &T);
    fn render(&self);
}

pub enum Context {
    Index(Index),
    Single(Single),
}
