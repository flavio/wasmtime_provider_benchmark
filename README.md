A benchmark comparing the performance of `WasmtimeProvider::clone` implementations

The benchmark is defined inside of `./benches/benchmarks.rs`.

The benchmarks depends on the `policy.wasm` file. This is a Kubewarden policy written using
`TinyGo`. The big size of the WebAssembly module makes the results of the benchmarks
more meaningful.

## Running the tests

The tests can be run with the following command:

```console
cargo bench
```

The output will be similar to the following one:

```console
$ cargo bench
   Compiling wasmtime_provider_bench v0.1.0 (/home/flavio/hacking/wasm/wapc/wasmtime_provider_bench_up)
    Finished bench [optimized] target(s) in 13.36s
     Running unittests src/lib.rs (target/release/deps/wasmtime_provider_bench-ba3219151993e0f0)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/benchmarks.rs (target/release/deps/benchmarks-96c0e843948a55cc)
new code/WasmtimeEngineProvider::clone
                        time:   [433.70 µs 434.79 µs 436.16 µs]
                        change: [-5.2990% -4.2338% -3.3898%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

old code/WasmtimeEngineProvider::clone
                        time:   [1.5178 ms 1.5198 ms 1.5222 ms]
                        change: [-6.7634% -6.4639% -6.2335%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
```
