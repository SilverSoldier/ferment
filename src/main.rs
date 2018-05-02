/**
 * Grep clone.
 * Program which takes a file and a regex as argument and outputs the lines containing regex.
 */

extern crate ansi_term;
extern crate getopts;

use ansi_term::Color::{Green, Red, Blue, Yellow};
use getopts::Options;
use std::env;
use std::fs::{File, metadata, read_dir};
use std::io::BufReader;
use std::io::prelude::*;
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};

pub fn spawn_thread(search_string: &str) -> (Sender<String>, JoinHandle<()>){
    let needle = search_string.to_string();
    let (tx, rx) = channel();
    let handler: JoinHandle<_> = thread::spawn(move || {
        thread_task(rx, &needle);
    });
    (tx, handler)
}

pub fn thread_task(rx: Receiver<String>, search_string: &str){
    loop {
        let file_name = match rx.recv() {
            Ok(f) => f,
            Err(_) => return
        };
        search_file(file_name, search_string);
    }
}

pub fn search_file(file_path: String, needle: &str){
    let mut results: Vec<(Vec<String>, u32)> = Vec::new();
    let file = match File::open(file_path.clone()) {
        Ok(f) => f,
        Err(err) => {
            println!("Unable to open {}: {}", Red.bold().paint(file_path), err);
            return ;
        }
    };
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
        print!("{}:{}: ", Yellow.paint(file_path.clone()), Green.paint(index.to_string()));
        for i in 0..split_string.len()-1 {
            print!("{}{}", split_string[i], Blue.bold().paint(needle));
        }
        println!("{}", split_string[split_string.len()-1])
    }
}

pub fn print_usage(program: &str, opts: Options){
    let help = format!("Usage: {} [OPTIONS] PATTERN FILE [FILE1]", program);
    println!("{}", opts.usage(&help));
}

pub fn give_jobs(workers: &Vec<(Sender<String>, JoinHandle<()>)>, mut worker: usize, jobs: Vec<String>, recursive: bool, n_threads: usize) -> usize {
    /* TODO: keep hashmap of files done, so as to not search already searched files */
    for file in jobs {
        let md = match metadata(file.clone()) {
            Ok(m) => m,
            Err(msg) => {
                println!("Error for {}: {}", file, msg);
                continue;
            }
        };
        if md.is_file(){
            let (ref tx, _) = workers[worker];
            tx.send(file).unwrap();
            worker = (worker + 1) % n_threads;
        }
        else if md.is_dir() && !recursive {
            println!("ferment: {}: Is a directory", Red.bold().paint(file));
        }
        else if md.is_dir() {
            let paths = read_dir(file).unwrap().map(|p| String::from(p.unwrap().path().to_str().unwrap())).collect();
            worker = give_jobs(&workers, worker, paths, recursive, n_threads);
        }
    }
    worker
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("r", "recursive", "read all files under each directory recursively.");
    opts.optopt("j", "jobs", "number of cores to use. If unspecified, assumes 1 core.", "JOBS");
    let mut matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => {
            println!("{}.", f);
            print_usage(&program, opts);
            return;
        }
    };

    if matches.opt_present("h") || matches.free.len() < 2 {
        print_usage(&program, opts);
        return;
    };

    let recursive = matches.opt_present("r");
    let n_threads = match matches.opt_str("j") {
        Some(num) => match num.parse::<usize>() {
            Ok(n) => n,
            Err(msg) => {
                println!("Number of jobs ({}) should be an integer. {}", num, msg);
                return;
            }
        },
        None => 1
    };

    let search_string = matches.free.remove(0);

    let mut workers = Vec::with_capacity(n_threads as usize);

    /* Spawn one thread */
    for i in 0..n_threads {
        workers.insert(i, spawn_thread(&search_string));
    }

    give_jobs(&workers, 0, matches.free, recursive, n_threads);

    for (tx, handler) in workers {
        drop(tx);
        handler.join().unwrap();
    }
}
