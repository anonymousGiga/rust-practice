// Source: https://github.com/korpling/graphANNIS/blob/main/core/src/util/memory_estimation.rs#L70

#[cfg(not(windows))]
pub mod platform {
    use std::os::raw::c_void;

    // Defines which actual function is used.
    //
    // We always use the system malloc instead of jemalloc.
    // On MacOS X, the external function is not called "malloc_usable_size", but "malloc_size"
    // (it basically does the same).
    extern "C" {
        #[cfg_attr(any(target_os = "macos", target_os = "ios"), link_name = "malloc_size")]
        fn malloc_usable_size(ptr: *const c_void) -> usize;
    }

    /// Get the size of a heap block.
    ///
    /// # Safety
    ///
    /// This calls a native system function with a user-provided pointer.
    /// While the pointer is checked not to be null, it can still point to any memory somewhere and causing problematic
    /// behavior. Therefore, calling this function is unsafe.
    pub unsafe extern "C" fn usable_size(ptr: *const c_void) -> usize {
        if ptr.is_null() {
            0
        } else {
            malloc_usable_size(ptr)
        }
    }
}
