#![no_std]

use core::mem::MaybeUninit;

const SLAB_SIZE: usize = 32;
 
pub struct Slab<T> {
    storage: [MaybeUninit<T>; SLAB_SIZE],
    used: [bool; SLAB_SIZE],
}

impl<T> Slab<T> {
    pub fn new() -> Self {  
        Self {
            storage: unsafe { MaybeUninit::uninit().assume_init() },
            used: [false; SLAB_SIZE],
        }
    }

    pub fn allocate(&mut self, value: T) -> Option<&mut T> {
        for i in 0..SLAB_SIZE {
            if !self.used[i] {
                self.used[i] = true;
                self.storage[i] = MaybeUninit::new(value);
                return unsafe { Some(&mut *self.storage[i].as_mut_ptr()) };
            }
        }
        None
    }

    pub fn deallocate(&mut self, ptr: *mut T) {
        for i in 0..SLAB_SIZE {
            if self.used[i] && self.storage[i].as_ptr() == ptr {
                self.used[i] = false;
                return;
            }
        }
    }
}

