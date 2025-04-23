use pattern_loops::for_match;

#[test]
fn for_match() {
	let data = b"123456789";

	let mut num = 0usize;

	for_match!((data) -> || {
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
fn for_match_2() {
	let data = "1234567890";

	let mut data_n_index = 0usize;
	for_match!((data.as_bytes().iter()) -> || {
		Some(a) => data_n_index += *a as usize,
		_ => break,
	});

	assert_eq!(data_n_index, 525);
}
