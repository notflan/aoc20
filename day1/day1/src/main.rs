use std::collections::HashMap;

#[allow(dead_code)]
mod input;

#[cfg(feature="big")]
use input::INPUT_BIG as INPUT;
#[cfg(not(feature="big"))] 
use input::INPUT;

#[cfg(not(feature="big"))] const TARGET: u64 = 2020;
#[cfg(feature="big")] const TARGET: u64 = 99920044;

// O(n^2)
fn main() {
    let deficits: HashMap<u64, usize> = (0..)
	.zip(INPUT.iter().copied())
	.filter_map(|(i, x)| TARGET.checked_sub(x).map(|x| (x, i)))
	.collect();

    //eprintln!("calculated deficits: {}", deficits.len());
    for (i, ix) in (0..).zip(INPUT.iter().copied())
    {
	for jx in INPUT[i..].iter().copied().skip(1)
	{
	    let sum = ix+jx;
	    if let Some(&idx) = deficits.get(&sum)
	    {
		//eprintln!("solution found?");
		println!("{}", if let Some(x) = INPUT[idx].checked_mul(ix).map(|x| x.checked_mul(jx)).flatten()
			 { x } else { continue; });
		return;
	    }
	}
    }
    //panic!("no solution");
}
