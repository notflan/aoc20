#![feature(str_split_once)]

#![allow(dead_code)]

use std::{
    io::BufReader,
    fs::OpenOptions,
    collections::HashSet,
};

#[cfg(feature="test")] 
const INPUT: &str ="input-test";
#[cfg(not(feature="test"))]
const INPUT: &str = "input";


mod parse;
mod bag;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed: HashSet<_> = parse::parse(BufReader::new(OpenOptions::new().read(true).open(INPUT)?)).collect();
    #[cfg(debug_assertions)]
    for x in parsed.iter() {
	eprintln!("{:?}", x);
    }

    
    Ok(())
}
