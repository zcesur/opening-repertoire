mod repertoire;
mod tree;

use std::error::Error;
use std::fs::File;

use pgn_reader::{BufferedReader, Color, SanPlus};

use repertoire::GameVisitor;
use tree::Tree;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Config {
    pub pgn_path: String,
    pub starting_moves: Vec<SanPlus>,
    pub color: Color,
    pub max_moves: usize,
    pub inode_max_depth: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config> {
        if args.len() < 6 {
            return Err("not enough arguments".into());
        }

        let pgn_path = args[1].clone();
        let starting_moves = starting_moves_from_str(&args[2])?;
        let color = color_from_str(&args[3])?;
        let max_moves = args[4].parse::<usize>()?;
        let inode_max_depth = args[5].parse::<usize>()?;

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

pub fn run(config: Config) -> Result<()> {
    let file = File::open(&config.pgn_path)?;
    let mut reader = BufferedReader::new(file);
    let mut opening_tree = Tree::new();
    let mut visitor = GameVisitor::new(
        &mut opening_tree,
        &config.starting_moves,
        config.color,
        config.max_moves,
    );
    reader.read_all(&mut visitor)?;
    opening_tree.prune(config.color);
    print!("{}", opening_tree.pgn(config.color, config.inode_max_depth));
    Ok(())
}
