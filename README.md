# specs
Get systems specs in rust

Prints CPU, core, and memory information read from `sysctl` on macOS.
Performance/efficiency core counts are only available on Apple Silicon
running macOS 12 or later; keys that aren't present are shown as `n/a`.
On other platforms the program builds but exits with an "unsupported" message.

```
$ cargo run
machdep.cpu.brand_string: Apple M3
hw.ncpu: 8
hw.perflevel0.physicalcpu: 4
hw.perflevel1.physicalcpu: 4
hw.memsize: 24.0 GiB
```

MSRV: 1.61.0
