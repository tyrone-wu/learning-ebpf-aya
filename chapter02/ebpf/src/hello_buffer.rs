use core::mem;

use aya_ebpf::{macros::{kprobe, map}, maps::RingBuf, programs::ProbeContext, EbpfContext};
use aya_log_ebpf::warn;

use common_02::DataT;

const CPU_USAGE_CMD: &[u8; 16] = b"cpuUsage.sh\0\0\0\0\0";
const NODE_CMD: &[u8; 16] = b"node\0\0\0\0\0\0\0\0\0\0\0\0";

const MESSAGE: &[u8; 12] = b"Hello World\0";

#[map]
static OUTPUT: RingBuf = RingBuf::with_byte_size(4 * mem::size_of::<DataT>() as u32, 0);

#[kprobe]
pub fn hello_buffer(ctx: ProbeContext) -> u32 {
    match try_hello_buffer(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_hello_buffer(ctx: ProbeContext) -> Result<u32, u32> {
    let command = ctx.command().unwrap();
    if &command == CPU_USAGE_CMD || &command == NODE_CMD {
        return Ok(1);
    }

    let pid = ctx.pid();
    let uid = ctx.uid();

    match OUTPUT.reserve::<DataT>(0) {
        Some(mut buf) => {
            unsafe {
                (*buf.as_mut_ptr()).pid = pid;
                (*buf.as_mut_ptr()).uid = uid;
                (*buf.as_mut_ptr()).command = command;
                (*buf.as_mut_ptr()).message = *MESSAGE;
            };
            buf.submit(0);
        },
        None => warn!(&ctx, "not enough to reserve")
    }

    Ok(0)
}
