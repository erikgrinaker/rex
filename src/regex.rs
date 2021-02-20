use crate::compiler::Compiler;
use crate::error::Result;
use crate::parser::Parser;
use crate::pattern::Pattern;

pub struct RegEx(Pattern);

impl RegEx {
    pub fn compile(pattern: &str) -> Result<RegEx> {
        Ok(Self(
            Compiler::new().compile(Parser::new(pattern).parse()?)?,
        ))
    }

    pub fn matches(&self, text: &str) -> bool {
        self.0.matches(text)
    }
}

pub fn matches(pattern: &str, text: &str) -> Result<bool> {
    Ok(RegEx::compile(pattern)?.matches(text))
}
