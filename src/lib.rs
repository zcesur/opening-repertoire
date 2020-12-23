mod chess_move;
mod reader;
mod tree;

use std::error::Error;
use std::fs::File;

use clap::ArgMatches;
use pgn_reader::{BufferedReader, Color, SanPlus};

use reader::PGNVisitor;
use tree::Tree;

type BoxedError = Box<dyn Error>;
type Result<T> = std::result::Result<T, BoxedError>;

pub enum OutputType {
    Pgn,
    Json,
    JsonPretty,
    Tree,
}

pub struct Config {
    pub pgn_path: String,
    pub starting_moves: Vec<SanPlus>,
    pub color: Color,
    pub max_moves: usize,
    pub inode_max_depth: usize,
    pub output_type: OutputType,
    pub prune: bool,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Config> {
        let pgn_path = matches
            .value_of("path")
            .ok_or("invalid path")
            .map(|x| x.to_owned())?;

        let starting_moves = match matches.value_of("starting-moves") {
            Some(s) => starting_moves_from_str(s),
            None => Ok(vec![]),
        }?;

        let color = match matches.value_of("color") {
            Some("white") => Ok(Color::White),
            Some("black") => Ok(Color::Black),
            _ => Err("invalid color"),
        }?;

        let max_moves = matches
            .value_of("max-moves")
            .ok_or::<BoxedError>("invalid max-moves".into())
            .and_then(|x| x.parse::<usize>().map_err(|e| e.into()))?;

        let inode_max_depth = matches
            .value_of("inode-max-depth")
            .ok_or::<BoxedError>("invalid inode-max-depth".into())
            .and_then(|x| x.parse::<usize>().map_err(|e| e.into()))?;

        let output_type = match matches.value_of("output-type") {
            Some("pgn") => Ok(OutputType::Pgn),
            Some("json") => Ok(OutputType::Json),
            Some("json-pretty") => Ok(OutputType::JsonPretty),
            Some("tree") => Ok(OutputType::Tree),
            _ => Err("invalid output-type"),
        }?;

        Ok(Config {
            pgn_path,
            starting_moves,
            color,
            max_moves,
            inode_max_depth,
            output_type,
            prune: !matches.is_present("disable-pruning"),
        })
    }
}

fn starting_moves_from_str(s: &str) -> Result<Vec<SanPlus>> {
    s.split(",")
        .map(|s| s.parse::<SanPlus>().map_err(|e| e.into()))
        .collect()
}

pub fn run(config: &Config) -> Result<String> {
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

    if config.prune {
        opening_tree.prune(config.color);
    }

    match config.output_type {
        OutputType::Pgn => Ok(opening_tree.pgn(config.color, config.inode_max_depth)),
        OutputType::Json => serde_json::to_string(&opening_tree).map_err(|e| e.into()),
        OutputType::JsonPretty => serde_json::to_string_pretty(&opening_tree).map_err(|e| e.into()),
        OutputType::Tree => Ok(opening_tree.to_string()),
    }
}
