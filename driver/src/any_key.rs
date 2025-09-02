use crate::envelope::Envelope;
use std::any::{Any, TypeId};
use std::hash::Hash;

pub trait AnyKey {
	fn as_any(&self) -> &dyn Any;
	fn dyn_eq(&self, other: &dyn AnyKey) -> bool;
	fn dyn_hash(&self, hasher: &mut Envelope);

	fn type_id(&self) -> TypeId;
}

impl<T> AnyKey for T
where
	T: Any + Eq + Hash,
{
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn dyn_eq(&self, other: &dyn AnyKey) -> bool {
		if let Some(o) = other.as_any().downcast_ref::<T>() {
			self == o
		} else {
			false
		}
	}

	fn dyn_hash(&self, hasher: &mut Envelope) {
		self.hash(hasher);
	}

	fn type_id(&self) -> TypeId {
		self.as_any().type_id()
	}
}
