use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use aya::maps::HashMap;
use aya::programs::KProbe;
use aya::Ebpf;
use log::info;
use tokio::signal::unix::{signal, SignalKind};
use tokio::time::sleep;

pub(crate) async fn hello_map(bpf: &mut Ebpf) -> Result<(), anyhow::Error> {
    let program: &mut KProbe = bpf.program_mut("hello_map").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_execve", 0)?;

    let sig = Arc::new(AtomicBool::new(true));
    watch_signal(Arc::clone(&sig));

    while sig.load(Ordering::Relaxed) {
        println!("---");
        let counter_table: HashMap<_, u32, u64> =
            HashMap::try_from(bpf.map("COUNTER_TABLE").unwrap())?;
        for entry in counter_table.iter() {
            let (gid, count) = entry?;
            println!("{gid}: {count}");
        }

        sleep(Duration::from_secs(2)).await;
    }

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
