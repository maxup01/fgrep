use crate::mem::UmemReg;
use aya::{include_bytes_aligned, maps::XskMap, Ebpf};
use error::NetcapError;
use std::{io, mem};

pub(crate) struct EbpfProgram {
    ebpf: Ebpf,
}

impl EbpfProgram {
    pub fn load() -> Result<Self, NetcapError> {
        let ebpf =
            Ebpf::load(include_bytes_aligned!(env!("EBPF_PATH"))).map_err(NetcapError::Ebpf)?;

        let ebpf_program = Self { ebpf };

        Ok(ebpf_program)
    }

    pub fn attach_umem_to_ebpf(&mut self, xdp_umem_reg: UmemReg) -> Result<(), NetcapError> {
        let socket_fd = unsafe { libc::socket(libc::AF_XDP, libc::SOCK_RAW, 0) };

        let result = unsafe {
            libc::setsockopt(
                socket_fd,
                libc::SOL_XDP,
                libc::XDP_UMEM_REG,
                &xdp_umem_reg as *const _ as *const libc::c_void,
                mem::size_of::<UmemReg>() as libc::socklen_t,
            )
        };

        if result == -1 {
            return Err(NetcapError::Io(io::Error::last_os_error()));
        }

        let mut xsk_map = XskMap::try_from(
            self.ebpf
                .map_mut("SOCKETS")
                .expect("SOCKETS should exist in ebpf program"),
        )
        .map_err(NetcapError::XdpMap)?;

        xsk_map.set(0, socket_fd, 0)?;

        Ok(())
    }
}
