use criterion::{criterion_group, criterion_main, Criterion};
use absagl::groups::{FiniteGroup, modulo::Modulo, Additive, Group};

fn bench_is_closed(c: &mut Criterion) {
    let n = 500;
    let elements = Modulo::<Additive>::generate_group(n).unwrap();
    let group = FiniteGroup::new(elements);

    let mut config = Criterion::default()
        .sample_size(10) // Only one sample
        .measurement_time(std::time::Duration::from_secs(1)); // Minimal measurement time

    config.bench_function("is_closed", |b| b.iter(|| group.is_closed()));
    config.bench_function("is_closed_parallel", |b| b.iter(|| group.is_closed_parallel()));
}

criterion_group!(benches, bench_is_closed);
criterion_main!(benches);