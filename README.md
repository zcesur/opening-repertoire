opening-repertoire
==========

A Rust library that generates chess opening repertoires from chess games in PGN notation.

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
