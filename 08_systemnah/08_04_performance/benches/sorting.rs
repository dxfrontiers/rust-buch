use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_buch_08_04_performance::{sort_recursive, generate_custom_data, sort_iterative};

criterion_group!(benches, bench_sorting);
criterion_main!(benches);

fn bench_sorting(c: &mut Criterion) {

    let mut group = c.benchmark_group("sort_array");
    for i in 0..4 {

        group.bench_with_input(BenchmarkId::new("recursive", i), &i, |b, i| {
            let mut data = generate_custom_data(18+*i);
            b.iter(|| sort_recursive(&mut data));
        });

        group.bench_with_input(BenchmarkId::new("iterative", i), &i, |b, i| {
            let mut data = generate_custom_data(18+*i);
            b.iter(|| sort_iterative(&mut data));
        });
    }
}
