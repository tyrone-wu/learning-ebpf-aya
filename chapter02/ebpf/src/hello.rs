use aya_ebpf::{macros::kprobe, programs::ProbeContext, EbpfContext};
use aya_log_ebpf::info;

// Converting ctx.command to str leaves trailing null chars.
const CPU_USAGE_CMD: &[u8; 16] = b"cpuUsage.sh\0\0\0\0\0";
const NODE_CMD: &[u8; 16] = b"node\0\0\0\0\0\0\0\0\0\0\0\0";

#[kprobe]
pub fn hello(ctx: ProbeContext) -> u32 {
    match try_hello(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_hello(ctx: ProbeContext) -> Result<u32, u32> {
    match ctx.command() {
        Ok(cmd) => {
            // I'm using vscode to connect to this vm so there's a lot of 'node' and 'cpuUsage.sh'
            // spam in the background.
            if &cmd != CPU_USAGE_CMD && &cmd != NODE_CMD {
                let cmd_str = unsafe { core::str::from_utf8_unchecked(&cmd) };
                info!(&ctx, "Hello World! {}", cmd_str);
            }
        },
        Err(_) => {},
    }

    Ok(0)
}
