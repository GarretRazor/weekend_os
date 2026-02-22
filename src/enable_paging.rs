#[repr(transparent)]
pub struct Pte(u32);

impl Pte{
    pub fn new(ppn: u32 , flags: u8) -> self{
         Self(pp << 10) | flags(as u32))
    }
}
#[repr(4096))]
pub struct PageTable{
     pub entries: [Pte; 1024],
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
