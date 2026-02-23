#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Pte(usize);
const KERNEL_PHYSICAL_ADDRESS: usize = 0x80000000;
impl Pte{
    pub fn new(ppn: usize , flags: u8) -> Self{
         Self((ppn << 10) | (flags as usize))
    }
}
#[repr(align(4096))]
pub struct PageTable{
     pub entries: [Pte; 1024],
}

#[repr(align(4096))]
pub struct LeafPageTable{
          pub entries: [Pte; 1024],
}
pub static mut KERNEL_LEAF_TABLE: LeafPageTable = LeafPageTable{
      entries: [Pte(0); 1024],
};
pub static mut ROOT_PAGE_TABLE: PageTable = PageTable{
         entries: [const { Pte(0) }; 1024],
     };
pub mod flags{
       pub const Valid: u8 = 1 << 0; 
       pub const Readable: u8 = 1 << 1;
       pub const Writeable: u8 = 1 << 2; 
       pub const Executeable: u8 = 1 << 3; 
       pub const User: u8 = 1 << 4;
}


pub unsafe fn get_index(physical_address: usize ) -> usize
{
    const SHIFT: usize = 22;
    return physical_address >> SHIFT;
}
pub unsafe fn get_ppn(physical_address: usize) -> usize {
    const SHIFT: usize = 12;
    return physical_address >> SHIFT;
}
pub unsafe fn setup_root_table(root: &mut PageTable){
          let physical_index = get_index(KERNEL_PHYSICAL_ADDRESS);
          let leaf_addr = core::ptr::addr_of!(KERNEL_LEAF_TABLE) as usize;
          let leaf_ppn = get_ppn(leaf_addr);
          root.entries[physical_index] = Pte::new(leaf_ppn, flags::Valid | flags::Readable | flags::Executeable);
          let high_memory_index = 1023;
          root.entries[high_memory_index] = Pte::new(leaf_ppn, flags::Valid | flags::Readable | flags::Executeable);

        }
pub unsafe fn set_satp_value(ppn: usize) -> usize
{
    return ppn | (1 << 31);
}

pub unsafe fn enable_paging(root_table_addr: usize){
    let ppn = get_ppn(root_table_addr);
    let satp_value = set_satp_value(ppn);
    unsafe {
        core::arch::asm!(
            "csrw satp, {0}",
            "sfence.vma", in(reg) satp_value
            );
           }
}
#[inline(always)]
pub unsafe fn flush_tlb() {
          unsafe {
              core::arch::asm!(
                  "sfence.vma zero, zero");
          }
}
pub unsafe fn setup_kernel_leaf(leaf: &mut LeafPageTable){
    for i in 0 .. 32 { 
          let phys_addr = 0x80000000 + (i * 4096);
          let ppn = get_ppn(phys_addr) as usize; 
          leaf.entries[i] = Pte::new(ppn, 
        flags::Valid | flags::Readable | flags::Writeable | flags::Executeable);
     } 

}
