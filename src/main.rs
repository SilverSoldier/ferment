/**
 * Grep clone.
 * Program which takes a file and a regex as argument and outputs the lines containing regex.
 */

extern crate ansi_term;
extern crate getopts;

use ansi_term::Color::{Green, Red, Blue};
use getopts::Options;
use std::env;
use std::fs::{File, metadata};
use std::io::BufReader;
use std::io::prelude::*;

pub fn print_usage(program: &str, opts: Options){
    let help = format!("Usage: {} [OPTIONS] PATTERN FILE [FILE1]", program);
    println!("{}", opts.usage(&help));
}

pub fn search_file(file_path: String, needle: &str){
    let mut results: Vec<(Vec<String>, u32)> = Vec::new();
    let file = File::open(file_path.clone()).unwrap_or_else(|err| {
        panic!("Unable to open {}: {}", file_path, err)
    });
    for (num, line) in (0..).zip(BufReader::new(file).lines().filter_map(move |res| res.ok())) {
        let split: Vec<String> = line.split(needle)
            .map(|x| x.to_string())
            .collect();
        if split.len() != 1 {
            results.push((split, num));
        }
    }
    let mut result_iter = results.into_iter();
    while let Some((split_string, index)) = result_iter.next() {
        print!("{}:{}: ", Red.paint(file_path.clone()), Green.paint(index.to_string()));
        for i in 0..split_string.len()-1 {
            print!("{}{}", split_string[i], Blue.bold().paint(needle));
        }
        println!("{}", split_string[split_string.len()-1])
    }
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("R", "recursive", "recursively search directories");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => panic!(f.to_string())
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let recursive = matches.opt_str("R");
    if matches.free.len() < 2 {
        print_usage(&program, opts);
        return;
    };

    let search_string: &String = &matches.free[0];
    let filename: &String = &matches.free[1];
    let md = metadata(filename).unwrap();
    if md.is_file(){
        search_file(filename.clone(), &search_string);
    } else if md.is_dir() && recursive.is_some() {
        /* List all files in directory and all will search for string */
    }
}
