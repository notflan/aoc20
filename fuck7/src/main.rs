#![feature(str_split_once)]

#![allow(dead_code)]

use std::{
    io::BufReader,
    fs::OpenOptions,
    collections::HashSet,
};

#[cfg(feature="test")] 
const INPUT: &str ="input-test2";
#[cfg(not(feature="test"))]
const INPUT: &str = "input";

const NEEDLE: &str = "shiny gold";

mod parse;
mod bag;

fn cnt_rules(rule: &bag::Rule, set: &HashSet<bag::Rule>) -> usize
{
    let mut already_done: HashSet<&bag::Rule> = HashSet::new();
    rule.all_rules(set).map(|x| {
	let mut o = cnt_rules(x, set) + x.inner_rules(set).count();

	if already_done.insert(x) || true {
	    if o > 0 {
		eprintln!("{:?} -> {}", x, o);
		o+=1;
	    } else if !already_done.contains(x) {
		eprintln!("{:?} -> {}", x ,1);
		return 1;
	    }
	    o
	} else {
	    0
	}
    }).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed: HashSet<_> = parse::parse(BufReader::new(OpenOptions::new().read(true).open(INPUT)?)).collect();

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

    //#[cfg(feature="part2")]
    {
	println!("{}", cnt_rules(parsed.get(NEEDLE).unwrap(), &parsed));
    }
    Ok(())
}
