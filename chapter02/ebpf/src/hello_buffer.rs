use core::mem;

use aya_ebpf::{macros::{kprobe, map}, maps::RingBuf, programs::ProbeContext, EbpfContext};
use aya_log_ebpf::info;

use common::DataT;

// #[map]
// static OUTPUT: PerfEventArray<DataT> = PerfEventArray::with_max_entries(1, 0);

#[map]
static OUTPUT: RingBuf = RingBuf::with_byte_size(1 * mem::size_of::<DataT>() as u32, 0);

#[kprobe]
pub fn hello_buffer(ctx: ProbeContext) -> u32 {
    match try_hello_buffer(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_hello_buffer(ctx: ProbeContext) -> Result<u32, u32> {
    let pid = ctx.pid();
    let uid = ctx.uid();
    let command = ctx.command().expect("oopsie can't get command"); // bpf_get_current_comm()
    let message: [u8; 12] = "Hello World".as_bytes().try_into().expect("incorrect length");

    let data_t = DataT {
        pid,
        uid,
        command,
        message,
    };
    // OUTPUT.output(&ctx, &data_t, 0);
    match OUTPUT.reserve::<DataT>(0) {
        Some(mut buf) => {
            unsafe { *buf.as_mut_ptr() = data_t };
            buf.submit(0);
        },
        None => info!(&ctx, "not enough space to reserve")
    }

    Ok(0)
}
