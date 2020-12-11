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
};
use smallmap::Map;
mod input;

const INPUT: &str = "input";

type Adaptors = Map<NonZeroU8, ()>;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Diffs([usize; 3]);

impl ops::Add for Diffs
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output{
	Self([
	    self.0[0] + other.0[0],
	    self.0[1] + other.0[1],
	    self.0[2] + other.0[2],
	])
    }
}

impl ops::AddAssign for Diffs
{
    fn add_assign(&mut self, rhs: Self) {
	self.0[0] += rhs.0[0];
	self.0[1] += rhs.0[1];
	self.0[2] += rhs.0[2];
    }
}

impl Diffs
{
    pub fn jd1(&self) -> usize
    {
	self.0[0]
    }
    pub fn jd2(&self) -> usize
    {
	self.0[1]
    }
    pub fn jd3(&self) -> usize
    {
	self.0[2]
    }
}

fn parse_input(file: &str) -> Result<impl Iterator<Item = input::Int>, io::Error>
{
    let file = OpenOptions::new().read(true).open(file)?;
    Ok(input::read_input(BufReader::new(file)))
}

fn find_smallest(map: &Adaptors, finding: impl Into<u8>) -> (Option<u8>, Diffs)
{
    #![allow(unused_assignments)]

    let finding = finding.into();
    let mut diffs = Diffs::default();
    let mut set=false;    
    macro_rules! find {
	($l:literal) => {
	    {
		let shim = finding + $l;
		debug_assert!(shim > 0);
		//eprintln!("Looking up {}", shim);
		match map.get(&unsafe {
		    NonZeroU8::new_unchecked(shim)
		}).map(|_| shim) {
		    Some(yes) if !set => {
			set = true;
			diffs.0[($l-1)] += 1;
			Some(yes)
		    },
		    x => x
		}
	    }
	}
    };
    (iter![
	find!(1),
	find!(2),
	find!(3)
    ].filter_map(std::convert::identity)
     .min(), diffs)
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
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let all_jolts = {
	let mut map: Adaptors = conv_input(get_input()).collect();
	unsafe {
	    map.insert(NonZeroU8::new_unchecked(map.iter().map(|&(x, _)| x).max().unwrap().get() + 3), ());
	}
	map
    };
    debug_assert_eq!(all_jolts.num_pages(), 1); // Assert we have space efficiency.

    let mut current = 0;
    #[allow(unused_variables)]
    let mut sum=0;
    let mut whole_diffs = Diffs::default();
    loop {
	if let (Some(next), diffs) = find_smallest(&all_jolts, current) {
	    current = next;
	    sum+=1;
	    whole_diffs += diffs;
	} else {
	    break;
	}
    }

    println!("{}", whole_diffs.jd1() * whole_diffs.jd3());
    Ok(())
}
