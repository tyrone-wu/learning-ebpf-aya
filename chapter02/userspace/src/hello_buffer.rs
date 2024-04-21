use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::str;

use aya::maps::RingBuf;
use aya::programs::KProbe;
use aya::Ebpf;
use log::info;
use tokio::signal;
use tokio::signal::unix::{signal, SignalKind};

use common_02::DataT;

pub(crate) async fn hello_buffer(bpf: &mut Ebpf) -> Result<(), anyhow::Error> {
    let program: &mut KProbe = bpf.program_mut("hello_buffer").unwrap().try_into()?;
    program.load()?;
    program.attach("__x64_sys_execve", 0)?;

    let sig = Arc::new(AtomicBool::new(true));
    watch_signal(Arc::clone(&sig));

    let mut output: RingBuf<_> = bpf.take_map("OUTPUT").unwrap().try_into()?;

    while sig.load(Ordering::Relaxed) {
        match output.next() {
            Some(raw) => {
                // // decode raw binary into DataT; not sure if there's already something builtin for this
                // let pid: u32 = raw[16..20].iter().rev().fold(0, |mut acc, byte| {
                //     acc <<= 8;
                //     acc |= *byte as u32;
                //     acc
                // });
                // let uid: u32 = raw[20..24].iter().rev().fold(0, |mut acc, byte| {
                //     acc <<= 8;
                //     acc |= *byte as u32;
                //     acc
                // });
                // let command = unsafe { core::str::from_utf8_unchecked(&raw[0..16]) };
                // let message = unsafe { core::str::from_utf8_unchecked(&raw[24..]) };

                // no need to manually decode, cast to DataT pointer
                let data_t = raw.as_ptr() as *const DataT;
                let (pid, uid, command, message) = unsafe {(
                    (*data_t).pid,
                    (*data_t).uid,
                    str::from_utf8(&(*data_t).command)?,
                    str::from_utf8(&(*data_t).message)?,
                )};

                info!("pid: {pid}\tuid: {uid}\tcmd: {command}\tmsg: {message}");
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
