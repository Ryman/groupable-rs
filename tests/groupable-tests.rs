extern crate groupable;

use std::collections::HashMap;
use groupable::Groupable;

static XS : &'static [char] = ['h', 'b', 'i', 'y', '!', 'e'];

#[test]
fn hashmap() {
    let map = XS.iter().enumerate()
                       .map(|(i, &c)| (i % 2, c))
                       .group::<HashMap<uint, String>>();

    assert_eq!(map[0].as_slice(), "hi!");
    assert_eq!(map[1].as_slice(), "bye");
}
