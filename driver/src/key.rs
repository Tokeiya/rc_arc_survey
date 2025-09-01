use std::any::Any;
use std::hash::Hash;

pub struct Key(Box<dyn Any>);

impl Key {
	pub fn new<T: Any + Hash>(t: T) -> Self {
		Self(Box::new(t))
	}
}

impl PartialEq for Key {
	fn eq(&self, other: &Self) -> bool {
		self.0.eq(&other.0)
	}
}
