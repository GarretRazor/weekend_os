use core::arch::asm;


//pub fn do_interrupt(){

//}


pub fn set_interrupt(){

    unsafe{ 
        //let handler_addr = do_interrupt as *const ();
        asm!(
            "csrw mtvec, {0}",
            in(reg) 0xdead);
    }
}
