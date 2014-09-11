#![feature(macro_rules)]
extern crate groupable;

use std::collections::{HashMap, TreeMap, SmallIntMap, TrieMap};
use groupable::Groupable;

static XS : &'static [char] = ['h', 'b', 'i', 'y', '!', 'e'];

macro_rules! test_string (
    ($t:ty) => (
        {
            let map = XS.iter().enumerate()
                           .map(|(i, &c)| (i % 2, c))
                           .group::<$t>();

            assert_eq!(map[0].as_slice(), "hi!");
            assert_eq!(map[1].as_slice(), "bye");
        }
    )
)

macro_rules! test_char_vec (
    ($t:ty) => (
        {
            let map = XS.iter().enumerate()
                           .map(|(i, &c)| (i % 2, c))
                           .group::<$t>();

            assert_eq!(map[0].as_slice(), ['h', 'i', '!'].as_slice());
            assert_eq!(map[1].as_slice(), ['b', 'y', 'e'].as_slice());
        }
    )
)

#[test]
fn hashmap() {
    test_string!(HashMap<uint, String>)
    test_char_vec!(HashMap<uint, Vec<char>>)
}

#[test]
fn treemap() {
    test_string!(TreeMap<uint, String>)
    test_char_vec!(TreeMap<uint, Vec<char>>)
}

#[test]
fn smallintmap() {
    test_string!(SmallIntMap<String>)
    test_char_vec!(SmallIntMap<Vec<char>>)
}

#[test]
fn triemap() {
    test_string!(TrieMap<String>)
    test_char_vec!(TrieMap<Vec<char>>)
}
