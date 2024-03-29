use aya::programs::KProbe;
use aya::Ebpf;
use log::info;
use tokio::signal;

pub(crate) async fn hello(bpf: &mut Ebpf) -> Result<(), anyhow::Error> {
    let program: &mut KProbe = bpf.program_mut("hello").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_execve", 0)?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;

    Ok(())
}
