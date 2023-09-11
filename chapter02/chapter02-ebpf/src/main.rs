#![no_std]
#![no_main]

use aya_bpf::{macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::info;

#[kprobe]
pub fn chapter02(ctx: ProbeContext) -> u32 {
    match try_chapter02(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_chapter02(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function __x64_sys_execve called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
