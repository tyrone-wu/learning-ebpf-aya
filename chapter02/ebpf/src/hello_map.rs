use aya_ebpf::{macros::{kprobe, map}, maps::HashMap, programs::ProbeContext, EbpfContext};

#[map]
static COUNTER_TABLE: HashMap<u32, u64> = HashMap::<u32, u64>::with_max_entries(1024, 0);

#[kprobe]
pub fn hello_map(ctx: ProbeContext) -> i64 {
    match try_hello_map(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_hello_map(ctx: ProbeContext) -> Result<i64, i64> {
    let uid: u32 = ctx.uid(); // bpf_get_current_uid_gid() also exists toget the uid
    let p = COUNTER_TABLE.get_ptr_mut(&uid);
    if let Some(count) = p {
        unsafe { *count += 1 };
    } else {
        COUNTER_TABLE.insert(&uid, &1, 0)?;
    }

    Ok(0)
}
