extern crate groupable;

use std::collections::{HashMap, TreeMap, SmallIntMap, TrieMap};
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

#[test]
fn treemap() {
    let map = XS.iter().enumerate()
                       .map(|(i, &c)| (i % 2, c))
                       .group::<TreeMap<uint, String>>();

    assert_eq!(map[0].as_slice(), "hi!");
    assert_eq!(map[1].as_slice(), "bye");
}

#[test]
fn smallintmap() {
    let map = XS.iter().enumerate()
                       .map(|(i, &c)| (i % 2, c))
                       .group::<SmallIntMap<String>>();

    assert_eq!(map[0].as_slice(), "hi!");
    assert_eq!(map[1].as_slice(), "bye");
}

#[test]
fn triemap() {
    let map = XS.iter().enumerate()
                       .map(|(i, &c)| (i % 2, c))
                       .group::<TrieMap<String>>();

    assert_eq!(map[0].as_slice(), "hi!");
    assert_eq!(map[1].as_slice(), "bye");
}
