use core::arch::asm;

#[unsafe(no_mangle)] 
pub extern "C" fn m_trap_handler() {
    let mcause: usize;
    let mtval: usize; 
    let mepc: usize; 
    unsafe {
        asm!("csrr {}, mcause", out(reg) mcause);
        asm!("csrr {}, mtval", out(reg) mtval);
        asm!("csrr {}, mepc", out(reg) mepc);

        panic!( 
            "Trap! mcause: {:#x}, mepc: {:#x}, mepc: {:#x}",
            mcause, mtval, mepc
            );
    }
}
