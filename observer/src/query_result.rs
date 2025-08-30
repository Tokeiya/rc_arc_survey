pub enum QueryResult<T> {
	Some(T),
	KeyNotFound,
	Dropped,
	Locked,
}
