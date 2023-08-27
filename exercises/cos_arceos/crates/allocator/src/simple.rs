//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator{
    allocations:i32,
    start:usize,
    next:usize,
    size:usize
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            allocations:0,
            size:0, 
            next:0,
            start:0
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.allocations = 0;
        self.size = _size;
        self.start = _start;
        self.next = _start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
       self.size  += _size;
       Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        if _layout.size()>(self.size - (self.next - self.start))
        {
            Err(crate::AllocError::NoMemory)
        }
        else
        {
            let start_ptr = self.next;
            self.next = self.next+_layout.size();
            self.allocations+=1;
            return Ok(NonZeroUsize::new(start_ptr).unwrap());
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations-=1;
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.next
    }

    fn available_bytes(&self) -> usize {
        self.size-(self.next-self.start)
    }
}
