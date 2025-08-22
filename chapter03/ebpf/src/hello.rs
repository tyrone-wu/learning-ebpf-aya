use aya_ebpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_log_ebpf::info;

#[no_mangle]
static mut COUNTER: i32 = 0;

#[xdp]
pub fn hello(ctx: XdpContext) -> u32 {
    match try_hello(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_hello(ctx: XdpContext) -> Result<u32, u32> {
    // unsafe { COUNTER += 1 };
    let count = unsafe {
        COUNTER += 1;
        COUNTER
    };
    info!(&ctx, "Hello World {}", count);
    Ok(xdp_action::XDP_PASS)
}
