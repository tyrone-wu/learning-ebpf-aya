use aya_ebpf::{macros::{map, raw_tracepoint}, maps::ProgramArray, programs::RawTracePointContext};
use aya_log_ebpf::info;

#[map]
static SYSCALL: ProgramArray = ProgramArray::with_max_entries(300, 0);

#[raw_tracepoint(tracepoint="sched_switch")]
pub fn hello_tail(ctx: RawTracePointContext) -> i32 {
    match try_hello_tail(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_hello_tail(ctx: RawTracePointContext) -> Result<i32, i32> {
    info!(&ctx, "tracepoint sched_switch called");
    Ok(0)
}

// fn hello(ctx: RawTracePointContext) -> Result<i32, i32> {
//     Ok(0)
// }

// fn hello_exec(ctx: RawTracePointContext) -> Result<i32, i32> {
//     Ok(0)
// }

// fn hello_timer(ctx: RawTracePointContext) -> Result<i32, i32> {
//     Ok(0)
// }
