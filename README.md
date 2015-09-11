This is a parser for list of nodes from Tox wiki: https://wiki.tox.chat/users/nodes


Build status: [![Build Status](https://travis-ci.org/zetok/tox-wiki-nodes-parser.svg)](https://travis-ci.org/zetok/tox-wiki-nodes-parser)


# Installation
Running it is fairly simple.

Download [binary for Linux x86_64](https://github.com/zetok/tox-wiki-nodes-parser/releases/download/v0.0.1/tox-wiki-nodes-parser) and run it.


To compile yourself:

1. Install [Rust](http://www.rust-lang.org/)
2. Make with `cargo build`
3. Run with `./target/debug/./tox-wiki-nodes-parser`

# Usage

Parser downloads the list of nodes from ``https://wiki.tox.chat/users/nodes?do=edit`` and prints to stdout parsed list of nodes.


# License

Licensed under GPLv3+, for details see [COPYING](/COPYING).