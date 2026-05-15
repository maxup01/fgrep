#![no_std]
#![no_main]

use aya_ebpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};

#[xdp]
pub fn program(_ctx: XdpContext) -> u32 {
    xdp_action::XDP_PASS
}
