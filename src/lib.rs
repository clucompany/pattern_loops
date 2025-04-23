//Copyright (c) 2019-2025 #UlinProject Denis Kotlyarov (Денис Котляров)

//-----------------------------------------------------------------------------
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

// or

//-----------------------------------------------------------------------------
//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

// #Ulin Project 1819 2025

/*!
Macros for creating expressive loops with pattern matching.


# Full use

Purpose: To read lines from a file ignoring comments and special characters using macros (for_match, while_match).

```rust
#[macro_use]
extern crate cycle_match;

use std::io::Read;

fn main() -> Result<(), std:: io::Error> {
	let mut read_buffer = [0u8; 128];
	let mut buffer = Vec::with_capacity(130);
	let mut file = std::fs::File::open("./read.txt")?;

	while_match!((file.read(&mut read_buffer)) -> || {
		Ok(0) => break,
		Ok(len) => {
			let real_array = &read_buffer[..len];

			for_match!(@'read (real_array.into_iter()) -> |iter| {
				Some(13u8) => continue,
				Some(b'\n') => {
					if buffer.len() > 0 {
						println!("#line: {}", unsafe { std::str::from_utf8_unchecked(&buffer) });
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
	if buffer.len() > 0 {
		println!("#line: {}", unsafe { std::str::from_utf8_unchecked(&buffer) });
	}

	Ok(())
}
```

# Use 1 (while_match)

Purpose: Convert characters to a digital sequence using a macro while_math.

```rust
#[macro_use]
extern crate cycle_match;

fn main() {
	let data = b"123456789";

	let mut num = 0usize;

	let mut iter = data.iter();
	while_match!((iter) -> || {
		Some(b'0') => {},
		Some(a @ b'0' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!("Unk byte: {:?}", a),
		_ => break
	});

	assert_eq!(num, 123456789);
}
```


*/

#![no_std]
#![allow(clippy::tabs_in_doc_comments)]

mod while_match;
mod for_match;
mod loop_match;

#[doc(hidden)]
#[macro_export]
macro_rules! cycle_match {
	// ext version
	[@$m_type:tt ($($match_args:tt)+): $(#[$name:tt] {$($ext_data:tt)*}),+ $(,)? ] => {
		$crate::ext_cycle_match! {
			@$m_type ($($match_args)+):

			$(
				#[$name] {$($ext_data)*}
			)*
		}
	};

/*
	// default version, empty match
	[@$m_type:tt ($($match_args:tt)+): ] => {
		compile_error! (
			concat!("An internal description of the 'match' language construct was expected.

Provided by: \"\"

Expected \"
	Some(a) => ... ,
	None => ... ,
\"
"
			)
		)
	};
*/

/*
	// default version, empty version
	[@$m_type:tt ($($match_args:tt)+): ] => {

	};
*/

	// default version
	[@$m_type:tt ($($match_args:tt)+): $($data:tt)* ] => {
		match $($match_args)+ {
			$($data)*
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! ext_cycle_match {
	[@$m_type:tt (): ] => {}; //break

	[@$m_type:tt ($($match_args:tt)+):	#[begin] {$($data:tt)*} $($all_data:tt)*] => {{
		$crate::cycle_match! {
			@$m_type ($($match_args)+): $($data)*
		};

		$crate::ext_cycle_match! {
			@$m_type ():

			$($all_data)*
		}
	}};

	[@$m_type:tt ($($match_args:tt)+):	#[insert] {$($data:tt)*} $($all_data:tt)*] => {{
		{$($data)*};

		$crate::ext_cycle_match! {
			@$m_type ($($match_args)+):

			$($all_data)*
		}
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! init_cycle_vars {
	[
		{$($_unk:tt)*} {} {}
	] => {};

	[
		{ $($all:tt)* }
		{ $([$($i_next:tt)+])? $(, [$($i:tt)+])* }
		{ $([$($e_next:tt)+])? $(, [$($e:tt)+])* }
	] => {

		$crate::__init_cycle_vars_init! {
			{ $($all)* }  [$($($i_next)*)?] {$([$($i)*]),*} : [$($($e_next)+)?] {$([$($e)+]),*}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __init_cycle_vars_init {
	[
		{$($all_tt:tt)*} [] {} : [] {}
	] => {}; //break, empty variables

	//error  let _ = $e;
	[
		{$($all_tt:tt)*} [] {$($unk_i:tt)*} : [ $($e:tt)+ ]
	] => {
		//#0
		compile_error!(
			concat!(
				"For the expression \"", stringify!($($e)+), "\", a name was expected in | ... |, but this was not done. (You can specify either a name or _ to ignore the name)
"
			)
		);
	};

	//error  let e = _;
	[
		{$($all_tt:tt)*} [ $($i2:tt)+ ] {$($unk_i:tt)*} : []
	] => {
		compile_error!(
			concat!(
				"For the name \"", stringify!($($i2)+) ,"\", an expression in (...) was expected, but this was not done. (You can specify an expression, or remove the extra name in | ... |)
"
			)
		);
	};

	[
		{$($all_tt:tt)*}

		[ _ ] {$([$($next_i:tt)*])? $(, [$($unk_i:tt)*])*} : [ $($e:tt)+ ] { $([$($next_e:tt)*])? $(, [$($unk_e:tt)*])* }
	] => {
		{ $($e)+ };


		$crate::__init_cycle_vars_init! {
			{$($all_tt)*}

			[$($($next_i)*)?] { $([$($unk_i)*]),* } : [$($($next_e)*)?] { $([$($unk_e)*]),* }
		}
	};


	//fn next
	[
		{ $( [$($check:tt)*] )? }

		[ $i:ident ] {$([$($next_i:tt)*])? $(, [$($unk_i:tt)*])*} : [ $($e:tt)+ ] { $([$($next_e:tt)*])? $(, [$($unk_e:tt)*])* }
	] => {
		{
			$(
				$crate::__init_cycle_varss_name_check! {
					[$($check)*]:  $i
				}
			)?
		}

		let mut $i = $($e)+;

		$crate::__init_cycle_vars_init! {
			{ $( [$($check)*] )? }

			[$($($next_i)*)?] { $([$($unk_i)*]),* } : [$($($next_e)*)?] { $([$($unk_e)*]),* }
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __init_cycle_varss_name_check {
	[
		[]: $($__ignore:tt)*
	] => {}; //break

	[
		[ {$self_name:ident}  $(, {$next_expr:ident} )* ]:  $i:ident
	] => {{
		macro_rules! __check_ident_equality {
			[
				if_ident(#{$i} && #{$i}) $ok:block else $err:block
			] => {$ok};
			[
				if_ident(#{$i} && #{$i}) $ok:block
			] => {$ok};

			[
				if_ident(#{$self_name} && #{$self_name}) $ok:block else $err:block
			] => {$ok};
			[
				if_ident(#{$self_name} && #{$self_name}) $ok:block
			] => {$ok};


			[
				if_ident(#{$self_name} && #{$uunk:tt}) $ok:block
			] => {};
			[
				if_ident(#{$self_name} && #{$uunk:tt}) $ok:block else $err:block
			] => {$err};
		}

		__check_ident_equality!(
			if_ident(#{$self_name} && #{$i}) {
				compile_error!(
					concat!(
						"Name conflict, possibly undefined behavior. Call the variable \"",
							stringify!($i) ,"\" something different."
					)
				);
			}else {

			}
		);

		$crate::__init_cycle_varss_name_check! {
			[ $({$next_expr}),* ]: $i
		}
	}};
}
