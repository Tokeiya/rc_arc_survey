use crate::any_key::AnyKey;
use std::hash::Hash;

pub struct Key(Box<dyn AnyKey>);

impl Key {
	pub fn from_key<T>(key: T) -> Key
	where
		T: AnyKey + 'static,
	{
		Key::from(Box::new(key) as Box<dyn AnyKey>)
	}
}

impl From<Box<dyn AnyKey>> for Key {
	fn from(value: Box<dyn AnyKey>) -> Self {
		Self(value)
	}
}

impl PartialEq for Key {
	fn eq(&self, other: &Self) -> bool {
		let o = other.0.as_ref();
		self.0.equal(o)
	}
}

impl Eq for Key {}

impl Hash for Key {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let arr = self.0.prepare_hash();
		state.write(&arr);
	}
}
