use std::{
    io::{
	BufRead,
    },
};

pub fn test(input: impl IntoIterator<Item=Int>)
{
    for x in input
    {
	if !(0..=255).contains(&x) {
	    panic!("Overflew u8: {}", x);
	} else {
	    println!(" -> {} OK", x);
	}
    }
}


pub type Int = u8;

/// Read the real input from a file
pub fn read_input<'r, R: BufRead+ 'r>(from: R) -> impl Iterator<Item=Int> + 'r
{
    from.lines().filter_map(|x| x.ok().map(|y| y.parse().ok()).flatten())
}

/// Get the test input
pub fn test_input() -> impl Iterator<Item=Int>
{
    return TEST_INPUT_2.iter().copied()
}

const TEST_INPUT: &[Int] = &[
    16,
    10,
    15,
    5,
    1,
    11,
    7,
    19,
    6,
    12,
    4,
];

const TEST_INPUT_2: &[Int] = &[
    28,
    33,
    18,
    42,
    31,
    14,
    46,
    20,
    48,
    47,
    24,
    23,
    49,
    45,
    19,
    38,
    39,
    11,
    1,
    32,
    25,
    35,
    8,
    17,
    7,
    9,
    4,
    2,
    34,
    10,
    3, 
];
