#[cfg(feature = "allocator-api2")]
use allocator_api2::alloc::{AllocError, Allocator};
#[cfg(feature = "allocator-api2")]
use std::alloc::{GlobalAlloc, System};
#[cfg(feature = "allocator-api2")]
use std::ptr::NonNull;
#[cfg(feature = "allocator-api2")]
use std::slice;
use std::alloc::Layout;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

#[cfg(feature = "allocator-api2")]
pub use allocator_api2::vec::Vec;

static ALLOC: AtomicUsize = AtomicUsize::new(0);
static DEALLOC: AtomicUsize = AtomicUsize::new(0);

pub struct TrackingAllocator;

pub fn reset() {
    ALLOC.store(0, SeqCst);
    DEALLOC.store(0, SeqCst);
}

pub fn record_alloc(layout: Layout) {
    ALLOC.fetch_add(layout.size(), SeqCst);
}

pub fn record_dealloc(layout: Layout) {
    DEALLOC.fetch_add(layout.size(), SeqCst);
}

pub fn stats() -> Stats {
    let alloc: usize = ALLOC.load(SeqCst);
    let dealloc: usize = DEALLOC.load(SeqCst);
    let diff = (alloc as isize) - (dealloc as isize);

    Stats {
        alloc,
        dealloc,
        diff,
    }
}

#[cfg(feature = "allocator-api2")]
unsafe impl Allocator for TrackingAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            let ptr = System.alloc(layout);
            if ptr.is_null() {
                Err(AllocError)
            } else {
                let slice_ptr: *mut [u8] = slice::from_raw_parts_mut(ptr, layout.size());
                let non_null_slice: NonNull<[u8]> = NonNull::new_unchecked(slice_ptr);
                record_alloc(layout);

                Ok(non_null_slice)
            }
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        record_dealloc(layout);
        let raw_ptr: *mut u8 = ptr.as_ptr();
        System.dealloc(raw_ptr, layout);
    }
}

pub struct Stats {
    pub alloc: usize,
    pub dealloc: usize,
    pub diff: isize,
}
