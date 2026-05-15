use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetcapError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ebpf error: {0}")]
    Ebpf(#[from] aya::EbpfError),

    #[error("xdp map error: {0}")]
    XdpMap(#[from] aya::maps::MapError),
}
