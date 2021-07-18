use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maximizer::line_superiority::{maximum_matching, StaticVec};

pub fn criterion_benchmark(c: &mut Criterion) {
    let neighbors: [StaticVec<4>; 4] = [
        StaticVec {
            len: 2,
            content: [0, 2, 0, 0],
        },
        StaticVec {
            len: 2,
            content: [0, 1, 0, 0],
        },
        StaticVec {
            len: 2,
            content: [3, 0, 0, 0],
        },
        StaticVec {
            len: 3,
            content: [0, 3, 2, 0],
        },
    ];

    c.bench_function("Maximum matching 4", |b| {
        b.iter(|| maximum_matching(black_box(&neighbors)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
