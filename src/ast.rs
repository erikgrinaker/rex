#[derive(Clone, Debug)]
pub enum Expression {
    Alternation(Vec<Expression>), // (|)
    Anchor(Anchor),               // ^ or $
    Any,                          // .
    Class(Class),                 // []
    Literal(char),                // x
    Quantifier(u8, Option<u8>),   // ? * + {}
    Sequence(Vec<Expression>),
}

#[derive(Clone, Debug)]
pub enum Anchor {
    Start,
    End,
}

#[derive(Clone, Debug)]
pub enum Class {
    Alnum,
    Alpha,
    Digit,
    Lower,
    Space,
    Upper,
}
