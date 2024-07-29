# Rust Async Framework

## Tools

* `lscpu -C` can show `COHERENCY-SIZE` as the "minimum amount of data in bytes transferred from memory to cache".
* Can show thread names in htop by F2 → Display options → Show custom thread names

## TODO

- [ ] Start with https://github.com/polyfractal/bounded-spsc-queue (look at recent PRs to get building), and compare with [crossbeam](https://github.com/crossbeam-rs/crossbeam) primitives/performance

## References

* https://eli.thegreenplace.net/2016/c11-threads-affinity-and-hyperthreading/
* https://kb.ettus.com/Getting_Started_with_DPDK_and_UHD
* https://github.com/crossbeam-rs/rfcs/wiki learning resources
  + C++ Concurrency in Action, Second Edition
* [Is Parallel Programming Hard, And, If So, What Can You Do About It? (Release v2023.06.11a)](https://arxiv.org/abs/1701.00854)

## Repos/References to use and try

- [ ] [Rust `std` library](https://doc.rust-lang.org/std/index.html)
- [ ] [Thread pool - Wikipedia](https://en.wikipedia.org/wiki/Thread_pool)
- [ ] [disruptor-rs](https://github.com/nicholassm/disruptor-rs)
- [ ] [clap](https://github.com/clap-rs/clap): command line argument parser
- [ ] [tui-rs](https://github.com/fdehau/tui-rs): Terminal User Interface (TUI) and dashboard crate
- [ ] [hashbrown](https://docs.rs/hashbrown/latest/hashbrown/)
- [ ] [tracing](https://docs.rs/tracing/latest/tracing/)
  + [tracing_subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)

