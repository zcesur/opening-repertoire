use std::process;

use clap::{App, Arg};

use opening_repertoire::Config;

fn main() {
    let matches = App::new("opening-repertoire")
        .version("0.1.0")
        .author("Zafer Cesur")
        .about("Generates a chess opening repertoire from a PGN")
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .help("Repertoire color: either 'white' or 'black'")
                .required(true)
                .takes_value(true)
                .value_name("COLOR"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("path/to/input.pgn")
                .required(true)
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("starting_moves")
                .short("s")
                .long("starting_moves")
                .help("Filter games by some comma-separated starting moves, e.g. 'e4,c5'")
                .takes_value(true)
                .value_name("STRING"),
        )
        .arg(
            Arg::with_name("max_moves")
                .short("m")
                .long("max_moves")
                .help("Maximum number of moves (default: 10)")
                .takes_value(true)
                .value_name("NUM"),
        )
        .arg(
            Arg::with_name("inode_max_depth")
                .short("d")
                .long("inode_max_depth")
                .help("Maximum depth of variations that stem from internal (non-leaf) nodes (default: 8)")
                .takes_value(true)
                .value_name("NUM"),
        )
        .get_matches();

    let config = Config::new(matches).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = opening_repertoire::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
