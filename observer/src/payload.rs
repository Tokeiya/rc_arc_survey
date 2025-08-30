use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::LazyLock;

static ID_SEED: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));

#[derive(Debug)]
pub struct Payload(u64);

impl Payload {
	pub(crate) fn generate() -> Self {
		let id = ID_SEED.fetch_add(1, Ordering::Relaxed);
		Self(id)
	}
	pub fn get(&self) -> u64 {
		self.0
	}
}
