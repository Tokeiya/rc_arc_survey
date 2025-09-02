mod any_key;
mod key;
mod rec_hassher;

use crate::any_key::AnyKey;
use crate::key::Key;
use std::any::Any;
use std::collections::HashSet;
fn main() {
	let mut set = HashSet::<Key>::new();
	dbg!(set.insert(Key::from_key("hello")));
	dbg!(set.insert(Key::from_key("hello")));
	dbg!(set.insert(Key::from_key("world")));
	dbg!(set.insert(Key::from_key(42)));
}
