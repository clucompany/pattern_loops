/// [DOC IS ATTACHED!] The `while` loop, combined with matching.
#[macro_export]
macro_rules! while_match {

	[ $(@$prefix:tt)?		() $($unk_tt:tt)* ] => {{
		compile_error! (
			"Initial macro arguments are required, please describe them in (...).

Record form: (Iterator, A_Variable, ...) -> |...| ...
			
1. (Iterator, Required): The name of the iterator we are working with.
2. (A_Variable, Optional): The name of the rewritable variable (you can omit and write _ or declare a new variable with the desired name using `let mut MyVar`)
3. (..._variable, Optional): Description of internal variables.


Expected (1): (slice) -> ...
// let mut __hidden_a;
// loop { __hidden_a = slice.next(); ... }

Expected (2): (slice.my_next()) -> ...
// let mut __hidden_a;
// loop { __hidden_a = slice.my_next(); ... }

Expected (3): (slice, _, ...) -> ...
// let mut __hidden_a;
// let mut $(...) = $(...)
// loop { __hidden_a = slice.next() }

Expected (4): (iter, a, ...) -> ...
// let mut a;
// let mut $(...) = $(...)
// loop { a = iter.next() ... }

Expected (4): (iter, let mut a, ...) -> ...
// let mut a;
// let mut $(...) = $(...)
// loop { a = iter.next() }

Expected (5): (iter, _ 1024, ...) -> |my_usize, ...| ...
// let mut __hidden_a;
// let mut my_usize = 1024;
// let mut $(...) = $(...)
// loop { __hidden_a = iter.next() }
"
		)
	}};

	[ $(@$prefix:tt)?		($($args:tt)*) -> || { $($data:tt)* } ] => {{
		$crate::build_while_match! {
			([$($prefix)?]):

			[$($args)*]
			[]

			{
				$($data)*
			}
		}
	}};

	[ $(@$prefix:tt)?		($($args:tt)*) -> |$($name_tt:tt),*| { $($data:tt)* } ] => {{
		$crate::build_while_match! {
			([$($prefix)?]):

			[$($args)*]
			[$($name_tt),*]

			{
				$($data)*
			}
		}
	}};
	//

	[ $($tt:tt)* ] => {{
		compile_error! (
			concat! (
"Undefined form of macro recording.

Provided by: \"", stringify!($($tt)*) ,"\"


Full record form: @'my_while (iter, a, ...) -> |...| {...}
Short record form: (iter) -> || {}

Expected (1, Short version): 

```
let data = b\"123456789\";
let mut num = 0usize;

let mut iter = data.iter();
while_match!((iter) -> || {
	Some(b'0') => {},
	Some(a @ b'1' ..= b'9') => {
		num *= 10;
		num += (a - b'0') as usize;
	},
	Some(a) => panic!(\"Unk byte: {:?}\", a),
	_ => break
});
```

Expected (2, Full version): 

```
let mut iter = 0..=15;
let while_match = while_match!((iter.next(), let mut a, 255) -> |data| {
	Some(15) => break data,
	Some(_num) => data += 1,
	_ => break data,
});
assert_eq!(while_match, 270);
```

"
			)
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_while_match {
	//$iter args -> full args
	[($($all_tt:tt)*): [$iter:ident] [$($names:tt)*] {$($data:tt)*} ] => {
		$crate::build_while_match! {
			($($all_tt)*): [$iter, _] [$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr] [$($names:tt)*] {$($data:tt)*} ] => {
		$crate::build_while_match! {
			($($all_tt)*): [$iter, _] [$($names)*] {
				$($data)*
			}
		}
	};

	//decode args
	[($($all_tt:tt)*): [$iter:ident, let mut $a:ident $($args:tt)*] [$($names:tt)*] {$($data:tt)*} ] => {
		let mut $a;
		$crate::build_while_match! {
			($($all_tt)*): [$iter, $a $($args)*] [$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr, let mut $a:ident $($args:tt)*] [$($names:tt)*] {$($data:tt)*} ] => {
		let mut $a;
		$crate::build_while_match! {
			($($all_tt)*): [$iter, $a $($args)*] [$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:ident, _ $($args:tt)*] [$($names:tt)*]  {$($data:tt)*} ] => {
		let mut __a_hidden;
		$crate::build_while_match! {
			($($all_tt)*): [$iter, __a_hidden $($args)*] [$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr, _ $($args:tt)*] [$($names:tt)*] {$($data:tt)*} ] => {
		let mut __a_hidden;
		$crate::build_while_match! {
			($($all_tt)*): [$iter, __a_hidden $($args)*] [$($names)*] {
				$($data)*
			}
		}
	};

	[
		([$($prefix:tt)?]):
			[$iter:ident, $a:ident $(,$nn_e:expr)*	$(,)?]
			[$($nn_i:tt),*					$(,)?]

			{
				$($data:tt)*
			}
	] => {
		$crate::init_cycle_vars! {
			{	[{$iter}, {$a}]	}

			{ $([$nn_i]),* }
			{ $([$nn_e]),* }
		}

		$($prefix:)? loop {
			$a = core::iter::Iterator::next(&mut $iter);
			$crate::cycle_match!(@while ($a): $($data)*);
		}
	};

	[
		([$($prefix:tt)?]):
			[$iter:expr, $a:ident $(, $nn_e:expr)*	$(,)?]
			[$( $nn_i:tt ),*					$(,)?]
			{
				$($data:tt)*
			}
	] => {
		$crate::init_cycle_vars! {
			{	[{$a}]	}

			{ $([$nn_i]),* }
			{ $([$nn_e]),* }
		}

		$($prefix:)? loop {
			$a = $iter;
			$crate::cycle_match!(@while ($a): $($data)*);
		}
	};
}
