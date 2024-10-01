#![no_std]
#![no_main]

//7E

use core::arch::asm;

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
        core::ptr::write_volatile(0x3f20_0008 as *mut u32, 1<<3);

        loop {
            // Turn pin 21 on
            core::ptr::write_volatile(0x3f20_001C as *mut u32, 1<<21);
            for _ in 0..500000 {
                asm!("nop");
            }

            // Turn pin 21 off
            core::ptr::write_volatile(0x3f20_0028 as *mut u32, 1<<21);
            for _ in 0..500000 {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}