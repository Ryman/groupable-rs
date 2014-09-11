Implements grouping behaviour for `Iterators`.

# Example

```rust
use std::collections::HashMap;
use groupable::Groupable;

let evens = range(0u, 10).map(|i| (i % 2 == 0, i))
                         .group::<HashMap<bool, Vec<uint>>>();

assert_eq!(evens[true].as_slice(), [0, 2, 4, 6, 8].as_slice());
assert_eq!(evens[false].as_slice(), [1, 3, 5, 7, 9].as_slice());
```
