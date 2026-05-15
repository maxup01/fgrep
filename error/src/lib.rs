use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetcapError {
    #[error("Error io error occured: {0}")]
    IoError(String),
    #[error("Error xdp error occured: {0}")]
    XdpError(String),
}
