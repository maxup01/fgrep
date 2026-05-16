use crate::mem::UmemReg;
use aya::{include_bytes_aligned, maps::XskMap, Ebpf};
use error::NetcapError;
use std::{
    io, mem,
    os::fd::{FromRawFd, OwnedFd},
};

#[allow(unused)]
pub(crate) struct EbpfProgram {
    ebpf: Ebpf,
    socket_fd: Option<OwnedFd>,
}

impl EbpfProgram {
    pub fn load() -> Result<Self, NetcapError> {
        let ebpf =
            Ebpf::load(include_bytes_aligned!(env!("EBPF_PATH"))).map_err(NetcapError::Ebpf)?;

        let ebpf_program = Self {
            ebpf,
            socket_fd: None,
        };

        Ok(ebpf_program)
    }

    pub fn setup_xsk(&mut self, ifindex: u32, xdp_umem_reg: UmemReg) -> Result<(), NetcapError> {
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

        let ring_size: usize = (xdp_umem_reg.len / 3) as usize;

        let result = unsafe {
            libc::setsockopt(
                socket_fd,
                libc::SOL_XDP,
                libc::XDP_UMEM_FILL_RING,
                &ring_size as *const _ as *const libc::c_void,
                mem::size_of::<u32>() as libc::socklen_t,
            )
        };

        if result == -1 {
            return Err(NetcapError::Io(io::Error::last_os_error()));
        }

        let result = unsafe {
            libc::setsockopt(
                socket_fd,
                libc::SOL_XDP,
                libc::XDP_UMEM_COMPLETION_RING,
                &ring_size as *const _ as *const libc::c_void,
                mem::size_of::<u32>() as libc::socklen_t,
            )
        };

        if result == -1 {
            return Err(NetcapError::Io(io::Error::last_os_error()));
        }

        let result = unsafe {
            libc::setsockopt(
                socket_fd,
                libc::SOL_XDP,
                libc::XDP_RX_RING,
                &ring_size as *const _ as *const libc::c_void,
                mem::size_of::<u32>() as libc::socklen_t,
            )
        };

        if result == -1 {
            return Err(NetcapError::Io(io::Error::last_os_error()));
        }

        let sxdp = libc::sockaddr_xdp {
            sxdp_family: libc::AF_XDP as u16,
            sxdp_flags: 0,
            sxdp_ifindex: ifindex,
            sxdp_queue_id: 0,
            sxdp_shared_umem_fd: 0,
        };

        let result = unsafe {
            libc::bind(
                socket_fd,
                &sxdp as *const _ as *const libc::sockaddr,
                mem::size_of::<libc::sockaddr_xdp>() as libc::socklen_t,
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
        self.socket_fd = Some(unsafe { OwnedFd::from_raw_fd(socket_fd) });

        Ok(())
    }
}
