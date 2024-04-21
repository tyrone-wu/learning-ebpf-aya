use aya::{programs::RawTracePoint, Ebpf};
use log::info;
use tokio::signal;

pub(crate) async fn hello_tail(bpf: &mut Ebpf) -> Result<(), anyhow::Error> {
    let program: &mut RawTracePoint = bpf.program_mut("hello_tail").unwrap().try_into()?;
    program.load()?;
    program.attach("sched_switch")?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;

    Ok(())
}
