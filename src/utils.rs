mod asciidoctor_builder;
mod duration;
mod git;
mod html;
pub mod jinjaext;
mod tests;
mod tmpl;

pub use asciidoctor_builder::*;
pub use duration::*;
pub use git::*;
pub use html::*;
pub use tmpl::*;

pub fn cpu_num() -> usize {
    std::thread::available_parallelism()
        .map(|item| item.get())
        .unwrap_or(16)
}
