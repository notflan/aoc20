use std::{
    io::{
	self, BufRead, BufReader, Write,
    },
    fs,
    error,
};

fn gen_input<W: Write+?Sized, R: BufRead>(output: &mut W, lines: R, const_name: &str) -> io::Result<()> {

    eprint!("Generating {}... ", const_name);
    writeln!(output, "pub const {}: &[u64] = &[", const_name)?;
    for line in lines.lines()
    {
	writeln!(output, "	{},", line?)?;
    }
    output.write_all(b"];\n")?;
    eprintln!("OK");
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>>{
    let mut output = fs::OpenOptions::new()
	.write(true)
	.truncate(true)
	.create(true)
	.open("./src/input.rs").expect("Couldn't open output");

    gen_input(&mut output, BufReader::new(fs::OpenOptions::new()
					  .read(true)
					  .open("../input").expect("Couldn't open input")), "INPUT")?;
    gen_input(&mut output, BufReader::new(fs::OpenOptions::new()
					  .read(true)
					  .open("../input-big").expect("Couldn't open big input")), "INPUT_BIG")?;
    
    Ok(())
}
