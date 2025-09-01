use observer::payload::Payload;

mod dyn_hasher;
mod dyn_key;
mod key;

fn main() {}

fn foo(b: &Box<Payload>) {
	let a = b.get();
	println!("{}", a);
}
