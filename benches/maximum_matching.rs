use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maximizer::line_superiority::{maximum_matching, maximum_matching_simple};

pub fn criterion_benchmark(c: &mut Criterion) {
    let neighbors: [&[usize]; 4] = [&[0, 2], &[0, 1], &[3, 0], &[0, 3, 2]];

    c.bench_function("Maximum matching 4", |b| {
        b.iter(|| maximum_matching(black_box(&neighbors)))
    });
    c.bench_function("Maximum matching simple 4", |b| {
        b.iter(|| maximum_matching_simple(black_box(&neighbors)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
