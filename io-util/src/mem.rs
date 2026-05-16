use std::{io, ptr, slice};

use error::NetcapError;

#[repr(C)]
pub struct UmemReg {
    pub addr: u64,
    pub len: u64,
    pub chunk_size: u32,
    pub headroom: u32,
    pub flags: u32,
}

#[allow(dead_code)]
pub struct Umem {
    pub data: &'static mut [u8],
    mem_ptr: *mut libc::c_void,
    len: usize,
    chunk_size: usize,
}

impl Umem {
    pub fn new(huge_pages_num: usize, chunk_size_kb: usize) -> Result<Self, NetcapError> {
        const HUGE_PAGE_SIZE: usize = 1024 * 1024 * 2;
        const CHUNK_BASE: usize = 1024;

        let len: usize = HUGE_PAGE_SIZE * huge_pages_num;
        let chunk_size: usize = CHUNK_BASE * chunk_size_kb;

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
            return Err(NetcapError::Io(io::Error::last_os_error()));
        }

        let data = unsafe { slice::from_raw_parts_mut(mmap_ptr as *mut u8, len) };

        let umem = Self {
            data,
            mem_ptr: mmap_ptr,
            len,
            chunk_size,
        };

        Ok(umem)
    }

    pub fn umem_reg(&self) -> UmemReg {
        UmemReg {
            addr: self.mem_ptr as u64,
            len: self.len as u64,
            chunk_size: self.chunk_size as u32,
            headroom: 0,
            flags: 0,
        }
    }
}

impl Drop for Umem {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.mem_ptr, self.len) };
    }
}
