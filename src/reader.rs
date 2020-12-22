use pgn_reader::{Color, SanPlus, Skip, Visitor};

use crate::chess_move::Move;
use crate::tree::{NodeIndex, Tree};

pub struct PGNVisitor<'a> {
    turn: Color,
    plies: usize,
    skip: Skip,
    repertoire_color: Color,
    max_moves: usize,
    tree: &'a mut Tree<Move>,
    cursor: Option<NodeIndex>,
    starting_moves: &'a [SanPlus],
}

impl<'a> PGNVisitor<'a> {
    pub fn new(
        tree: &'a mut Tree<Move>,
        starting_moves: &'a [SanPlus],
        repertoire_color: Color,
        max_moves: usize,
    ) -> PGNVisitor<'a> {
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

impl<'a> Visitor for PGNVisitor<'a> {
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

        let val = Move {
            color: self.turn,
            san_plus,
        };
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
