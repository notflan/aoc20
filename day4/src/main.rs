#![feature(str_split_once)]
#![feature(min_const_generics)]

use std::{
    fs,
    io::{
	self,
	BufReader,
	BufRead,
    },
    convert::TryFrom,
};

#[cfg(not(feature="test"))] const INPUT: &str = "input";
#[cfg(feature="test")] const INPUT: &str = "input-test";

mod error;
mod passport;

fn parse_single(from: &mut (impl BufRead + ?Sized)) -> Result<String, io::Error>
{
    let mut string = String::new();
    loop {
	let n = string.len();
	let r = from.read_line(&mut string)?;
	let nw = &string[n..(n+r)];
	if nw.trim().len()==0 {
	    break;
	}
    }
    Ok(string)
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut input = BufReader::new(fs::OpenOptions::new()
        .read(true)
        .open(INPUT)?);

    let mut valid = 0;
    loop {
	let line = parse_single(&mut input)?;
	if line.trim().len() > 0 {
	    #[allow(unused_variables)]
	    if let Ok(ppinf) = passport::PassportInfo::try_from(&line[..]) {
		#[cfg(not(feature="part2"))] { valid+=1; }
		#[cfg(feature="part2")] {
		    match passport::Passport::try_from(ppinf) {
			Err(err) => {
			    eprintln!("Error: {}", err);			    
			},
			Ok(_) => valid+=1,
		    }
		}
	    }
	} else {
	    break;
	}
    }
    println!("{}", valid);
    Ok(())
}
