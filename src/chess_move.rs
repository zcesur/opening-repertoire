use std::fmt;

use pgn_reader::{Color, SanPlus};

#[derive(PartialEq)]
pub struct Move {
    pub color: Color,
    pub san_plus: SanPlus,
}

impl Move {
    pub fn dots(&self) -> String {
        match self.color {
            Color::White => String::from("."),
            Color::Black => String::from("..."),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.san_plus)
    }
}
