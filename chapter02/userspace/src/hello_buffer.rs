use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use aya::maps::RingBuf;
use aya::programs::KProbe;
use aya::Ebpf;
use log::info;
use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};

pub(crate) async fn hello_buffer(bpf: &mut Ebpf) -> Result<(), anyhow::Error> {
    let program: &mut KProbe = bpf.program_mut("hello_buffer").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_execve", 0)?;

    let sig = Arc::new(AtomicBool::new(true));
    watch_signal(Arc::clone(&sig));

    let mut output: RingBuf<_> = bpf.take_map("OUTPUT").unwrap().try_into()?;
    while sig.load(Ordering::Relaxed) {
        match output.next() {
            Some(data) => {
                info!("{:?}", &*data);
            }
            None => {}
        }
    }

    signal::ctrl_c().await?;
    Ok(())
}

fn watch_signal(sig: Arc<AtomicBool>) {
    tokio::spawn(async move {
        let mut stream = signal(SignalKind::interrupt()).unwrap();
        stream.recv().await;
        sig.store(false, Ordering::Relaxed);
    });
    info!("Waiting for Ctrl-C...");
}
