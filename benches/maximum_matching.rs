use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maximizer::line_superiority::{maximum_matching, maximum_matching_simple};

pub fn maximum_matching_4(c: &mut Criterion) {
    let neighbors: [&[usize]; 4] = [&[0, 2], &[0, 1], &[3, 0], &[0, 3, 2]];

    c.bench_function("Maximum matching 4", |b| {
        b.iter(|| maximum_matching(black_box(&neighbors)))
    });
    c.bench_function("Maximum matching simple 4", |b| {
        b.iter(|| maximum_matching_simple(black_box(&neighbors)))
    });
}

pub fn maximum_matching_9(c: &mut Criterion) {
    let neighbors: [&[usize]; 9] = [
        &[1, 4, 5, 6, 8],
        &[5, 6, 7],
        &[0, 3, 4, 5, 6, 7, 8],
        &[2, 3, 4, 7],
        &[0, 1, 2, 5, 8],
        &[2, 4, 5, 6],
        &[0, 1, 2, 4, 5, 6, 7, 8],
        &[0, 1, 2, 5, 7, 8],
        &[0, 1, 2, 3, 7],
    ];

    c.bench_function("Maximum matching 9", |b| {
        b.iter(|| maximum_matching(black_box(&neighbors)))
    });
    c.bench_function("Maximum matching simple 9", |b| {
        b.iter(|| maximum_matching_simple(black_box(&neighbors)))
    });
}

criterion_group!(benches, maximum_matching_4, maximum_matching_9);
criterion_main!(benches);
