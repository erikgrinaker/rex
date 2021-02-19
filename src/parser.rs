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

    fn next_if_char(&mut self, expect: char) -> Option<char> {
        match self.chars.peek() {
            Some(c) if c == &expect => self.chars.next(),
            _ => None,
        }
    }

    pub fn parse(&mut self) -> Result<ast::Expression> {
        // We always parse into an alternation, since the root expression may
        // turn out to be an alternation (e.g. 'foo|bar').
        let expr = self.parse_alternation()?;

        // If there's any remaining characters in the pattern, they were unexpected.
        if let Some(c) = self.chars.peek() {
            return Err(Error(format!("unexpected character {}", c)));
        }
        Ok(expr)
    }

    // Parses an alternation of sequences (i.e. |-separated patterns).
    fn parse_alternation(&mut self) -> Result<ast::Expression> {
        let mut alts = vec![self.parse_sequence()?];
        while self.next_if_char('|').is_some() {
            alts.push(self.parse_sequence()?)
        }
        Ok(ast::Expression::Alternation(alts))
    }

    // Parses a sequence of atoms into an ast::Expression::Sequence.
    fn parse_sequence(&mut self) -> Result<ast::Expression> {
        let mut seq = Vec::new();
        while let Some(atom) = self.parse_atom()? {
            seq.push(atom)
        }
        Ok(ast::Expression::Sequence(seq))
    }

    // Parses an atom, if found.
    fn parse_atom(&mut self) -> Result<Option<ast::Expression>> {
        let atom = match self.chars.peek() {
            Some('^') => ast::Expression::Anchor(ast::Anchor::Start),
            Some('$') => ast::Expression::Anchor(ast::Anchor::End),
            Some('.') => ast::Expression::Any,
            Some('?') => ast::Expression::Quantifier(0, Some(1)),
            Some('*') => ast::Expression::Quantifier(0, None),
            Some('+') => ast::Expression::Quantifier(1, None),
            Some('(') => self.parse_group()?,
            Some(')') => return Ok(None),
            Some('[') => self.parse_class()?,
            Some(']') => return Ok(None),
            Some('{') => self.parse_quantifier()?,
            Some('}') => return Ok(None),
            Some('|') => return Ok(None),
            Some('\\') => {
                self.chars.next();
                if let Some(c) = self.chars.next() {
                    ast::Expression::Literal(c)
                } else {
                    return Err(Error("unexpected end of pattern after \\".into()));
                }
            }
            Some(c) => ast::Expression::Literal(*c),
            None => return Ok(None),
        };
        self.chars.next();
        Ok(Some(atom))
    }

    fn parse_class(&mut self) -> Result<ast::Expression> {
        todo!()
    }

    fn parse_group(&mut self) -> Result<ast::Expression> {
        todo!()
    }

    fn parse_quantifier(&mut self) -> Result<ast::Expression> {
        todo!()
    }
}
