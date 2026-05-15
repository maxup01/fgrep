use crate::mem::XdpUmemReg;
use aya::{Ebpf, EbpfError};
use std::{io, mem};

pub(crate) struct EbpfProgram {
    ebpf: Ebpf,
}

impl EbpfProgram {
    pub fn load() -> Result<Self, EbpfError> {
        let ebpf = Ebpf::load(include_bytes_aligned!(env!("EBPF_PATH")))?;
        let ebpf_program = Self { ebpf };

        Ok(ebpf_program)
    }

    pub fn register_umem(&self, xdp_umem_reg: XdpUmemReg) -> Result<(), io::Error> {
        let socket_fd = unsafe { libc::socket(libc::AF_XDP, libc::SOCK_RAW, 0) };

        let result = unsafe {
            libc::setsockopt(
                socket_fd,
                libc::SOL_XDP,
                libc::XDP_UMEM_REG,
                &xdp_umem_reg as *const _ as *const libc::c_void,
                mem::size_of::<XdpUmemReg>() as libc::socklen_t,
            )
        };

        if result == -1 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}
