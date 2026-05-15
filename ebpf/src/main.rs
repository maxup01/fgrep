#![no_std]
#![no_main]

use core::panic::PanicInfo;

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::XskMap,
    programs::XdpContext,
};

#[map]
static SOCKETS: XskMap = XskMap::with_max_entries(1, 0);

#[xdp]
pub fn program(_ctx: XdpContext) -> u32 {
    SOCKETS.redirect(0, 0).unwrap_or(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
