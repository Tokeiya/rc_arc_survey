use std::any::Any;

pub mod any_key;
pub mod envelope;
mod key;

use observer::rc_observer::register;
fn main() {
	let rc = register();
}
