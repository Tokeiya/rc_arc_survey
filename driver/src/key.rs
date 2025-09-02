use crate::any_key::AnyKey;
use crate::envelope::Envelope;
use std::any::Any;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub struct Key(Box<dyn AnyKey>);

impl Key {
	pub fn from_key<T: AnyKey + 'static>(key: T) -> Self {
		Self(Box::new(key) as Box<dyn AnyKey>)
	}
}

impl PartialEq for Key {
	fn eq(&self, other: &Self) -> bool {
		self.0.dyn_eq(other.0.as_ref())
	}
}

impl Eq for Key {}

impl Hash for Key {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let mut envelope = Envelope::from(state);

		self.0.type_id().hash(&mut envelope);
		self.0.dyn_hash(&mut envelope);
	}
}

pub fn any_key_sample() {
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
