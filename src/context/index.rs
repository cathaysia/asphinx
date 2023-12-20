use std::path::PathBuf;

use asphinx_derive::api;

use super::*;

#[api(Default)]
pub struct Index {}

impl Parse for Index {
    fn parse<T: AsRef<PathBuf>>(&self, path: &T) {}

    fn render(&self) {}
}
