use core::mem;
use std::alloc::{alloc, dealloc, Layout};


#[no_mangle]
pub unsafe extern "C" fn make_buffer(size: usize) -> usize {
    alloc(Layout::array::<u8>(size).expect("capacity overflow")) as usize
}

#[no_mangle]
pub unsafe extern "C" fn drop_buffer(ptr: usize, size: usize) {
    dealloc(
        ptr as *mut u8,
        Layout::array::<u8>(size).expect("capacity overflow"),
    );
}

pub struct MemPtr {
    pub ptr: i32,
    pub len: i32,
}

impl MemPtr {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr as *mut u8, self.len as usize) }
    }
}

impl From<i64> for MemPtr {
    fn from(val: i64) -> Self {
        let (ptr, len) = unsafe { mem::transmute::<i64, (i32, i32)>(val) };
        MemPtr { ptr, len }
    }
}

impl Drop for MemPtr {
    fn drop(&mut self) {
        unsafe {
            drop_buffer(self.ptr as usize, self.len as usize);
        }
    }
}
