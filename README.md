# todo

## Prerequisites

1. Install bpf-linker: `cargo install bpf-linker`

## Build eBPF

```bash
$ cargo build-ebpf <chapter-number>
# Example: cargo build-ebpf 2
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag.

## Build Userspace

```bash
cargo build-us <chapter-number>
# Example: cargo build-us 2
```

## Build eBPF & Userspace and Run

```bash
cargo rb -c <chapter-number> -p <program-name>
# cargo rb -c 2 -p hello_map
```

Programs can be found under:
```
.
└─chapterXX/
   └─ebpf/
      └─src/
         ├─main.rs
         ├─<prog-name>.rs
         ├─...
         └─<prog-name>.rs
```

## Run

```bash
cargo r -c <chapter-number> -p <program-name>
# cargo r -c 2 -p hello_map
```
