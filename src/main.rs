#![no_std]
#![no_main]
use core::arch::asm;

mod interrupts;
mod enable_paging;
mod traps;
//use interrupts::do_interrupt;
use interrupts::set_interrupt;
use enable_paging::enable_paging;
use enable_paging::setup_root_table;
use enable_paging::ROOT_PAGE_TABLE;
use enable_paging::setup_kernel_leaf;
use enable_paging::KERNEL_LEAF_TABLE;
use enable_paging::flush_tlb;
use traps::m_trap_handler;
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

     unsafe{ 
      let root_ptr = core::ptr::addr_of_mut!(ROOT_PAGE_TABLE);
      let leaf_ptr = core::ptr::addr_of_mut!(KERNEL_LEAF_TABLE);
      setup_kernel_leaf(&mut *leaf_ptr);
      setup_root_table(&mut *root_ptr);
      core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
      
      let handler_addr = m_trap_handler as usize;
      asm!("csrw mtvec, {}", in(reg) handler_addr);
      enable_paging(core::ptr::addr_of! (ROOT_PAGE_TABLE) as usize);

      flush_tlb();
     }
      loop{};
}
