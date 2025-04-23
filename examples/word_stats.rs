use pattern_loops::{for_match, loop_match, while_match};

/*
	A simple example of counting specific characters used in a text.
*/

fn main() -> Result<(), &'static str> {
	let s_indata = "
	
	/*
	tabs:
		1 History

		1.1 Nupedia
		1.2 Launch and early growth
		1.3 Milestones
	*/
	
	Wikipedia:
	
	Wikipedia[b] is a free online encyclopedia, written and maintained by a community of 
	volunteers, known as Wikipedians, through open collaboration and the wiki software MediaWiki. 
	Founded by Jimmy Wales and Larry Sanger on January 15, 2001, 
	Wikipedia has been hosted since 2003 by the Wikimedia Foundation, 
	an American nonprofit organization funded mainly by donations from readers.
	[2] Wikipedia is the largest and most-read reference work in history.[3][4] 
	
	# https://en.wikipedia.org/wiki/Wikipedia 
	// 23.04.25 17:58 (+03)
";

	let mut c_comments = 0;

	let mut c_asciispecsymb = 0;

	let mut c_asciipoints = 0;
	let mut c_asciicommas = 0;
	let mut c_asciispaces = 0;

	let mut c_utf8ru = 0;
	let mut c_uppercase_utf8ru = 0;
	let mut c_lowercase_utf8ru = 0;

	let mut c_utf8en = 0;
	let mut c_uppercase_utf8en = 0;
	let mut c_lowercase_utf8en = 0;

	let mut c_asciinumbers = 0;

	for_match!(@'data_chars_loop (s_indata.chars(), let mut a) -> |iter| {
		Some(' ') => c_asciispaces += 1,
		Some(',') => c_asciicommas += 1,
		Some('.') => c_asciipoints += 1,

		Some('\n') | Some('\t') => c_asciispecsymb += 1,

		Some('0' ..= '9') => c_asciinumbers += 1,
		Some('А' ..= 'Я') | Some('Ё') => {
			c_utf8ru += 1;
			c_uppercase_utf8ru += 1;
		},
		Some('а' ..= 'я') | Some('ё') => {
			c_utf8ru += 1;
			c_lowercase_utf8ru += 1;
		},


		Some('A' ..= 'Z') => {
			c_utf8en += 1;
			c_uppercase_utf8en += 1;
		},
		Some('a' ..= 'z') => {
			c_utf8en += 1;
			c_lowercase_utf8en += 1;
		},

		Some('#') => while_match!((iter, a, c_comments += 1) -> |_| {
			Some('\n') => continue 'data_chars_loop,
			Some(..) => {},
			_ => break 'data_chars_loop,
		}),
		Some('/') => match iter.next() {
			Some('*') => loop_match!(@'comment_loop (a, a = iter.next(), c_comments += 1) -> |_, _| {
				Some('*') => match iter.next() {
					Some('/') => continue 'data_chars_loop,
					Some(..) => {
						a = iter.next();
						continue 'comment_loop;
					},
					_ => return Err("The symbol '/' was expected. "),
				},
				Some(..) => {
					a = iter.next();
					continue 'comment_loop;
				},
				_ => return Err("The symbol '*' was expected. "),
			}),

			Some('/') => while_match!((iter, a, c_comments += 1) -> |_| {
				Some('\n') => continue 'data_chars_loop,
				Some(..) => {},
				_ => break 'data_chars_loop,
			}),

			_ => return Err("The symbol '*' was expected. "),
		},

		//Some(..) => return Err("Unknown symbol '{}'", a),
		Some(..) => {},
		_ => break,
	});

	println!(
		"---------
comment_count:  {}
special_symbol_count:  {}
period_count:  {}
comma_count:  {}
space_count:  {}
---------
number_count:  {}
---------
russian_char_count:    {}
uppercase_russian_count:  {}
lowercase_russian_count:  {}
---------
english_char_count:    {}
uppercase_english_count:  {}
lowercase_english_count:  {}
---------",
		c_comments,
		c_asciispecsymb,
		c_asciipoints,
		c_asciicommas,
		c_asciispaces,
		c_asciinumbers,
		c_utf8ru,
		c_uppercase_utf8ru,
		c_lowercase_utf8ru,
		c_utf8en,
		c_uppercase_utf8en,
		c_lowercase_utf8en,
	);

	Ok(())
}
