/**
 * Grep clone.
 * Program which takes a file and a regex as argument and outputs the lines containing regex.
 */

extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;

const USAGE: &'static str = "
Congrep.

Usage:
    ./congrep -i <input> -r <regex>

Options:
-h --help   Show this screen
--version   Show version
-i --input  Input file name/stdin
-r --regex  Regex to match
";

#[derive(Debug, Deserialize)]
struct Args {
    input: String,
    regex: String,
}

pub fn main() {
    /* Get the command line arguments */
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
