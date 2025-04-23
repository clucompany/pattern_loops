use pattern_loops::{for_match, loop_match, while_match};

fn main() {
	let data = "
	
	/*
	tabs:
		1 History

		1.1 Nupedia
		1.2 Launch and early growth
		1.3 Milestones
	*/
	
	Википедия:
	
	Wikipedia (ˌwɪkɪˈpiːdiə (About this soundlisten) wik-ih-PEE-dee-ə or ˌwɪkiˈpiːdiə
	(About this soundlisten) wik-ee-PEE-dee-ə) is a multilingual online encyclopedia 
	created and maintained as an open collaboration project[3] by a community of 
	volunteer editors using a wiki-based editing system.[4] It is the largest 
	and most popular general reference work on the World Wide Web,[5][6][7] and 
	is one of the most popular websites ranked by Alexa as of October 2019.[8] It 
	features exclusively free content and no commercial ads, and is owned and supported 
	by the Wikimedia Foundation, a non-profit organization funded primarily through 
	donations.[9][10][11][12] 
	
	# https://en.wikipedia.org/wiki/Wikipedia 
	// 17.11.19 15:06 (+03)
";

	let mut q_comments = 0;

	let mut q_special_utf8 = 0;

	let mut q_points = 0;
	let mut q_commas = 0;
	let mut q_spaces = 0;

	let mut q_russian_utf8 = 0;
	let mut q_russian_big_utf8 = 0;
	let mut q_russian_small_utf8 = 0;

	let mut q_engl_utf8 = 0;
	let mut q_engl_big_utf8 = 0;
	let mut q_engl_small_utf8 = 0;

	let mut q_numbers = 0;

	for_match!(@'main_loop (data.chars(), let mut a) -> |iter| {
		Some(' ') => q_spaces += 1,
		Some(',') => q_commas += 1,
		Some('.') => q_points += 1,

		Some('\n') | Some('\t') => q_special_utf8 += 1,

		Some('0' ..= '9') => q_numbers += 1,
		Some('А' ..= 'Я') | Some('Ё') => {
			q_russian_utf8 += 1;
			q_russian_big_utf8 += 1;
		},
		Some('а' ..= 'я') | Some('ё') => {
			q_russian_utf8 += 1;
			q_russian_small_utf8 += 1;
		},


		Some('A' ..= 'Z') => {
			q_engl_utf8 += 1;
			q_engl_big_utf8 += 1;
		},
		Some('a' ..= 'z') => {
			q_engl_utf8 += 1;
			q_engl_small_utf8 += 1;
		},

		Some('#') => while_match!((iter, a, q_comments += 1) -> |_| {
			Some('\n') => continue 'main_loop,
			Some(_a) => {},
			_ => break 'main_loop,
		}),
		Some('/') => match iter.next() {
			//a = iter.next, At start!!
			Some('*') => loop_match!(@'comment_loop (a, a = iter.next(), q_comments += 1) -> |_, _| {
				Some('*') => match iter.next() {
					Some('/') => continue 'main_loop,
					Some(_a) => {
						a = iter.next();
						continue 'comment_loop;
					},
					_ => panic!("The symbol '/' was expected. "),
				},
				Some(..) => {
					a = iter.next();
					continue 'comment_loop;
				},
				_ => panic!("The symbol '*' was expected. "),
			}),

			Some('/') => while_match!((iter, a, q_comments += 1) -> |_| {
				Some('\n') => continue 'main_loop,
				Some(_a) => {},
				_ => break 'main_loop,
			}),

			_ => panic!("The symbol '*' was expected. "),
		},

		//Some(..) => panic!("Unknown symbol '{}'", a),
		Some(..) => {},
		_ => break,
	});

	println!(
		"---------
q_comments:	{}
q_special:	{}
q_points:	{}
q_commas:	{}
q_spaces:	{}
---------
q_numbers:	{}
---------
q_russian:		{}
q_russian_big:	{}
q_russian_small:	{}
---------
q_engl:		{}
q_engl_big:		{}
q_engl_small:	{}
---------",
		q_comments,
		q_special_utf8,
		q_points,
		q_commas,
		q_spaces,
		q_numbers,
		q_russian_utf8,
		q_russian_big_utf8,
		q_russian_small_utf8,
		q_engl_utf8,
		q_engl_big_utf8,
		q_engl_small_utf8,
	);
}
