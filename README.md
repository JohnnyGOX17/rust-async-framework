# Rust Async Framework

## Benchmarking

Uses [Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html) for [`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html). Mainly uses `gnuplot` to [generate benchmark plots](https://bheisler.github.io/criterion.rs/book/user_guide/plots_and_graphs.html).

## TODO

- [ ] Commpare [bounded-spsc-queue](Start with https://github.com/polyfractal/bounded-spsc-queue (look at recent PRs to get building),) with [crossbeam](https://github.com/crossbeam-rs/crossbeam) primitives/performance

## Repos/References to use and try

- [ ] [Rust `std` library](https://doc.rust-lang.org/std/index.html)
- [ ] [Thread pool - Wikipedia](https://en.wikipedia.org/wiki/Thread_pool)
- [ ] [disruptor-rs](https://github.com/nicholassm/disruptor-rs)
  + https://github.com/coralblocks/CoralRing
  + https://github.com/FutureSDR/FutureSDR
  + https://github.com/eclipse-iceoryx/iceoryx2
  + https://github.com/dagrs-dev/dagrs
- [ ] [Crust of Rust: Atomics and Memory Ordering](https://youtu.be/rMGWeSjctlY?si=p9_oZkq_Eo6B0bNV)
- [ ] [A Bounded SPSC queue for Rust](https://github.com/JohnnyGOX17/bounded-spsc-queue)
- [ ] [Why is ringbuf crate so fast?- Reddit](https://www.reddit.com/r/rust/comments/1h3bqv0/why_is_ringbuf_crate_so_fast/)
- [ ] Can manually set thread priority (on non Apple Silicon devices) using [`core_affinity` crate](https://docs.rs/core_affinity/latest/core_affinity/)
- [ ] [crossbeam](https://github.com/crossbeam-rs/crossbeam)
- [ ] [hashbrown](https://docs.rs/hashbrown/latest/hashbrown/)
- [ ] [tracing](https://docs.rs/tracing/latest/tracing/)

