extern crate clap;
extern crate colored;
extern crate regex;

use clap::{App, Arg};
use colored::*;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = App::new("greplite")
        .version("0.1")
        .about("searches for stuff")
        .arg(
            Arg::with_name("pattern")
                .help("the pattern")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("file to search")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("context")
                .long("context")
                .short("C")
                .takes_value(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");
    let context_ = args.value_of("context").unwrap_or("0");
    let context = context_.parse::<usize>().unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re, context);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re, context);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex, context_lines: usize) {
    let mut lines: Vec<String> = Vec::new();
    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, &String, bool)>> = Vec::new();

    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => {
                tags.push(i);
                let v = Vec::with_capacity(2 * context_lines + 1);
                ctx.push(v);
            }
            _ => (),
        }
        lines.push(line);
    }

    if tags.len() == 0 {
        return;
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;
            if (i >= lower_bound) && (i <= upper_bound) {
                let is_match = i == *tag;
                let local_ctx = (i, line, is_match);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line, is_match) in local_ctx.iter() {
            let line_num = i + 1;
            let line_coloured = match is_match {
                true => line.red(),
                false => line.normal(),
            };
            println!("{}: {}", line_num, line_coloured);
        }
        println!("");
    }
}
