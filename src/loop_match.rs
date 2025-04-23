/// [DOC IS ATTACHED!] The `For` loop, combined with matching.
#[macro_export]
macro_rules! loop_match {
	[ $(@$prefix:tt)?		() $($unk_tt:tt)* ] => {{
		compile_error! (
			"Initial macro arguments are required, please describe them in (...).

Record form: (A_Variable, ...) -> || ...
			
1. (A_Variable, Required): The name of the variable or executable expression to compare.
2. (..._variable, Optional): Description of internal variables.


Expected (1): (a) -> || ...
// loop { match a {...} }

Expected (2): (a.next(), ...) -> |...| ...
// let mut $(...) = $(...)
// loop { match a.next() {...} }

Expected (3): (a.next(), 1024, ...) -> |my_usize, ...| -> ...
// let mut my_usize = 1024;
// let mut $(...) = $(...)
// loop { match a.next() {...} }
"
		)
	}};


	[ $(@$prefix:tt)?		($($args:tt)*) -> || { $($data:tt)* } ] => {{
		$crate::build_loop_match! {
			(
				[$($prefix)?]
				[$($args)*]
				[]
			) {
				$($data)*
			}
		}
	}};

	[ $(@$prefix:tt)?		($($args:tt)*) -> |$($nn_i:tt),*| { $($data:tt)* } ] => {{
		$crate::build_loop_match! {
			(
				[$($prefix)?]
				[$($args)*]
				[$($nn_i),*]
			) {
				$($data)*
			}
		}
	}};

	[ $($tt:tt)* ] => {{
		compile_error! (
			concat!(
				"Undefined form of macro recording.

Provided by: \"", stringify!($($tt)*) ,"\"

Short record form: (a) -> || {}
Full record form: @'my_for (a.next(), ...) -> |...| {...}

Expected (1, Short version): 

```
	let data = b\"123456789\";
	
	let mut num = 0usize;
	
	let mut iter = data.iter();
	loop_match!((iter.next()) -> || {
		Some(b'0') => {},
		Some(a @ b'1' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!(\"Unk byte: {:?}\", a),
		_ => break
	});
	
	assert_eq!(num, 123456789);
```

Expected (2, Full version): 

```
	let data = \"1234567890\";
	
	let mut iter = data.as_bytes().into_iter();
	let data_n_index = loop_match!((iter.next(), 0usize) -> |data_n_index| {
		Some(a) => data_n_index += *a as usize,
		_ => break data_n_index,
	});
	
	assert_eq!(data_n_index, 525);
```

")

		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_loop_match {
	[
		([$($prefix:tt)?][ $a:expr $(, $nn_e:expr)* $(,)? ][$($nn_i:tt),*] $(,)?) {
			$($data:tt)*
		}
	] => {
		$crate::init_cycle_vars! {
			{}

			{ $([$nn_i]),* }
			{ $([$nn_e]),* }
		}

		$($prefix:)? loop {
			$crate::cycle_match!(@loop ($a): $($data)*);
		}
	};
	[
		([$($prefix:tt)?][ $a:ident $(, $nn_e:expr)* $(,)? ][$($nn_i:tt),*] $(,)?) {
			$($data:tt)*
		}
	] => {
		$crate::init_cycle_vars! {
			{ [{$a}] }

			{ $([$nn_i]),* }
			{ $([$nn_e]),* }
		}

		$($prefix:)? loop {
			$crate::cycle_match!(@loop ($a): $($data)*);
		}
	};
}
