#![no_std]
#![no_main]

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Turn pin 21 into an output
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1<<3);

        loop {
            // Turn pin 21 on
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1<<3);
            for _ in 0..500000 {
                core::ptr::read_volatile(0x3F20_001C as *const u32);
            }

            // Turn pin 21 off
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1<<3);
            for _ in 0..500000 {
                core::ptr::read_volatile(0x3F20_001C as *const u32);
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}