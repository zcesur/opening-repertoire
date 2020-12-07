use std::fmt;

use pgn_reader::{Color, SanPlus};

pub trait Colored {
    fn color(&self) -> Color;
}

#[derive(PartialEq)]
pub struct ColoredSanPlus(pub Color, pub SanPlus);

impl fmt::Display for ColoredSanPlus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl Colored for ColoredSanPlus {
    fn color(&self) -> Color {
        self.0
    }
}

pub fn dots(color: Color) -> String {
    match color {
        Color::White => String::from("."),
        Color::Black => String::from("..."),
    }
}
