#![no_std]
#![no_main]
use core::arch::asm;

mod interrupts;
//use interrupts::do_interrupt;
use interrupts::set_interrupt;
#[panic_handler]
fn panic(__info: &core::panic::PanicInfo) -> ! { 
           loop{}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub unsafe extern "C" fn _start() -> ! { 
    //set_interrupt();
    unsafe { asm!( 
        "la sp, __stack_top",
        "j {main}",
        main = sym rust_main,
        options(noreturn)
        );
    }
}

fn rust_main() -> !{
      set_interrupt();
      loop{};
}
