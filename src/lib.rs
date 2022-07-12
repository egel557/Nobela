#[macro_use]
extern crate pest_derive;

mod character;
pub mod parser;
pub use character::Character;
pub mod server;
#[cfg(test)]
pub mod test;

pub const FILE_EXTENSION: &str = "nobela";
