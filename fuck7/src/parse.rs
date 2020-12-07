use std::io::BufRead;
use super::bag;

fn parse_rule(from: impl AsRef<str>) -> Option<bag::Rule>
{
    let from = from.as_ref();
    let mut ps = from.split("bag");
    
    let name = ps.next()?.trim();
    Some(bag::Rule::new(name, (0..).zip(ps).filter_map(|(i, s)| {
	let (n, s) = if i == 0 {
	    const JUNK: &str = "s contain ";
	    
	    let (spec, rest)  = (&s[JUNK.len()..]).split_once(char::is_whitespace).unwrap();
	    let n: usize = match spec {
		"no" => 0,
		x => x.parse().unwrap(),
	    };
	    (n, rest.trim())
	} else {
	    if s.contains(".") { return None; }
	    
	    let s = if s.starts_with("s") {
		&s[3..]
	    } else if s.starts_with(",") {
		&s[2..]
	    } else {
		s
	    }.trim();
	    //if s.starts_with(".") { return None; }

	    let (spec, rest)  = s.split_once(char::is_whitespace).unwrap();
	    (spec.parse().unwrap(), rest.trim())
	};
	if n < 1 {
	    None
	} else {
	    Some((n, s.to_owned()))
	}
    })))
}

pub fn parse<R: BufRead>(buf: R) -> impl Iterator<Item = bag::Rule>
{
    buf.lines().filter_map(|x| {
	x.ok().map(parse_rule).flatten()
    })
}
