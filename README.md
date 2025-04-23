<div id="header" align="center">

  <b>[pattern_loops]</b>
  
  (Macros for creating expressive loops with pattern matching.)
  </br></br>

<div id="badges">
  <a href="./LICENSE_MIT">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/mit.png?raw=true" alt="mit"/>
  </a>
  <a href="./LICENSE_APACHE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/pattern_loops">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/pattern_loops">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>
	
  [![CI](https://github.com/clucompany/pattern_loops/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/pattern_loops/actions/workflows/CI.yml) 


</div>
</div>

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
pattern_loops = "0.2.0"
```

and this to your source code:

```rust
use pattern_loops::for_match;
use pattern_loops::while_match;
use pattern_loops::loop_match;
```
## Example

### while_match

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
		Some(a @ b'1' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!("Unk byte: {:?}", a),
		_ => break
	});
	
	assert_eq!(num, 123456789);
}
```

<a href="./examples">
  See all
</a>

## License

This project has a dual license according to (LICENSE-MIT) and (LICENSE-APACHE-2-0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2019-2025 #UlinProject</b>
	
  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License

<div align="left">
  <a href="./LICENSE_APACHE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>
    
  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>

### MIT License

<div align="left">
  <a href="./LICENSE_MIT">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/mit.png?raw=true" alt="mit"/>
  </a>
  <b>&nbsp;Licensed under the MIT License.</b>
  </br></br></br></br>
</div>
