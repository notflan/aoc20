use std::{
    hash::{Hash, Hasher,},
    collections::HashSet,
    borrow::Borrow,
};

type Bags = HashSet<Rule>;
type BagRef = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule
{
    bag: BagRef,
    contains: Vec<(usize, BagRef)>,
}

impl Borrow<String> for Rule
{
    fn borrow(&self) -> &String {
	&self.bag
    }
}
impl Borrow<str> for Rule
{
    fn borrow(&self) -> &str {
	self.name()
    }
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
	self.bag.hash(state)
    }
}

impl Rule
{
    #[inline] pub fn name(&self) -> &str
    {
	&self.bag[..]
    }
    
    /// Find the rules for each inner bag within this context
    pub fn inner_rules<'a>(&'a self, hashes: &'a Bags) -> impl Iterator<Item = &'a Rule> + 'a
    {
	self.contains.iter().filter_map(move |(n, re)| Some(std::iter::repeat(hashes.get(re)?).take(*n))).flatten()
    }
    #[inline] pub fn new(bag: impl Into<String>, contains: impl IntoIterator<Item = (usize, String)>) -> Self
    {
	return Self::new_ex(bag.into(), contains.into_iter().collect());
    }
    pub fn new_ex(bag: String, contains: Vec<(usize, String)>) -> Self
    {
	Self {bag, contains}
    }

    pub fn all_rules<'a>(&'a self, hashes: &'a Bags) -> RuleIterator<'a>
    {
	RuleIterator
	{
	    base: self.contains.iter(),
	    hashes,
	    held: Vec::with_capacity(self.contains.len()),
	}
    }
}

#[derive(Debug)]
pub struct RuleIterator<'a>
{
    base: std::slice::Iter<'a, (usize, BagRef)>,
    hashes: &'a Bags,
    held: Vec<&'a Rule>,
}

impl<'a> RuleIterator<'a>
{
    pub fn search(self, s: impl AsRef<str>) -> Option<&'a Rule>
    {
	let s=  s.as_ref();
	for rule in self {
	    if rule.name() == s {
		return Some(rule);
	    }
	}
	None
    }
}

impl<'a> Iterator for RuleIterator<'a>
{
    type Item = &'a Rule;
    fn next(&mut self) -> Option<Self::Item>
    {
	if self.held.is_empty() {
	    match self.base.next() {
		Some((n, re)) => {
		    let ins = self.hashes.get(re).unwrap();
		    for _ in 0..*n {
			self.held.push(ins);
		    }
		},
		None => return None,
	    }
	}
	let ret = self.held.remove(0);
	self.held.extend(ret.inner_rules(self.hashes));
	Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
	(self.base.len(), None)
    }
}
impl<'a> std::iter::FusedIterator for RuleIterator<'a>{}
