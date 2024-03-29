use std::{path::PathBuf, process::Command};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    /// Build the release target
    #[clap(long)]
    pub release: bool,
    /// Chapter to build
    #[clap(name = "chapter", short, long)]
    pub chapter: u8,
}

pub fn build_userspace(opts: Options) -> Result<(), anyhow::Error> {
    let chapter = format!("{:0>2}", opts.chapter);

    let target_dir = format!("--target-dir=../../target/chapters/{chapter}");
    let mut args = vec!["build", target_dir.as_str()];
    if opts.release {
        args.push("--release")
    }

    let dir = PathBuf::from(format!("chapter{chapter}/userspace"));
    let status = Command::new("cargo")
        .current_dir(dir)
        .args(&args)
        .status()
        .expect("failed to build userspace program");
    assert!(status.success());

    Ok(())
}
