This is a parser for list of nodes from Tox wiki: https://wiki.tox.chat/users/nodes


Build status: [![Build Status](https://travis-ci.org/zetok/tox-wiki-nodes-parser.svg)](https://travis-ci.org/zetok/tox-wiki-nodes-parser)


# Installation
Running it is fairly simple.

Download [binary for Linux x86_64](https://github.com/zetok/tox-wiki-nodes-parser/releases/download/v0.0.0/tox-wiki-nodes-parser) and run it.


To compile yourself:

1. Install [Rust](http://www.rust-lang.org/)
2. Make with `cargo build`
3. Run with `./target/debug/./tox-wiki-nodes-parser`

# Usage

Parser takes content of file `nodes.txt` from working directory and prints to stdout parsed list of nodes.

Provide file `nodes.txt` in working directory, with content from wiki, and run parser.

You can do that by e.g. running this script:

```bash
wget https://github.com/zetok/tox-wiki-nodes-parser/releases/download/v0.0.0/tox-wiki-nodes-parser && \
wget https://wiki.tox.chat/users/nodes?do=edit -O nodes.txt && \
chmod +x tox-wiki-nodes-parser && \
./tox-wiki-nodes-parser
```


# License

Licensed under GPLv3+, for details see [COPYING](/COPYING).