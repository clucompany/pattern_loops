use pattern_loops::for_match;

fn main() {
	let data = "12345678901";

	let mut a;
	let result = for_match!(@'begin (data.as_bytes().iter(), a, 0usize) -> |iter, num| {
		Some(b'0') => {},
		Some(b'1') => num += 1,
		Some(b'2') => num += 2,
		Some(b'3') => num += 3,
		Some(b'4') => num += 4,
		Some(b'5') => num += 5,
		Some(b'6') => num += 6,
		Some(b'7') => num += 7,
		Some(b'8') => num += 8,
		Some(b'9') => break 'begin num,
		Some(a) => panic!("Unk byte '{:?}'", a),
		_ => break 'begin num,
	});

	assert_eq!(a, Some(&b'9'));
	assert_eq!(result, 36);
}
