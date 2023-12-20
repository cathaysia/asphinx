use std::path::PathBuf;

use asphinx_derive::api;

#[api(Default)]
pub struct Single {
    path: PathBuf,
}

impl Single {
    pub fn parse<T: AsRef<PathBuf>>(&self, path: &T) {}
}
