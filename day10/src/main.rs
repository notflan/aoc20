#![allow(dead_code)]

#[macro_use] extern crate ad_hoc_iter;
#[macro_use] extern crate cfg_if;

use std::{
    io,
    io::BufReader,
    fs::OpenOptions,
    num::NonZeroU8,
    ops,
    iter,
    iter::Sum,
};
use smallmap::Map;

mod input;
mod diff;
use diff::Diffs;

const INPUT: &str = "input";

type Adaptors = Map<NonZeroU8, ()>;

fn parse_input(file: &str) -> Result<impl Iterator<Item = input::Int>, io::Error>
{
    let file = OpenOptions::new().read(true).open(file)?;
    Ok(input::read_input(BufReader::new(file)))
}

/// Get an iterator over the possible adaptors after the adaptor `finding`. The iterator will be between 0 and 3 elements.
/// Which adaptor has been changed *first* is increased in `diffs` if it is provided.
fn iterate_adaptor_chain(map: &Adaptors, mut diffs: Option<&mut Diffs>, finding: impl Into<u8>) -> impl Iterator<Item = u8>
{
    #![allow(unused_assignments)]

    let finding = finding.into();
    let mut set=false;    
    macro_rules! find {
	($l:literal) => {
	    {
		let shim = finding + $l;
		debug_assert!(shim > 0);
		
		match map.get(&unsafe {
		    NonZeroU8::new_unchecked(shim)
		}).map(|_| shim) {
		    Some(yes) if !set => {
			set = true;
			if let Some(diffs) = diffs.as_mut() {
			    *diffs.at_mut($l-1) += 1;
			}
			Some(yes)
		    },
		    x => x
		}
	    }
	}
    };
    iter![
	find!(1),
	find!(2),
	find!(3)
    ].filter_map(std::convert::identity)
}

#[inline] fn find_smallest(map: &Adaptors, finding: impl Into<u8>) -> (Option<u8>, Diffs)
{
    let mut diffs = Diffs::default();
    (iterate_adaptor_chain(map, Some(&mut diffs), finding).min(), diffs)
}

#[inline(always)] fn conv_input(input: impl Iterator<Item=input::Int>) -> impl Iterator<Item = (NonZeroU8, ())>
{
    input.filter_map(NonZeroU8::new).zip(iter::repeat(()))
}

#[inline(always)] fn get_input() -> impl Iterator<Item=input::Int>
{
    cfg_if!{
	if #[cfg(feature="test")]  {
	    input::test_input()
	} else {
	    parse_input(INPUT).unwrap()
	}   
    }
}

mod part2;


fn main() -> Result<(), Box<dyn std::error::Error>>
{
    
    let (all_jolts, output_rat) = {
	let mut map: Adaptors = conv_input(get_input()).collect();
	let orat = map.iter().map(|&(x, _)| x).max().unwrap().get() + 3; // rating of output device
	unsafe {
	    map.insert(NonZeroU8::new_unchecked(orat), ());
	}
	(map, orat)
    };
    debug_assert_eq!(all_jolts.num_pages(), 1); // Assert we have space efficiency.
    
    let diffs: Diffs = std::iter::successors(Some((0, Diffs::default())), |&(next, _)| {
	if let (Some(next), d) = find_smallest(&all_jolts, next) {
	    Some((next, d))
	} else {
	    None
	}
    }).map(|(_, d)| d).sum();
    
    println!("{}", diffs.jd1() * diffs.jd3());
    println!("{}", {
	//let start = all_jolts.iter().map(|&(k, _)| u8::from(k)).min().unwrap();
	part2::solve(all_jolts, output_rat)
    });
    Ok(())
}
