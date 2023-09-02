#![no_std]
#![feature(asm_experimental_arch)]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn __foo() {
    // your code here
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}