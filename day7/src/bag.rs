use std::{
    mem,
    ops,
    hash::{Hash, Hasher,},
};
use generational_arena::{
    Arena,
    Index,
};

#[derive(Debug, Clone, Eq)]
pub struct Bag
{
    name: String,
    contains: Vec<Index>,
}

impl Hash for Bag {
    #[inline] fn hash<H: Hasher>(&self, state: &mut H) {
	self.as_ref().hash(state)
    }
}

impl<T> PartialEq<T> for Bag
where T: AsRef<BagRef>
{
    fn eq(&self, other: &T) -> bool
    {
	self.as_ref() == other.as_ref()
    }
}

impl Bag
{
    pub fn new(name: String) -> Self
    {
	Self {
	    name,
	    contains: Vec::new(),
	}
    }
    pub fn push_contents(&mut self, idx: Index)
    {
	self.contains.push(idx)
    }
    pub fn bags_in<'a>(&'a self, w: &'a Arena<Bag>) -> impl Iterator<Item = &'a BagRef> + 'a
    {
	self.contains.iter().filter_map(move |x| w.get(x.clone())).map(Self::as_ref)
    }
    pub fn contains_in(&self, w: &Arena<Bag>,  bag: impl AsRef<BagRef>) -> bool
    {
	for x in self.bags_in(w)
	{
	    if x == bag.as_ref() {
		return true;
	    }
	}
	false
    }
    
}

impl AsRef<BagRef> for Bag
{
    #[inline] fn as_ref(&self) -> &BagRef
    {
	BagRef::new_unchecked(&self.name[..])
    }
}

impl ops::Deref for Bag
{
    type Target = BagRef;
    #[inline] fn deref(&self) -> &Self::Target {
	self.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BagRef
{
    name: str
}

impl AsRef<BagRef> for BagRef
{
    #[inline] fn as_ref(&self) -> &BagRef
    {
	self
    }
}


impl BagRef
{
    #[inline] fn new_unchecked<'a>(from: &'a str) -> &'a BagRef
    {
	unsafe {
	    mem::transmute(from)
	}
    }
}
