use crate::any_key::AnyKey;
use crate::envelope::Envelope;
use std::hash::{Hash, Hasher};

pub struct Key(Box<dyn AnyKey>);

impl Key {
	pub fn from_key<T: AnyKey + 'static>(key: T) -> Self
	where
		T: AnyKey,
	{
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
		let mut envelope = Envelope::from(state as &mut dyn Hasher);

		self.0.type_id().hash(&mut envelope);
		self.0.dyn_hash(&mut envelope);
	}
}
