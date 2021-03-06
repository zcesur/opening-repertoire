use std::fmt;

use pgn_reader::{Color, SanPlus};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct Move {
    pub color: Color,
    pub san_plus: SanPlus,
    frequency: u64,
}

impl Move {
    pub fn new(color: Color, san_plus: SanPlus) -> Self {
        Self {
            color,
            san_plus,
            frequency: 0,
        }
    }

    pub fn frequency(&self) -> u64 {
        self.frequency
    }

    pub fn inc_frequency(&mut self) {
        // TODO: Use checked_add to ensure no integer overflow occurs
        self.frequency += 1;
    }

    pub fn dots(&self) -> String {
        match self.color {
            Color::White => String::from("."),
            Color::Black => String::from("..."),
        }
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.san_plus == other.san_plus
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.san_plus, self.frequency)
    }
}

impl Serialize for Move {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Move", 2)?;
        s.serialize_field("san_plus", &self.san_plus.to_string())?;
        s.serialize_field("frequency", &self.frequency)?;
        s.end()
    }
}
