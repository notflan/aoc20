use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Diffs([usize; 3]);

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

impl Sum for Diffs
{
    fn sum<I>(iter: I) -> Self
    where I: Iterator<Item=Self>
    {
	iter.fold(Default::default(), |x, y| x + y)
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
    #[inline] pub fn jd1(&self) -> usize
    {
	self.0[0]
    }
    #[inline] pub fn jd2(&self) -> usize
    {
	self.0[1]
    }
    #[inline] pub fn jd3(&self) -> usize
    {
	self.0[2]
    }
    
    #[inline] pub fn at(&self, idx: usize) -> &usize
    {
	&self.0[idx]
    }
    #[inline] pub fn at_mut(&mut self, idx: usize) -> &mut usize
    {
	&mut self.0[idx]
    }
}

impl From<(usize,usize,usize)> for Diffs
{
    fn from((f0,f1,f2): (usize,usize,usize)) -> Self
    {
	Self([f0,f1,f2])
    }
}
impl From<Diffs> for (usize,usize,usize)
{
    fn from(from: Diffs) -> Self
    {
	(from.jd1(), from.jd2(), from.jd3())
    }
}

