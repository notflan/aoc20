use std::{
    fmt, error,
};

#[derive(Debug)]
pub struct InfoParseError;

impl error::Error for InfoParseError{}
impl fmt::Display for InfoParseError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "failed to parse passport info structure")
    }
}

// --- 

#[derive(Debug)]
pub enum BoundedU16FromStrError
{
    Parse,
    Bound(std::ops::RangeInclusive<usize>),
}

impl error::Error for BoundedU16FromStrError{}
impl fmt::Display for BoundedU16FromStrError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	match self {
	    Self::Parse => write!(f, "failed to parse u8"),
	    Self::Bound(b) => write!(f, "value was not in bounds {:?}", b),
	}
    }
}

#[derive(Debug)]
pub enum HeightFromStrError
{
    Parse(BoundedU16FromStrError),
    Unit,
}
impl error::Error for HeightFromStrError
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	match &self
	{
	    Self::Parse(s) => Some(s),
	    _ => None,
	}
    }
}
impl fmt::Display for HeightFromStrError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	match self {
	    Self::Unit => write!(f, "invalid/unknown height unit"),
	    Self::Parse(_) => write!(f, "number was not in bounds for unit"),
	}
    }
}



#[derive(Debug)]
pub struct HairColourFromStrError;

impl error::Error for HairColourFromStrError{}
impl fmt::Display for HairColourFromStrError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "invalid hair colour")
    }
}


#[derive(Debug)]
pub struct EyeColourFromStrError;

impl error::Error for EyeColourFromStrError{}
impl fmt::Display for EyeColourFromStrError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "invalid eye colour")
    }
}


#[derive(Debug)]
pub struct PassportIdFromStrError;

impl error::Error for PassportIdFromStrError{}
impl fmt::Display for PassportIdFromStrError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "invalid passport ID")
    }
}


// --

#[derive(Debug)]
pub enum PassportParseError
{
    PassportID(PassportIdFromStrError),
    EyeColour(EyeColourFromStrError),
    HairColour(HairColourFromStrError),
    Height(HeightFromStrError),
    ExprYear(BoundedU16FromStrError),
    IssueYear(BoundedU16FromStrError),
    BirthYear(BoundedU16FromStrError),
}

impl error::Error for PassportParseError
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
	use PassportParseError::*;
	#[allow(non_snake_case)]
	Some(match &self {
	    PassportID(passportIdFromStrError) => passportIdFromStrError,
	    EyeColour(eyeColourFromStrError) => eyeColourFromStrError,
	    HairColour(hairColourFromStrError) => hairColourFromStrError,
	    Height(heightFromStrError) => heightFromStrError,
	    ExprYear(boundedU8FromStrError) => boundedU8FromStrError,
	    IssueYear(boundedU8FromStrError) => boundedU8FromStrError,
	    BirthYear(boundedU8FromStrError) => boundedU8FromStrError,
	})
    }
}

impl fmt::Display for PassportParseError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	use PassportParseError::*;
	write!(f, "invalid ")?;
	match self {
	    PassportID(_) => write!(f, "passport id"),
	    EyeColour(_) => write!(f, "eye colour"),
	    HairColour(_) =>write!(f, "hair colour"), 
	    Height(_) => write!(f, "height"),
	    ExprYear(_) => write!(f, "expire year"),
	    IssueYear(_) => write!(f, "issue year"),
	    BirthYear(_) => write!(f, "birth year"),
	}?;
	write!(f, ": ")?;
	use error::Error;
	write!(f, "{}", self.source().unwrap())
    }
}
