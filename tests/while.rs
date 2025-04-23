use pattern_loops::while_match;

#[test]
fn while_match() {
	let data = b"123456789";

	let mut num = 0usize;

	let mut iter = data.iter();
	while_match!((iter) -> || {
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
