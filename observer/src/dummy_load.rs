use std::sync::atomic::AtomicUsize;

#[derive(Debug)]
pub struct DummyLoad(usize);

static ID_SEED: AtomicUsize = AtomicUsize::new(0);

pub fn generate_dummy() -> DummyLoad {
	DummyLoad(ID_SEED.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}

impl DummyLoad {
	pub fn id(&self) -> usize {
		self.0
	}
}
