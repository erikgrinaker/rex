use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
    todo,
};

use crate::ast;
use crate::compiler::Compiler;
use crate::error::Result;

type Cursor<'a> = Peekable<Enumerate<Chars<'a>>>;

pub struct Pattern {
    pub states: Vec<State>,
}

impl Pattern {
    pub fn compile(expr: ast::Expression) -> Result<Pattern> {
        Compiler::new().compile(expr)
    }

    pub fn matches(&self, text: &str) -> bool {
        self.matches_at(text.chars().enumerate().peekable(), 0)
    }

    fn matches_at(&self, cursor: Cursor, state: usize) -> bool {
        match &self.states[state] {
            State::Accept => true,
            State::Match(transitions) => {
                for transition in transitions {
                    let mut next_cursor = cursor.clone();
                    if transition.condition.attempt(&mut next_cursor)
                        && self.matches_at(next_cursor, transition.target)
                    {
                        return true;
                    }
                }
                false
            }
        }
    }
}

pub enum State {
    Accept,
    Match(Vec<Transition>),
}

pub struct Transition {
    pub condition: Condition,
    pub target: usize, // Pattern.states index
}

pub enum Condition {
    Always,
    Start,
    End,
    Literal(char),
}

impl Condition {
    fn attempt(&self, cursor: &mut Cursor) -> bool {
        match self {
            Self::Always => true,
            Self::Start => matches!(cursor.peek(), Some((pos, _)) if pos == &0),
            Self::End => cursor.peek().is_none(),
            Self::Literal(l) => match cursor.next() {
                Some((_, c)) => l == &c,
                None => false,
            },
        }
    }
}
