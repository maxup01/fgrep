use std::{io, ptr, slice};

#[allow(dead_code)]
pub struct Umem {
    pub data: &'static mut [u8],
    mem_ptr: *mut libc::c_void,
    len: usize,
}

impl Umem {
    pub fn new(huge_pages_num: usize) -> Result<Self, io::Error> {
        const HUGE_PAGE_SIZE: usize = 1024 * 1024 * 2;

        let len: usize = HUGE_PAGE_SIZE * huge_pages_num;

        let mmap_ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_ANONYMOUS | libc::MAP_HUGETLB,
                -1,
                0,
            )
        };

        if mmap_ptr == libc::MAP_FAILED {
            return Err(io::Error::last_os_error());
        }

        let data = unsafe { slice::from_raw_parts_mut(mmap_ptr as *mut u8, len) };

        let umem = Self {
            data,
            mem_ptr: mmap_ptr,
            len,
        };

        Ok(umem)
    }
}
