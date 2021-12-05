![Rust](https://github.com/stesel/perf_meter/workflows/Rust/badge.svg?branch=master)
# perf_meter

## Performance measure for 🦀

## Example

```rust
fn usage() {
    let perf_meter = PerfMeter::new(1);
    perf_meter.tick(); // Call on each update
}
/// After interval secs it prints out:
/// "performed {x} ops per {y} sec"
```
