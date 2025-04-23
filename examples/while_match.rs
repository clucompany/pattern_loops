use pattern_loops::while_match;

fn main() {
	let data = b"123456789";

	let mut num = 0usize;

	let mut iter = data.iter();
	while_match!((iter) -> || {
		Some(b'0') => {},
		Some(a @ b'1' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!("Unknown byte: {:?}", a),
		_ => break
	});

	/*
	while_match!(@'begin (data.iter(), _, 1024usize, 10, "test") -> |a, b, c| {
		_ => break,
	});
	*/

	assert_eq!(num, 123456789);
}
