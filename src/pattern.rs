use crate::ast;
use crate::error::Result;

pub struct Pattern {
    pub states: Vec<State>,
}

impl Pattern {
    pub fn compile(expr: ast::Expression) -> Result<Pattern> {
        Ok(Pattern {
            states: vec![State::Accept],
        })
    }

    pub fn matches(&self, text: &str, state: usize) -> bool {
        match &self.states[state] {
            State::Accept => true,
            State::Match(_) => todo!(),
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

pub enum Condition {}
