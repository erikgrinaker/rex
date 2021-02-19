use crate::ast;
use crate::error::Result;
use crate::pattern::{Condition, Pattern, State, Transition};

pub struct Compiler {}

impl Compiler {
    pub fn new(expr: ast::Expression) -> Compiler {
        Compiler {}
    }

    pub fn compile(self) -> Result<Pattern> {
        Ok(Pattern {
            states: vec![State::Accept],
        })
    }
}
