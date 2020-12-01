use std::collections::HashMap;

#[allow(dead_code)]
mod input;

#[cfg(feature="big")]
use input::INPUT_BIG as INPUT;
#[cfg(not(feature="big"))] 
use input::INPUT;

#[cfg(not(feature="big"))] const TARGET: u64 = 2020;
#[cfg(feature="big")] const TARGET: u64 = 99920044;

// O(n^2) ?
fn main() {
    // doesn't work on big input?
    #[cfg(feature="u128")] 
    let input: Box<[_]> = INPUT[..].iter().map(|&x| x as u128).collect();
    #[cfg(feature="u128")] 
    let target = TARGET as u128;
    // ^ doesn't help..

    #[cfg(not(feature="u128"))] 
    let input = &INPUT[..];
    #[cfg(not(feature="u128"))] 
    let target = TARGET;
    
    let deficits: HashMap<_, usize> = (0..)
	.zip(input.iter().copied())
	.filter_map(|(i, x)| target.checked_sub(x).map(|x| (x, i)))
	.collect();

    eprintln!("calculated deficits: {}", deficits.len());
    for (i, ix) in (0..).zip(input.iter().copied())
    {
	for jx in input[i..].iter().copied().skip(1)
	{
	    if let Some(sum) = ix.checked_add(jx) {
		if let Some(&idx) = deficits.get(&sum)
		{
		    eprintln!("solution found?");
		    println!("{}", if let Some(x) = input[idx].checked_mul(ix).map(|x| x.checked_mul(jx)).flatten()
			     { x } else { continue; });
		    return;
		}
	    }
	}
    }
    panic!("no solution");
}
