use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read;
use std::time::Duration;

fn host_callback(
    _id: u64,
    _bd: &str,
    _ns: &str,
    _op: &str,
    _payload: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(vec![])
}

pub fn bench_new_code(c: &mut Criterion) {
    use wapc_1_3 as wapc;
    use wasmtime_provider_1_3 as wasmtime_provider;
    use wasmtime_provider_1_3::wasmtime;

    let module_bytes = read("policy.wasm").expect("Cannot read module from disk");

    let engine = wasmtime::Engine::default();
    let builder = wasmtime_provider::WasmtimeEngineProviderBuilder::new()
        .module_bytes(&module_bytes)
        .engine(engine);
    let provider = builder
        .build()
        .expect("Cannot create WasmtimeEngineProviderPre");

    let mut group = c.benchmark_group("new code");

    group.bench_function("WasmtimeEngineProvider::clone", |b| {
        b.iter(|| {
            let host =
                wapc::WapcHost::new(Box::new(provider.clone()), Some(Box::new(host_callback)))
                    .expect("cannot create wapc host");
            let _ = host
                .call(black_box("validate_settings"), black_box(b"{}"))
                .expect("error invoking guest");
        })
    });

    group.finish();
}

pub fn bench_old_code(c: &mut Criterion) {
    use wapc_1_0 as wapc;
    use wasmtime_provider_1_2 as wasmtime_provider;
    use wasmtime_provider_1_2::wasmtime;

    let module_bytes = read("policy.wasm").expect("Cannot read module from disk");

    let engine = wasmtime::Engine::default();
    let builder =
        wasmtime_provider::WasmtimeEngineProviderBuilder::new(&module_bytes).engine(engine);
    let provider = builder
        .build()
        .expect("Cannot create WasmtimeEngineProviderPre");

    let mut group = c.benchmark_group("old code");

    // this code is slower, it needs more evaluation time
    group.warm_up_time(Duration::from_secs(10));
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("WasmtimeEngineProvider::clone", |b| {
        b.iter(|| {
            let host =
                wapc::WapcHost::new(Box::new(provider.clone()), Some(Box::new(host_callback)))
                    .expect("cannot create wapc host");
            let _ = host
                .call(black_box("validate_settings"), black_box(b"{}"))
                .expect("error invoking guest");
        })
    });

    group.finish();
}

criterion_group!(benches, bench_new_code, bench_old_code);
criterion_main!(benches);
