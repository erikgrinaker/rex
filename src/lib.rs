#![warn(clippy::all)]

mod ast;
mod compiler;
mod error;
mod parser;
mod pattern;
mod regex;

pub use error::{Error, Result};
pub use regex::{matches, RegEx};
