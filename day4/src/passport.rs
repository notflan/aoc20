use std::{
    convert::TryFrom,
    collections::HashMap,
    str,
};
use super::error::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PassportInfo<'a>
{
    birth_year: &'a str,
    issue_year: &'a str,
    expr_year: &'a str,
    height: &'a str,
    hair_colour: &'a str,
    eye_colour: &'a str,
    pp_id: &'a str,
    cn_id: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for PassportInfo<'a>
{
    type Error = InfoParseError;

    fn try_from(from: &'a str) -> Result<Self, Self::Error>
    {
	let mut items = HashMap::new();
	for item in from.split_whitespace() {
	    if let Some((k, v)) = item.split_once(':') {
		if items.insert(k, v).is_some() {
		    return Err(InfoParseError);
		}
	    }
	}
	macro_rules! parse {
	    (? $l:literal) => {
		items.get($l).map(|&x| x)
	    };
	    ($l:literal) => {
		parse!(? $l).ok_or(InfoParseError)?;
	    };
	}
	
	Ok(Self{
	    birth_year: parse!("byr"),
	    issue_year: parse!("iyr"),
	    expr_year: parse!("eyr"),
	    height: parse!("hgt"),
	    hair_colour: parse!("hcl"),
	    eye_colour: parse!("ecl"),
	    pp_id: parse!("pid"),
	    cn_id: parse!(? "cid"),
	})
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundedU16<const LOW: usize, const HIGH: usize>
{
    value:u16
}

impl<const LOW: usize, const HIGH:usize> BoundedU16<LOW, HIGH>
{
    pub const fn bound() -> std::ops::RangeInclusive<usize>
    {
	LOW..=HIGH
    }
    pub const fn contains(value: u16) -> bool
    {
	let value = value as usize;
	value >= LOW && value <= HIGH
    }
}


impl<const LOW: usize, const HIGH:usize> str::FromStr for BoundedU16<LOW, HIGH>
{
    type Err = BoundedU16FromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let v: u16 = s.parse().map_err(|_| BoundedU16FromStrError::Parse)?;
	//println!("{:?}: {}", Self::bound(), v);
	if Self::contains(v) {
	    Ok(Self{value: v})
	} else {
	    Err(BoundedU16FromStrError::Bound(Self::bound()))
	}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Height
{
    Cm(BoundedU16<150, 193>),
    In(BoundedU16<56, 76>),
}

impl str::FromStr for Height
{
    type Err = HeightFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	if s.ends_with("cm")
	{
	    let num = &s[..(s.len()-2)];
	    Ok(Self::Cm(num.parse().map_err(HeightFromStrError::Parse)?))
	} else if s.ends_with("in")
	{
	    let num = &s[..(s.len()-2)];
	    Ok(Self::In(num.parse().map_err(HeightFromStrError::Parse)?))
	} else {
	    Err(HeightFromStrError::Unit)
	}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HairColour(String);

impl str::FromStr for HairColour
{
    type Err = HairColourFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	const HEX: &[u8] = b"0123456789abcdef";
	lazy_static::lazy_static! {
	    static ref VALID_CHARS: smallmap::Map<u8, ()> = {
		let mut map = smallmap::Map::new();
		for &b in HEX.iter()
		{
		    map.insert(b, ());
		}
		map
	    };
	}
	
	if s.starts_with("#") && s.len() == 7 {
	    let s = &s[1..];
	    let bytes = s.as_bytes();
	    for b in bytes.iter()
	    {
		if !VALID_CHARS.contains_key(b) {
		    return Err(HairColourFromStrError);
		}
	    }
	    Ok(Self(s.to_owned()))
	} else {
	    Err(HairColourFromStrError)
	}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u8)]
pub enum EyeColour
{
    Amb, Blu, Brn, Gry, Grn, Hzl, Oth,
}

impl str::FromStr for EyeColour
{
    type Err = EyeColourFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	Ok(match s.trim() {
	    "amb" => Self::Amb,
	    "blu" => Self::Blu,
	    "brn" => Self::Brn,
	    "gry" => Self::Gry,
	    "grn" => Self::Grn,
	    "hzl" => Self::Hzl,
	    "oth" => Self::Oth,
	    _ => return Err(EyeColourFromStrError),
	})
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PassportId
{
    id: u64,
}

impl str::FromStr for PassportId
{
    type Err = PassportIdFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	if s.len() == 9 {
	    Ok(Self{
		id: s.parse().map_err(|_| PassportIdFromStrError)?,
	    })
	} else {
	    Err(PassportIdFromStrError)
	}
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Passport
{
    birth_year: BoundedU16<1920, 2002>,
    issue_year: BoundedU16<2010, 2020>,
    expr_year: BoundedU16<2020, 2030>,
    height: Height,
    hair_colour: HairColour,
    eye_colour: EyeColour,
    pp_id: PassportId,
    cn_id: Option<()>,
}

impl<'a> TryFrom<PassportInfo<'a>> for Passport
{
    type Error = PassportParseError;

    fn try_from(from: PassportInfo<'a>) -> Result<Self, Self::Error>
    {
	Ok(Self{
	    birth_year: from.birth_year.parse().map_err(PassportParseError::BirthYear)?,
	    issue_year: from.issue_year.parse().map_err(PassportParseError::IssueYear)?,
	    expr_year: from.expr_year.parse().map_err(PassportParseError::ExprYear)?,
	    height: from.height.parse().map_err(PassportParseError::Height)?,
	    hair_colour: from.hair_colour.parse().map_err(PassportParseError::HairColour)?,
	    eye_colour: from.eye_colour.parse().map_err(PassportParseError::EyeColour)?,
	    pp_id: from.pp_id.parse().map_err(PassportParseError::PassportID)?,
	    cn_id: from.cn_id.map(|_| ()),
	})
    }
}
