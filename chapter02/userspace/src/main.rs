use std::env::args;

use aya::{include_bytes_aligned, Ebpf};
use aya_log::EbpfLogger;
use log::{debug, info, warn};

mod hello;
mod hello_buffer;
mod hello_map;
mod hello_tail;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!("Program missing.");
    }

    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {}", ret);
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    // TODO: probly try to modularize the loading since only dir changes
    #[cfg(debug_assertions)]
    let mut bpf = Ebpf::load(include_bytes_aligned!(
        "../../../target/chapters/02/bpfel-unknown-none/debug/ebpf-02"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Ebpf::load(include_bytes_aligned!(
        "../../../target/chapters/02/bpfel-unknown-none/release/ebpf-02"
    ))?;
    if let Err(e) = EbpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }

    match args[1].as_str() {
        "hello" => hello::hello(&mut bpf).await?,
        "hello_map" => hello_map::hello_map(&mut bpf).await?,
        "hello_buffer" => hello_buffer::hello_buffer(&mut bpf).await?,
        "hello_tail" => hello_tail::hello_tail(&mut bpf).await?,
        _ => panic!("Program {} does not exist.", args[1]),
    }

    info!("Exiting...");
    Ok(())
}
