use crate::payload::Payload;
use std::rc::Rc;

//static OBSERVERS: RefCell<HashMap<Weak<Payload>, Weak<Payload>>> = RefCell::new(HashMap::new());

pub fn register() -> Rc<Payload> {
	let payload = Payload::generate();
	let rc = Rc::new(payload);

	let weak = Rc::downgrade(&rc);

	todo!()
}

#[cfg(test)]
mod test {
	#[test]
	fn foo() {
		todo!();
	}
}
