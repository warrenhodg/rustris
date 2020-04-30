use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Colour {
    Empty,
    Wall,
    Value(usize),
}

impl fmt::Debug for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colour::Empty => write!(f, "empty"),
            Colour::Wall => write!(f, "wall"),
            Colour::Value(v) => write!(f, "#{0}", v),
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colour::Empty => write!(f, "empty"),
            Colour::Wall => write!(f, "wall"),
            Colour::Value(v) => write!(f, "#{0}", v),
        }
    }
}
