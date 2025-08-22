use anyhow::Context;
use aya::programs::{Xdp, XdpFlags};
use aya::Ebpf;
use log::info;
use tokio::signal;

pub(crate) async fn hello(bpf: &mut Ebpf) -> Result<(), anyhow::Error> {
    // Net interface to attach to
    let iface = "eth0";

    let program: &mut Xdp = bpf.program_mut("hello").unwrap().try_into()?;
    program.load()?;
    program.attach(iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;

    Ok(())
}
