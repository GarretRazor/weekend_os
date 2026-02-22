#[repr(transparent)]
pub struct Pte(u32);

impl Pte{
    pub fn new(ppn: u32 , flags: u8) -> self{
         Self(pp << 10) | flags(as u32))
    }
}
#[repr((4096))]
pub struct PageTable{
     pub entries: [Pte; 1024],
}
pub static mut ROOT_PAGE_TABLE:
     PageTable = PageTable{
         entries: [Pte(0); 1024],
     };
pub mod flags{
       pub const Valid: u8 = 1 << 0; 
       pub const Readble: u8 = 1 << 1;
       pub const Writeable: u8 = 1 << 2; 
       pub const Executable: u8 = 1 << 3; 
       pub const User: u8 = 1 << 4;
}

pub unsafe fn setup_root_table(root: &mut PageTable){
    let ram_physical_addr = 0x80000000;
    let ram_idx = ram_phys_addr >> 12;
    let ram_ppn = ram_phys_addr >> 12; 
    root.entries[ram_idx] = Pte::new(ram_ppn as u32, flags::Valid | flags::Readable | flags::Executable);
    let uart_phys_addr = 0x10000000;
    let uart_idx = uart_phys_addr >> 22;
    let uart_ppn = uart_phys_addr >>12;
    root.entries[uart_idx] = Pte::new(uart_ppn as u32, flags::Valid | flags::Readable | flags::Writeable);
} 
pub unsafe fn enable_paging(root_table_addr: usize) {
     let ppn = root_table_addr >> 12; 
     let satp_val = (1 << 31) | ppn;

     unsafe {
         core::arch::asm! ( 
             "csrw satp, {0}"
             ,"sfence.vma", in(reg) satp_val
             );
     }
}


