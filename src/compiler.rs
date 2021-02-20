use std::todo;

use crate::ast;
use crate::error::Result;
use crate::pattern::{Condition, Pattern, State, Transition};

pub struct Compiler;

impl Compiler {
    pub fn new() -> Compiler {
        Compiler
    }

    pub fn compile(&self, expr: ast::Expression) -> Result<Pattern> {
        let mut states = vec![State::Accept];

        Ok(Pattern { states })
    }

    fn compile_states(&self, expr: ast::Expression) -> Vec<State> {
        match expr {
            ast::Expression::Sequence(seq) => {}
            _ => todo!(),
        }
        todo!()
    }

    fn compile_condition(&self, expr: ast::Expression) -> Condition {
        match expr {
            ast::Expression::Anchor(ast::Anchor::Start) => Condition::Start,
            ast::Expression::Anchor(ast::Anchor::End) => Condition::End,
            ast::Expression::Literal(c) => Condition::Literal(c),
            _ => todo!(),
        }
    }
}
