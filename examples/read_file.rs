use std::{fs::File, io::Read};
use pattern_loops::{for_match, while_match};
use std::{
	fs::File,
	io::{Error, Read},
};

/*
	Example of block-by-block reading of a file consisting of many lines,
	with support for comments and the exclusion of unnecessary control characters
*/

fn main() -> Result<(), Error> {
	let mut read_buffer = [0u8; 128];
	let mut buffer = Vec::with_capacity(130);
	let mut file = File::open("./examples/read_file.txt")?;

	while_match!(@'next_readfile (file.read(&mut read_buffer)) -> || {
		Ok(0) => break 'next_readfile, // end
		Ok(len) => for_match!(@'read_cycle (read_buffer[..len].iter()) -> |iter| {
			// symbols that are absolutely not interesting,
			// but should not be included in the line
			Some(b'\r') => continue 'read_cycle,
			// new line
			Some(b'\n') if !buffer.is_empty() => show_line_and_clear_buffer(&mut buffer),
			Some(b'\n') => {},
			// implements support for unix comments in a file that do
			// not affect the data in any way
			Some(b'#') => while_match!((iter) -> || {
				Some(b'\n') => continue 'read_cycle,
				Some(..) => {},
				// if we run out of data, we try to read the file further.
				// if there is enough data, the cycle continues
				None => match file.read(&mut read_buffer) {
					Ok(0) => break 'next_readfile, // end
					Ok(len) => iter = read_buffer[..len].iter(),
					Err(e) => return Err(e),
				}
			}),
			// write useful data
			Some(a) => buffer.push(*a),
			_ => break,
		}),

		Err(e) => return Err(e),
	});
	if !buffer.is_empty() {
		show_line_and_clear_buffer(&mut buffer);
	}

	Ok(())
}

pub fn show_line_and_clear_buffer(buff: &mut Vec<u8>) {
	println!("#line: {}", String::from_utf8_lossy(buff));
	buff.clear();
}