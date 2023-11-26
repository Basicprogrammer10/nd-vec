# nd-vec

A compile time n-dimensional vector library.

```rust
use nd_vec::{Vector, vector};

let a = vector!(1, 2, 3);
let b = vector!(4, 5, 6);
println!("{}", a + b); // => (5, 7, 9)
```
