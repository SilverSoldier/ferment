/**
 * Grep clone.
 * Program which takes a file and a regex as argument and outputs the lines containing regex.
 */

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;

pub fn search_file(file_path: String, search_string: &'static str){
    let mut results: Vec<(String, u32)> = Vec::new();
    match File::open(file_path) {
        Ok(f) => {
            let reader = BufReader::new(f);
            let lines = reader.lines().map(|l| l.unwrap());
            let mut enumerate = lines.zip(0..);
            loop {
                match enumerate.next() {
                    Some ((line, num)) => {
                        if line.as_str().contains(search_string) {
                            results.push((line, num));
                        }
                    },
                    None => break
                }
            }
        },
        Err(msg) => {
            panic!("Cannot open file. {}", msg);
        }
    };
    let mut result_iter = results.into_iter();
    loop {
        match result_iter.next() {
            Some((line, num)) => {
                println!("{}| {}", num, line);
            },
            None => break
        }
    };
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    let filename: &String = &args[1];
    search_file(filename.clone(), "hello");
}
