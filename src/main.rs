use std::process;

use clap::{App, Arg};

use opening_repertoire::Config;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
                .help("path/to/games.pgn")
                .required(true)
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("starting-moves")
                .short("s")
                .long("starting-moves")
                .help("Filter games by some comma-separated starting moves, e.g. 'e4,c5'")
                .takes_value(true)
                .value_name("STRING"),
        )
        .arg(
            Arg::with_name("max-moves")
                .short("m")
                .long("max-moves")
                .help("Maximum number of moves")
                .takes_value(true)
                .value_name("NUM")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("inode-max-depth")
                .short("d")
                .long("inode-max-depth")
                .help("Maximum depth of variations that stem from internal (non-leaf) nodes")
                .takes_value(true)
                .value_name("NUM")
                .default_value("8"),
        )
        .arg(
            Arg::with_name("output-type")
                .short("t")
                .long("output-type")
                .help("Type of output: either 'pgn' or 'tree'")
                .takes_value(true)
                .value_name("STRING")
                .default_value("pgn"),
        )
        .arg(
            Arg::with_name("disable-pruning")
                .long("disable-pruning")
                .help("By default, moves are pruned with respect to the repertoire color such that there is a single response for each opponent move")
        )
        .get_matches();

    let config = Config::new(matches).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match opening_repertoire::run(&config) {
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
        Ok(res) => {
            print!("{}", res);
        }
    }
}
