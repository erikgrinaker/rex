use crate::compiler::Compiler;
use crate::error::Result;
use crate::parser::Parser;
use crate::pattern::Pattern;

pub struct RegEx(Pattern);

impl RegEx {
    pub fn compile(pattern: &str) -> Result<RegEx> {
        Ok(Self(
            Compiler::new(Parser::new(pattern).parse()?).compile()?,
        ))
    }

    pub fn matches(&self, text: &str) -> bool {
        self.0.matches(text, 0)
    }
}

pub fn matches(pattern: &str, text: &str) -> Result<bool> {
    Ok(RegEx::compile(pattern)?.matches(text))
}
