use std::{
    io::{
	self, BufRead,
    },
    sync::{
	mpsc,
    },
    thread,
    marker::*,
    collections::{HashMap, HashSet,},
};
use generational_arena::{
    Arena, Index,
};
use super::bag;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule
{
    contents: Vec<(usize, String)>,
}

fn parse_rest(from: mpsc::Receiver<String>) -> HashMap<String, Rule>
{
    let mut out = HashMap::new();
    while let Ok(line) = from.recv() {
	
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UnlinkedBag<'a>
{
    name: &'a str,
    cont: Rule,
}

enum MaybeLinked<'a>
{
    Unlinked(UnlinkedBag<'a>),
    Linked(bag::Bag),
}

pub fn parse<R: BufRead>(from: R) -> Result<Arena<bag::Bag>, io::Error>
{
    let mut all_possible = HashSet::<String>::new();
    let (tx, rx) = mpsc::channel();
    let w = thread::spawn(move || parse_rest(rx));
    
    macro_rules! unwrap {
	(? $opt:expr) => {
	    match $opt {
		Some(v) => v,
		_ => continue,
	    }
	};
	($res:expr) => {
	    unwrap!(? $res.ok())
	}
    }
    for line in from.lines()
    {
	let mut line = line?;

	let irest = {
	    const SPLIT: &str = "bags contain";
	    let bn = {
		let idx = unwrap!(? line.find(SPLIT));
		&line[..idx]
	    };
	    all_possible.insert(bn.trim().to_owned());
	};
	unwrap!(tx.send(line));
    }
    let (mut unlinked, nref) = {
	let mut ulinks = w.join().unwrap();
	let mut unlinked = Arena::with_capacity(all_possible.len());
	let mut nref = HashMap::new();
	for name in all_possible.iter()
	{
	    let urule = ulinks.remove(name).unwrap();
	    let idx = unlinked.insert(MaybeLinked::Unlinked(UnlinkedBag{name, cont: urule}));
	    nref.insert(name, idx);
	}
	(unlinked, nref)
    };

    let indecies: Vec<_> = unlinked.iter().map(|(i, _)| i).collect();
    for idx in indecies.into_iter()
    {
	let current = unlinked.get_mut(idx).unwrap();
	let linked = match current {
	    MaybeLinked::Unlinked(UnlinkedBag{name, cont: rule}) => {
		let mut linking = bag::Bag::new(name.to_owned());
		for (_, cont) in rule.contents.iter() {
		    linking.push_contents(*nref.get(cont).unwrap());
		}
		linking
	    },
	    _=> continue,
	};
	*current = MaybeLinked::Linked(linked);
    }

    //TODO: how tf can we convert from Arena<MaybeLinked<_>> into Arena<_>?????
    
    todo!()
}
