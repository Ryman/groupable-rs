#![feature(collections)]
extern crate groupable;

use std::collections::{HashMap, VecMap, BTreeMap};
use groupable::Groupable;

static XS : &'static [char] = &['h', 'b', 'i', 'y', '!', 'e'];

macro_rules! test_string (
    ($t:ty) => (
        {
            let map = XS.iter().enumerate()
                           .map(|(i, &c)| (i % 2, c))
                           .group::<$t>();

            assert_eq!(map[&0], "hi!");
            assert_eq!(map[&1], "bye");
        }
    )
);

macro_rules! test_char_vec (
    ($t:ty) => (
        {
            let map = XS.iter().enumerate()
                           .map(|(i, &c)| (i % 2, c))
                           .group::<$t>();

            assert_eq!(map[&0], ['h', 'i', '!']);
            assert_eq!(map[&1], ['b', 'y', 'e']);
        }
    )
);

#[test]
fn hashmap() {
    test_string!(HashMap<usize, String>);
    test_char_vec!(HashMap<usize, Vec<char>>);
}

#[test]
fn smallintmap() {
    test_string!(VecMap<String>);
    test_char_vec!(VecMap<Vec<char>>);
}

#[test]
fn btreemap() {
    test_string!(BTreeMap<usize, String>);
    test_char_vec!(BTreeMap<usize, Vec<char>>);
}
