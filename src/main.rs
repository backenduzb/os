#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(_magic: u32, _multiboot_info_addr: usize) -> ! {

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}