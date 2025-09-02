use crate::key::Key;
use std::any::Any;
use std::collections::HashSet;

pub mod any_key;
pub mod envelope;
mod key;

fn main() {
	let mut set: HashSet<Key> = HashSet::new();

	let a = 42;
	let b = &a as &dyn Any;

	dbg!(set.insert(Key::from_key("hello")));
	dbg!(set.insert(Key::from_key("hello".to_string())));
	dbg!(set.insert(Key::from_key(42i8)));
	dbg!(set.insert(Key::from_key(42i32)));

	println!("repeat again");

	dbg!(set.insert(Key::from_key("hello")));
	dbg!(set.insert(Key::from_key("hello".to_string())));
	dbg!(set.insert(Key::from_key(42i8)));
	dbg!(set.insert(Key::from_key(42i32)));
}
