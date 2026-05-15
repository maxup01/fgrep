#![no_std]

#[repr(C)]
pub struct PacketMetadata {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
    pub flags: u8,
    pub length: u16,
    pub timestamp: u64,
    pub payload: [u8; 256],
}
