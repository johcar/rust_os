#![no_std]
#![no_main]
#![feature(panic_implementation)]

use core::panic::PanicInfo;


// This function is called on panic 
// (panic_implementation deprecated and
// changed to panic_handler)
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// It's important that we disable the name mangling through
// the no_mangle attribute, otherwise the compiler would generate
// some cryptic _ZN3blog_os4_start7hb173fedf945531caE
#[no_mangle]  
pub extern "C" fn _start() -> ! {
    loop {}
}
