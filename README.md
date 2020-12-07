OpeningRepertoire
==========

OpeningRepertoire is a command line tool that generates chess opening repertoires from chess games in PGN format. The output can be imported into a study tool that lets you analyze (e.g. lichess) or practice (e.g. chessable) your repertoire.


Installation
------------
Use the package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install opening-repertoire.

```bash
cargo install --git https://github.com/zcesur/opening-repertoire
```

Documentation
-------------
```
USAGE:
    opening-repertoire [OPTIONS] --color <COLOR> --path <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --color <COLOR>              Repertoire color: either 'white' or 'black'
    -d, --inode_max_depth <NUM>      Maximum depth of variations that stem from internal (non-leaf) nodes (default: 8)
    -m, --max_moves <NUM>            Maximum number of moves (default: 10)
    -p, --path <FILE>                path/to/games.pgn
    -s, --starting_moves <STRING>    Filter games by some comma-separated starting moves, e.g. 'e4,c5'
```

Example
-------
Let's build a white repertoire against the [Sicilian Defence](https://en.wikipedia.org/wiki/Sicilian_Defence) (1. e4 c5):

```bash
wget https://raw.githubusercontent.com/zcesur/opening-repertoire/master/data/ericrosen-white.pgn
opening-repertoire -p ericrosen-white.pgn -c white -s e4,c5
```

Contributing
------------
Pull requests are welcome!

License
-------
Distributed under the BSD 3-Clause License. See the LICENSE file for more information.
