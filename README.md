# Rust Async Framework

## Tools

* `lscpu -C` can show `COHERENCY-SIZE` as the "minimum amount of data in bytes transferred from memory to cache".
* Can show thread names in htop by F2 → Display options → Show custom thread names
  + Can manually set thread priority (on non Apple Silicon devices) using [`core_affinity` crate](https://docs.rs/core_affinity/latest/core_affinity/).

## Benchmarking

Uses [Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html) for [`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html). Mainly uses `gnuplot` to [generate benchmark plots](https://bheisler.github.io/criterion.rs/book/user_guide/plots_and_graphs.html).

## TODO

- [ ] Commpare [bounded-spsc-queue](Start with https://github.com/polyfractal/bounded-spsc-queue (look at recent PRs to get building),) with [crossbeam](https://github.com/crossbeam-rs/crossbeam) primitives/performance

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

