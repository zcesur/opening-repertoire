mod chess_move;
mod reader;
mod tree;

use std::error::Error;
use std::fs::File;

use clap::ArgMatches;
use pgn_reader::{BufferedReader, Color, SanPlus};

use chess_move::Move;
use reader::PGNVisitor;
use tree::Tree;

type BoxedError = Box<dyn Error>;
type Result<T> = std::result::Result<T, BoxedError>;

pub struct Config {
    pub pgn_path: String,
    pub starting_moves: Vec<SanPlus>,
    pub color: Color,
    pub max_moves: usize,
    pub inode_max_depth: usize,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Config> {
        let pgn_path = matches
            .value_of("path")
            .ok_or::<BoxedError>("invalid path".into())
            .map(|x| x.to_owned())?;

        let starting_moves = match matches.value_of("starting_moves") {
            Some(s) => starting_moves_from_str(s),
            None => Ok(vec![]),
        }?;

        let color = matches
            .value_of("color")
            .ok_or::<BoxedError>("invalid color".into())
            .and_then(color_from_str)?;

        let max_moves = matches
            .value_of("max_moves")
            .unwrap_or("10")
            .parse::<usize>()?;

        let inode_max_depth = matches
            .value_of("inode_max_depth")
            .unwrap_or("8")
            .parse::<usize>()?;

        Ok(Config {
            pgn_path,
            starting_moves,
            color,
            max_moves,
            inode_max_depth,
        })
    }
}

fn color_from_str(s: &str) -> Result<Color> {
    match s {
        "white" => Ok(Color::White),
        "black" => Ok(Color::Black),
        _ => Err("invalid color".into()),
    }
}

fn starting_moves_from_str(s: &str) -> Result<Vec<SanPlus>> {
    s.split(",")
        .map(|s| s.parse::<SanPlus>().map_err(|e| e.into()))
        .collect()
}

pub fn run(config: &Config) -> Result<Tree<Move>> {
    let file = File::open(&config.pgn_path)?;
    let mut reader = BufferedReader::new(file);
    let mut opening_tree = Tree::new();
    let mut visitor = PGNVisitor::new(
        &mut opening_tree,
        &config.starting_moves,
        config.color,
        config.max_moves,
    );
    reader.read_all(&mut visitor)?;
    opening_tree.prune(config.color);
    Ok(opening_tree)
}
