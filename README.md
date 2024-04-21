# learning-ebpf-aya

Following along [lizrice](https://github.com/lizrice)'s [Learning eBPF](https://www.amazon.com/Learning-eBPF-Programming-Observability-Networking/dp/1098135121). 

repo reference: https://github.com/lizrice/learning-ebpf

### Prerequisites

1. Install bpf-linker: `cargo install bpf-linker`

### Build eBPF for Chapter

```bash
$ cargo build-ebpf <chapter-number>
# Example: cargo build-ebpf 2
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag.

### Build Userspace for Chapter

```bash
$ cargo build-us <chapter-number>
# Example: cargo build-us 2
```

### Build eBPF & Userspace and Run Program

```bash
$ cargo br -c <chapter-number> -p <program-name>
# cargo br -c 2 -p hello
```

Programs can be found under:
```
.
├─...
└─chapterXX/
   └─ebpf/
      └─src/
         ├─main.rs
         ├─<prog-name>.rs
         ├─...
         └─<prog-name>.rs
```

### Run Program

```bash
$ cargo r -c <chapter-number> -p <program-name>
# cargo r -c 2 -p hello
```
