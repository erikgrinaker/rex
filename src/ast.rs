pub enum Pattern {
    Anchor(Anchor),             // ^ or $
    Any,                        // .
    Alternation(Vec<Pattern>),  // |
    Class(Class),               // []
    Literal(String),            // abc
    Quantifier(u8, Option<u8>), // ? * + {}
    Sequence(Vec<Pattern>),     // ^.b?
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
