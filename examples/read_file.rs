use std::{fs::File, io::Read};
use pattern_loops::{for_match, while_match};

fn main() -> Result<(), std::io::Error> {
	let mut read_buffer = [0u8; 128];
	let mut buffer = Vec::with_capacity(130);
	let mut file = File::open("./examples/read_file.txt")?;

	while_match!((file.read(&mut read_buffer)) -> || {
		Ok(0) => break,
		Ok(len) => {
			let real_array = &read_buffer[..len];

			for_match!(@'read (real_array.iter()) -> |iter| {
				Some(13u8) => continue,
				Some(b'\n') => {
					if !buffer.is_empty() {
						println!("#line: {}", String::from_utf8_lossy(&buffer));
						buffer.clear();
					}
				},
				Some(b'#') => while_match!((iter) -> || {
					Some(b'\n') => continue 'read,
					Some(_a) => {},
					_ => break 'read,
				}),
				Some(a) => buffer.push(*a),
				_ => break,
			});
		},

		Err(e) => return Err(e),
	});
	if !buffer.is_empty() {
		println!("#line: {}", String::from_utf8_lossy(&buffer));
	}

	Ok(())
}
