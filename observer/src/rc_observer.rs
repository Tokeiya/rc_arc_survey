use crate::payload::Payload;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;
const OBSERVERS: ::std::thread::LocalKey<RefCell<HashMap<Weak<Payload>, Weak<Payload>>>> = {
	#[inline]
	fn __init() -> RefCell<HashMap<Weak<Payload>, Weak<Payload>>> {
		RefCell::new(HashMap::new())
	}
	unsafe {
		::std::thread::LocalKey::new(
			const {
				if ::std::mem::needs_drop::<RefCell<HashMap<Weak<Payload>, Weak<Payload>>>>() {
					|init| {
						#[thread_local]
						static VAL: ::std::thread::local_impl::LazyStorage<
							RefCell<HashMap<Weak<Payload>, Weak<Payload>>>,
							(),
						> = ::std::thread::local_impl::LazyStorage::new();
						VAL.get_or_init(init, __init)
					}
				} else {
					|init| {
						#[thread_local]
						static VAL: ::std::thread::local_impl::LazyStorage<
							RefCell<HashMap<Weak<Payload>, Weak<Payload>>>,
							!,
						> = ::std::thread::local_impl::LazyStorage::new();
						VAL.get_or_init(init, __init)
					}
				}
			},
		)
	}
};
