pub enum ObserverResult<T> {
	Some(T),
	Dropped,
	NotFound,
}

impl<T> From<T> for ObserverResult<T> {
	fn from(value: T) -> Self {
		Self::Some(value)
	}
}
