/// [DOC IS ATTACHED!] The `For` loop, combined with matching.
/// # Full use
/// ```rust
///
///#[macro_use]
///extern crate cycle_match;
///fn main() {
///	let data = "12345678901";
///
///	let mut a;
///	let result = for_match!(@'begin (data.as_bytes().into_iter(), a, 0usize) -> |iter, num| {
///		Some(b'0') => {},
///		Some(b'1') => num += 1,
///		Some(b'2') => num += 2,
///		Some(b'3') => num += 3,
///		Some(b'4') => num += 4,
///		Some(b'5') => num += 5,
///		Some(b'6') => num += 6,
///		Some(b'7') => num += 7,
///		Some(b'8') => num += 8,
///		Some(b'9') => break 'begin num,
///		Some(a) => panic!("Unk byte '{:?}'", a),
///		_ => break 'begin num,
///	});
///
///	assert_eq!(a, Some(&b'9'));
///	assert_eq!(result, 36);
///}
///```
#[macro_export]
macro_rules! for_match {

	[ $(@$prefix:tt)?		() $($unk_tt:tt)* ] => {{
		compile_error! (
			"Initial macro arguments are required, please describe them in (...).

Record form: (Iterator, A_Variable, ...) -> |Iterator, ...| ...

1. (Iterator, Required): The name of the iterator we are working with.
2. (A_Variable, Optional): The name of the rewritable variable (you can omit and write _ or declare a new variable with the desired name using `let mut MyVar`)
3. (..._variable, Optional): Description of internal variables.


Expected (1): (slice) -> || ...
// let mut __hidden_iter = slice.iter();
// let mut __hidden_a;
// loop { __hidden_a = __hidden_iter.next(); ... }

Expected (2): (slice) -> |iter| ...  
// let mut iter = slice.iter();
// let mut __hidden_a;
// loop { __hidden_a = iter.next(); ... }

Expected (3): (slice.into_iter(), let mut a) -> || ...
// let mut __hidden_iter = slice.into_iter();
// let mut a;
// loop { a = iter.next(); ... }

Expected (4): (slice.into_iter(), let mut a, 1024usize, ...) -> |iter, my_usize, ...| ...
// let mut iter = slice.into_iter();
// let mut my_usize = 1024usize;
// let mut a;
// let ... = ...
// loop { a = iter.next(); ... }

Expected (5): (slice.into_iter(), a, 1024usize, ...) -> |iter, my_usize, ...| ...
// let mut iter = slice.into_iter();
// let mut my_usize = 1024usize;
// let ... = ...
// loop { a = iter.next(); ... }

Expected (6): (slice.into_iter(), _, ...) -> |_, ...| ...
// let mut __hidden_iter = slice.into_iter();
// let mut __hidden_a;  // _ -> __hidden
// let ... = ...
// loop { __hidden_a = __hidden_iter.next(); ... }

Expected (7): (a.into_iter(), let mut a, 1024, ...) -> |iter, my_usize, ...| -> ...
// let mut iter = a.into_iter();
// let mut a;
// let mut my_usize = 1024;
// let ... = ...
// loop { a = iter.next(); ... }

"
		)
	}};


	//new let iter, _
	[ $(@$prefix:tt)?	 ($($args:tt)*) -> || { $($data:tt)* } ] => {{
		$crate::build_for_match! {
			(
				[$($prefix)?]
			):

			[$($args)*][]

			{
				$($data)*
			}
		}
	}};

	[ $(@$prefix:tt)?	 ($($args:tt)*) -> |$($nn_i:tt),*| { $($data:tt)* } ] => {{
		$crate::build_for_match! {
			(
				[$($prefix)?]
			):

			[$($args)*][$($nn_i),*]

			{
				$($data)*
			}
		}
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_for_match {
	//args
	[($($all_tt:tt)*): [$iter:ident] [$($names:tt)*] {$($data:tt)*}] => {
		$crate::build_for_match! {
			($($all_tt)*):

			[$iter, let mut __a_hidden][$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr] [$($names:tt)*]	{$($data:tt)*}] => {
		$crate::build_for_match! {
			($($all_tt)*):

			[$iter, let mut __a_hidden][$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:ident, let mut $a:ident $($args:tt)*] [$($names:tt)*] {$($data:tt)*}] => {
		let mut $a;
		$crate::build_for_match! {
			($($all_tt)*):

			[$iter, $a $($args)*][$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr, let mut $a:ident $($args:tt)*] [$($names:tt)*] {$($data:tt)*}] => {
		let mut $a;
		$crate::build_for_match! {
			($($all_tt)*):

			[$iter, $a $($args)*][$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:ident, _ $($args:tt)*] [$($names:tt)*]	{$($data:tt)*}] => {
		$crate::build_for_match! {
			($($all_tt)*):

			[$iter, let mut __a_hidden $($args)*][$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr, _ $($args:tt)*] [$($names:tt)*]	{$($data:tt)*}] => {
		$crate::build_for_match! {
			($($all_tt)*):

			[$iter, let mut __a_hidden $($args)*][$($names)*] {
				$($data)*
			}
		}
	};


	//names, empty args
	[($($all_tt:tt)*): [$($args:tt)*] [] {$($data:tt)*}] => {
		$crate::build_for_match! {
			($($all_tt)*):

			[$($args)*][__iter_hidden]{
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$($args:tt)*] [_ $($names:tt)*] {$($data:tt)*}] => {
		$crate::build_for_match! {
			($($all_tt)*):

			[$($args)*][__iter_hidden $($names)*] {
				$($data)*
			}
		}
	};


	[//iter ident
		( [$($prefix:tt)?] ):
			[$iter:ident, $a:ident	$(, $nn_e:expr)* $(,)?] //args
			[$iter_name: ident 	$(, $nn_i:tt)* $(,)?] //names
			{
				$($data:tt)*
			}
	] => {
		let mut $iter_name = $iter.iter();

		$crate::init_cycle_vars! {
			{ [ {$iter_name}, {$iter}, {$a}] }

			{ $([$nn_i]),* }
			{ $([$nn_e]),* }
		}

		$($prefix:)? loop {
			$a = core::iter::Iterator::next(&mut $iter_name);
			$crate::cycle_match!(@for ($a): $($data)*);
		}
	};
	[//iter ident
		( [$($prefix:tt)?] ):
			[$iter:expr, $a:ident	$(, $nn_e:expr)* $(,)?] //args
			[$iter_name: ident 	$(, $nn_i:tt)* $(,)?] //names

			{
				$($data:tt)*
			}
	] => {
		let mut $iter_name = $iter;

		$crate::init_cycle_vars! {
			{ [ {$iter_name}, {$a} ] }

			{ $([$nn_i]),* }
			{ $([$nn_e]),* }
		}

		$($prefix:)? loop {
			$a = core::iter::Iterator::next(&mut $iter_name);
			$crate::cycle_match!(@for ($a): $($data)*);
		}
	};
}
