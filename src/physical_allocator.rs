pub struct FrameAllocator{
        bitmap: [u32; 1024],
        start_addr: usize,
}

impl FrameAllocator {
     pub const fn new(start_addr: usize) -> Self {
          Self { 
               bitmap: [0; 1024], start_addr,
          }
     }
pub fn alloc_frame(&mut self) -> Option<usize> {
        for(i, word) in self.bitmap.iter_mut().enumerate(){
                if *word != !0 {
                      let bit = word.trailing_ones() as usize;
                      *word |= 1 << bit; 
                      let frame_idx = (i * 32) + bit;
                      return Some(self.start_addr + (frame_idx * 4096));
                }
        }
        None
}
pub fn free_frame(&mut self , addr: usize) {
       let frame_idx = (addr - self.start_addr) / 4096;
       let i = frame_idx / 32; 
       let bit = frame_idx % 32;
       self.bitmap[i] &= !(1 << bit);
     }

