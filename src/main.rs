use pgn_reader::{BufferedReader, Color, SanPlus};
use repertoire::GameVisitor;
use std::fs::File;
use tree::Tree;
mod repertoire;
mod tree;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn try_main() -> Result<()> {
    let file = File::open("data/ericrosen-white.pgn")?;
    let mut reader = BufferedReader::new(file);
    let mut opening_tree = Tree::new();
    let repertoire_color = Color::White;
    let max_moves = 10;
    let starting_moves = ["e4", "c5"]
        .iter()
        .map(|s| s.parse::<SanPlus>().map_err(|e| e.into()))
        .collect::<Result<Vec<_>>>()?;
    let mut visitor = GameVisitor::new(
        &mut opening_tree,
        &starting_moves,
        repertoire_color,
        max_moves,
    );
    reader.read_all(&mut visitor)?;
    opening_tree.prune(repertoire_color);
    println!("{}", opening_tree);
    Ok(())
}

fn main() {
    try_main().unwrap();
}
