# nd-vec [![Build](https://github.com/Basicprogrammer10/nd-vec/actions/workflows/rust.yml/badge.svg)](https://github.com/Basicprogrammer10/nd-vec/actions/workflows/rust.yml) [![Crates.io](https://img.shields.io/crates/v/nd-vec)](https://crates.io/crates/nd-vec)

A compile time n-dimensional vector library.

```rust
use nd_vec::vector;

let a = vector!(1, 2, 3);
let b = vector!(4, 5, 6);
println!("{:?}", a + b); // => (5, 7, 9)
```
