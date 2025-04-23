use pattern_loops::{for_match, loop_match, while_match};

#[test]
fn return_values() {
	let mut index = 0;
	let loop_match = loop_match!((index, 255) -> |data| {
		0 => {
			index += 1;
			data += 1;
			break data;
		},
		_ => {
			index += 2;
			break data;
		},
	});
	assert_eq!(index, 1);
	assert_eq!(loop_match, 256);

	let for_match = for_match!((0..=15, let mut a, 255) -> |_, data| {
		Some(15) => break data,
		Some(_num) => data += 1,
		_ => break data,
	});
	assert_eq!(for_match, 270);

	let mut iter = 0..=15;
	let while_match = while_match!((iter.next(), let mut a, 255) -> |data| {
		Some(15) => break data,
		Some(_num) => data += 1,
		_ => break data,
	});
	assert_eq!(while_match, 270);
}
