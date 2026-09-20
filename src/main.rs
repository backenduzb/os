#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod console;
mod font;
const MULTIBOOT2_BOOTLOADER_MAGIC: u32 = 0x36d76289;
const TAG_TYPE_END: u32 = 0;
const TAG_TYPE_FRAMEBUFFER: u32 = 8;

#[repr(C)]
struct MultibootInfo {
    total_size: u32,
    reserved: u32,
}

#[repr(C)]
struct Tag {
    tag_type: u32,
    size: u32,
}

#[repr(C)]
struct FramebufferTag {
    tag_type: u32,
    size: u32,
    address: u64,
    pitch: u32,
    width: u32,
    height: u32,
    bpp: u8,
    framebuffer_type: u8,
    reserved: u16,

    red_position: u8,
    red_mask_size: u8,
    green_position: u8,
    green_mask_size: u8,
    blue_position: u8,
    blue_mask_size: u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main(_magic: u32, _multiboot_info_addr: usize) -> ! {
    if _magic != MULTIBOOT2_BOOTLOADER_MAGIC {
        halt()
    }
    let info = _multiboot_info_addr as *const MultibootInfo;
    let total_size = unsafe { (*info).total_size as usize};
    
    let mut offset = 8usize;
    

    while offset < total_size {
        let tag = unsafe {
            &*((_multiboot_info_addr + offset) as *const Tag)
        };
        if tag.tag_type == TAG_TYPE_END {
            break;
        }
        if tag.tag_type == TAG_TYPE_FRAMEBUFFER {
            let framebuffer = unsafe {
                &*((_multiboot_info_addr + offset) as *const FramebufferTag)
            };

            let address = framebuffer.address;
            let pitch = framebuffer.pitch;
            let width = framebuffer.width;
            let height = framebuffer.height;
            let bpp = framebuffer.bpp;

            let _ = (address, pitch, width, height, bpp);
            unsafe {
                console::init(framebuffer.address as *mut u32,framebuffer.width as usize,framebuffer.height as usize,);

            }
            println!("Framebuffer: {:x}, {}x{}, {} bpp", address, width, height, bpp);
            break;
        }
        offset += (tag.size as usize + 7) & !7;
        

    }
    println!("Hello, world!");
    halt();
    
}
fn halt() -> ! {
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