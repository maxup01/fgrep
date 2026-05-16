use crate::{ebpf::EbpfProgram, mem::Umem};
use error::NetcapError;

pub struct XskReceiver {
    ebpf_program: EbpfProgram,
    umem: Umem,
}

impl XskReceiver {
    pub fn new(ifindex: u32) -> Result<Self, NetcapError> {
        let umem = Umem::new(5, 2)?;

        let mut ebpf_program = EbpfProgram::load()?;
        ebpf_program.setup_xsk(ifindex, umem.umem_reg())?;

        let packet_sniffer = XskReceiver { ebpf_program, umem };

        Ok(packet_sniffer)
    }
}
