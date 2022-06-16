use criterion::{black_box, criterion_group, criterion_main, Criterion};
use exer::search::binary_search;

fn benchmark(c: &mut Criterion) {
    let mut arr = black_box([12, 22, 33, 54, 76, 89, 99]);
    c.bench_function("bencher", |b| {
        b.iter(|| binary_search::_binary_search(&mut arr, 99))
    });
}

criterion_group!(bencher, benchmark);
criterion_main!(bencher);
