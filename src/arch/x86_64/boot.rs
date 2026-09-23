use crate::console;
use crate::println;
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

/// ブート情報の検証と初期化（Multiboot2の詳細をここに隠蔽）
pub fn init(magic: u32, multiboot_info_addr: usize) {
    if magic != MULTIBOOT2_BOOTLOADER_MAGIC {
        crate::halt();
    }

    let info = multiboot_info_addr as *const MultibootInfo;
    let total_size = unsafe { (*info).total_size as usize };
    let mut offset = 8usize;

    while offset < total_size {
        let tag = unsafe { &*((multiboot_info_addr + offset) as *const Tag) };
        if tag.tag_type == TAG_TYPE_END {
            break;
        }
        if tag.tag_type == TAG_TYPE_FRAMEBUFFER {
            let fb = unsafe { &*((multiboot_info_addr + offset) as *const FramebufferTag) };
            
            unsafe {
                console::init(
                    fb.address as *mut u32,
                    fb.width as usize,
                    fb.height as usize,
                );
            }
            println!(
                "Framebuffer: {:x}, {}x{}, {} bpp",
                fb.address, fb.width, fb.height, fb.bpp
            );
            break;
        }
        offset += (tag.size as usize + 7) & !7;
    }
}