use std::{
    ops::RangeInclusive,
    str,
    fmt,
    error,
};
use smallmap::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Policy
{
    letter: char,
    rep: RangeInclusive<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PolicyV2
{
    letter: char,
    i: usize,
    j: usize,
}

impl PolicyV2
{
    #[inline] pub fn validate_str(&self, s: &str) -> bool
    {
	self.validate(s.chars().collect::<Vec<_>>())
    }
    pub fn validate(&self, vec: impl AsRef<[char]>) -> bool
    {
	let vec = vec.as_ref();
	if self.i>=vec.len() || self.j>=vec.len() { return false; }
	
	match (vec[self.i], vec[self.j]) {
	    (l, ll) if l == ll => false,
	    (x, _) | (_, x) if x == self.letter => true,
	    _ => false,
	}
    }
}

impl Policy
{
    #[inline] pub fn into_v2(self) -> PolicyV2
    {
	PolicyV2
	{
	    letter: self.letter,
	    i: (*self.rep.start())-1,
	    j: (*self.rep.end())-1,
	}
    }

    #[inline(always)] pub fn validate_str(&self, s: impl AsRef<str>) -> bool
    {
	self.validate(s.as_ref().chars())
    }
    
    pub fn validate(&self, chars: impl IntoIterator<Item=char>) -> bool
    {
	let map: Map<char, usize> = {
	    let chars = chars.into_iter();
	    let mut mp = Map::new();
	    for ch in chars
	    {
		let cnt = mp.entry(ch).or_insert(0);
		*cnt += 1;
	    }
	    mp
	};
	if let Some(n) = map.get(&self.letter)
	{
	    self.rep.contains(n)
	} else {
	    false
	}
    }
}

impl fmt::Display for Policy
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "{}-{} {}", self.rep.start(), self.rep.end(), self.letter)
    }
}


impl str::FromStr for Policy
{
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
	let mut first = s.split('-');
	let num1 = first.next().ok_or(FromStrError)?;
	let (num2, letter) = {
	    let num2_rest = first.next().ok_or(FromStrError)?;
	    let mut rest = num2_rest.split(' ');
	    (rest.next().ok_or(FromStrError)?, rest.next().ok_or(FromStrError)?)
	};
	Ok(Self{
	    letter: letter.chars().next().ok_or(FromStrError)?,
	    rep: (num1.parse()?)..=(num2.parse()?),
	})
    }
}

#[derive(Debug)]
pub struct FromStrError;

impl From<std::num::ParseIntError> for FromStrError
{
    #[inline] fn from(_: std::num::ParseIntError) -> Self
    {
	Self
    }
}

impl error::Error for FromStrError{}
impl fmt::Display for FromStrError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "failed to parse policy rule")
    }
}
