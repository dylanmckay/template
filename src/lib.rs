#[macro_use] extern crate error_chain;
extern crate regex;

pub use self::errors::{Error, ErrorKind};
pub use self::ast::Ast;

pub mod ast;
pub mod parse;
pub mod trans;
pub mod errors;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

