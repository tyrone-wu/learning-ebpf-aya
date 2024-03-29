use std::process::Command;

use anyhow::Context as _;
use clap::Parser;

use crate::{
    build_ebpf::{build_ebpf, Architecture, Options as EbpfOptions},
    build_userspace::{build_userspace, Options as UserspaceOptions},
};

#[derive(Debug, Parser)]
pub struct Options {
    /// Set the endianness of the BPF target
    #[clap(default_value = "bpfel-unknown-none", long)]
    pub bpf_target: Architecture,
    /// Build and run the release target
    #[clap(long)]
    pub release: bool,
    /// The command used to wrap your application
    #[clap(short, long, default_value = "sudo -E")]
    pub runner: String,
    /// Chapter to build
    #[clap(name = "chapter", short, long)]
    pub chapter: u8,
    /// Arguments to pass to your application
    #[clap(name = "program", short, long)]
    pub program: String,
}

pub fn build_run(opts: Options) -> Result<(), anyhow::Error> {
    // build ebpf program
    build_ebpf(EbpfOptions {
        target: opts.bpf_target,
        release: opts.release,
        chapter: opts.chapter,
    })
    .context("Error while building eBPF program")?;

    // build  and userspace application
    build_userspace(UserspaceOptions {
        release: opts.release,
        chapter: opts.chapter,
    })
    .context("Error while building userspace application")?;

    run(opts)
}

/// Build and run the project
pub fn run(opts: Options) -> Result<(), anyhow::Error> {
    // profile we are building (release or debug)
    let chapter = format!("{:0>2}", opts.chapter);
    let profile = if opts.release { "release" } else { "debug" };
    let bin_path = format!("target/chapters/{chapter}/{profile}/userspace");

    // configure args
    let mut args: Vec<_> = opts.runner.trim().split_terminator(' ').collect();
    args.push(bin_path.as_str());
    args.push(&opts.program);

    // run the command
    let status = Command::new(args.first().expect("No first argument"))
        .args(args.iter().skip(1))
        .status()
        .expect("failed to run the command");

    if !status.success() {
        anyhow::bail!("Failed to run `{}`", args.join(" "));
    }
    Ok(())
}
