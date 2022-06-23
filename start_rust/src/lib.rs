#![no_std]

#[no_mangle]
pub extern "C" fn run_rust_main() {
    rust_lib::main();
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        rust_lib::panic();
    }
}
