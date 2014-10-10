//! The package provides a parser for the
//! [TGFF](http://ziyang.eecs.umich.edu/~dickrp/tgff/) (Task Graphs For Free)
//! format, which is a format for storing task graphs and accompanying data
//! used in scheduling and allocation research.

use std::io::{File, IoResult};

pub fn parse(path: Path) -> IoResult<()> {
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    Ok(())
}
