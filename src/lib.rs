#[macro_use]
extern crate log;

mod dna;
mod rna;

pub const WIDTH: u32 = 600;
pub const HEIGHT: u32 = 600;

pub use self::dna::DnaExecutor;
pub use self::rna::{RnaRenderer, RNA};
