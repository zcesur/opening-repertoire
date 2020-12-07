use crate::tree::{Colored, NodeIndex, Tree};
use pgn_reader::{Color, SanPlus, Skip, Visitor};
use std::fmt;

#[derive(PartialEq)]
pub struct ColoredSanPlus(Color, SanPlus);

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

pub struct GameVisitor<'a> {
    turn: Color,
    plies: usize,
    skip: Skip,
    repertoire_color: Color,
    max_moves: usize,
    tree: &'a mut Tree<ColoredSanPlus>,
    cursor: Option<NodeIndex>,
    starting_moves: &'a [SanPlus],
}

impl<'a> GameVisitor<'a> {
    pub fn new(
        tree: &'a mut Tree<ColoredSanPlus>,
        starting_moves: &'a [SanPlus],
        repertoire_color: Color,
        max_moves: usize,
    ) -> GameVisitor<'a> {
        Self {
            turn: Color::White,
            plies: 0,
            skip: Skip(false),
            repertoire_color,
            max_moves,
            tree,
            cursor: None,
            starting_moves,
        }
    }

    fn reset(&mut self) {
        self.turn = Color::White;
        self.plies = 0;
        self.skip = Skip(false);
        self.cursor = None;
    }

    fn max_plies(&self) -> usize {
        let offset = match self.repertoire_color {
            Color::White => 1,
            Color::Black => 0,
        };
        self.max_moves * 2 - offset
    }
}

impl<'a> Visitor for GameVisitor<'a> {
    type Result = ();

    fn begin_game(&mut self) {
        self.reset();
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.skip == Skip(true) {
            return;
        }

        let move_undesired =
            self.plies < self.starting_moves.len() && self.starting_moves[self.plies] != san_plus;
        let cutoff_reached = self.plies == self.max_plies();
        if move_undesired || cutoff_reached {
            self.skip = Skip(true);
            return;
        }

        let val = ColoredSanPlus(self.turn, san_plus);
        let new_cursor = match self.cursor {
            Some(idx) => self.tree.get_child_or_insert(val, idx),
            None => self.tree.get_root_or_insert(val),
        };
        self.cursor = Some(new_cursor);

        self.plies += 1;
        self.turn = !self.turn;
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {}
}
