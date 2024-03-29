use aya_ebpf::{macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::info;

#[kprobe]
pub fn hello(ctx: ProbeContext) -> u32 {
    match try_hello(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_hello(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "Hello World!");
    Ok(0)
}
