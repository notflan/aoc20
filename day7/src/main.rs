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

const NEEDLE: &str = "shiny gold";

mod parse;
mod bag;

fn cnt_rules(rule: &bag::Rule, set: &HashSet<bag::Rule>) -> usize
{
    match rule.children() {
	&[] => 0,
	children => {
	    children.iter().map(|(n, b)| *n + (*n * cnt_rules(set.get(b).unwrap(), set))).sum::<usize>()
	},
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed: HashSet<_> = parse::parse(BufReader::new(OpenOptions::new().read(true).open(INPUT)?)).collect();

    #[cfg(not(feature="part2"))] 
    {
	let mut found=0;
	for rule in parsed.iter() {
	    #[cfg(debug_assertions)]
	    eprintln!("{:?}", rule);

	    if rule.name() != NEEDLE {
		if rule.all_rules(&parsed).search(NEEDLE).is_some() {
		    found +=1;
		} 
	    }
	}

	println!("{}", found);
    }
    #[cfg(feature="part2")] println!("{}", cnt_rules(parsed.get(NEEDLE).unwrap(), &parsed));
    
    Ok(())
}
