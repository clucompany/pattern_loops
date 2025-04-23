use pattern_loops::loop_match;

#[test]
fn loop_match() {
	let data = b"123456789";

	let mut num = 0usize;

	let mut iter = data.iter();
	loop_match!((iter.next()) -> || {
		Some(b'0') => {},
		Some(a @ b'1' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!("Unk byte: {:?}", a),
		_ => break
	});

	assert_eq!(num, 123456789);
}

#[test]
fn loop_match_2() {
	let data = "1234567890";

	let mut iter = data.as_bytes().iter();
	let data_n_index = loop_match!((iter.next(), 0usize) -> |data_n_index| {
		Some(a) => data_n_index += *a as usize,
		_ => break data_n_index,
	});

	assert_eq!(data_n_index, 525);
}
