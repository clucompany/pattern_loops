use pattern_loops::for_match;

/*
	A classic example of using parsing a number from an arbitrary
	ASCII character set with partial trimming support.
*/

fn main() -> Result<(), String> {
	let data = b" 1234567890 ";

	let out = for_match!((data, _, 0usize) -> |_, num| {
		Some(b' ') => {},
		Some(a @ b'0' ..= b'9') => num = num * 10 + ((a - b'0') as usize),
		Some(a) => return Err(format!("Unknown byte: {:?}", a)),
		None => break num,
	});

	assert_eq!(out, 1234567890);
	Ok(())
}
