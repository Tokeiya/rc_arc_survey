mod any_key;
mod key;
mod rec_hasher;

use crate::any_key::AnyKey;
use crate::key::Key;
use std::any::Any;
use std::collections::HashSet;
use std::hash::Hasher;

fn main() {
	let mut set = HashSet::<Key>::new();
	dbg!(set.insert(Key::from_key("hello")));
	dbg!(set.insert(Key::from_key("hello".to_string())));
	dbg!(set.insert(Key::from_key("world")));
	dbg!(set.insert(Key::from_key(42)));
	dbg!(set.insert(Key::from_key(42u8)));

	println!("repeat again");

	dbg!(set.insert(Key::from_key("hello")));
	dbg!(set.insert(Key::from_key("hello".to_string())));
	dbg!(set.insert(Key::from_key("world")));
	dbg!(set.insert(Key::from_key(42)));
	dbg!(set.insert(Key::from_key(42u8)));
}
