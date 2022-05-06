#[macro_use]
extern crate log;

use criterion::{criterion_group, criterion_main, Criterion};
use env_logger::Env;

fn logit()  {
    for _i in 0 .. 1_000_000 {
        trace!("log trace");
        debug!("log debug");
        info!("log info");
        warn!("log warn");
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    env_logger::from_env(Env::default().default_filter_or("error")).init();
    c.bench_function("level_error", |b| b.iter(|| logit()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);