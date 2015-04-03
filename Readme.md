Easily aggregate groups of values from key-value Iterators.

# Example

```rust
use std::collections::HashMap;
use groupable::Groupable;

let evens = (0..10).map(|i| (i % 2 == 0, i))
                   .group::<HashMap<bool, Vec<usize>>>();

assert_eq!(evens[&true], [0, 2, 4, 6, 8]);
assert_eq!(evens[&false], [1, 3, 5, 7, 9]);
```
