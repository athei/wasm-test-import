mod seal {
	#[link(wasm_import_module = "seal0")]
	extern "C" {
		// Enable the no mangle attribute and this clashes with the "call" function from
		// dep which results in this function never imported nor called. The result is that
		// the "call" from dep is called two times.
		// #[no_mangle] 
		pub fn call() -> u32;
	}
}

#[no_mangle]
pub fn start() {
	unsafe {
		seal::call();
	}
	dep::call();
}