use crate::ast;
use crate::error::{Error, Result};
use std::{iter::Peekable, str::Chars};

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(pattern: &'a str) -> Parser<'a> {
        Parser {
            chars: pattern.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<ast::Expression> {
        Ok(ast::Expression::Empty)
    }
}
