pub enum Expression {
    Anchor(Anchor),               // ^ or $
    Any,                          // .
    Alternation(Vec<Expression>), // |
    Class(Class),                 // []
    Empty,                        //
    Literal(char),                // x
    Quantifier(u8, Option<u8>),   // ? * + {}
    Sequence(Vec<Expression>),    // ^.b?
}

pub enum Anchor {
    Start,
    End,
}

pub enum Class {
    Alnum,
    Alpha,
    Digit,
    Lower,
    Space,
    Upper,
}
