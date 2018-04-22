/**
 * Grep clone.
 * Program which takes a file and a regex as argument and outputs the lines containing regex.
 */

extern crate ansi_term;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use ansi_term::Color::{Green, Blue};

pub fn search_file(file_path: String, needle: &'static str){
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
        print!("{}| ", Blue.paint(index.to_string()));
        for i in 0..split_string.len()-1 {
            print!("{}{}", split_string[i], Green.paint(needle));
        }
        println!("{}", split_string[split_string.len()-1])
    }
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    let filename: &String = &args[1];
    search_file(filename.clone(), "hello");
}
