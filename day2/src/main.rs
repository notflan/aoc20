#![feature(str_split_once)]

use std::{
    fs,
    io::{
	BufReader,
	BufRead,
    },
    error,
};

const INPUT_FILE: &str = "input";

mod rule;

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = BufReader::new(fs::OpenOptions::new()
			       .read(true)
			       .open(INPUT_FILE)?);
    let mut ok=0;
    let mut ok2 =0;
    for line in input.lines()
    {
	if let Some((rule, pass)) = line?.split_once(':')
	{
	    let rule: rule::Policy = rule.parse()?;
	    let pass = pass.trim();
	    if rule.validate_str(pass) {
		ok+=1;
	    }
	    if rule.into_v2().validate_str(pass) {
		ok2+=1;
	    }
	}
    }
    println!("{}", ok);
    println!("{}", ok2);
    Ok(())
}
