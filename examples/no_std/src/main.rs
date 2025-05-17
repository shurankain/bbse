#![no_std]
#![no_main]

extern crate alloc;

use bitvec::vec::BitVec;
use bitvec::order::Msb0;
use bbse::{encode, encode_from, decode};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use core::alloc::Layout;
use alloc::alloc::GlobalAlloc;

struct DummyAlloc;

unsafe impl GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // do nothing
    }
}

#[global_allocator]
static ALLOC: DummyAlloc = DummyAlloc;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let path = encode(0, 16, 5);
    let value = decode(0, 16, &path);
    assert_eq!(value, 5);

    let path = encode_from(0, 16, 5, 8);
    let value = decode(0, 16, &path);
    assert_eq!(value, 5);

    let path = encode(42, 43, 42);
    assert!(path.is_empty());
    let value = decode(42, 43, &BitVec::<u8, Msb0>::new());
    assert_eq!(value, 42);

    let max = 255;
    let bits = encode(0, max + 1, max);
    assert_eq!(decode(0, max + 1, &bits), max);

    loop {}
}
