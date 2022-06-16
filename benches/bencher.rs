use criterion::{black_box, criterion_group, criterion_main, Criterion};
use exer::search;

fn benchmark(c: &mut Criterion) {
    let mut arr = black_box([
        12, 22, 33, 54, 76, 89, 99, 111, 223, 434, 546, 678, 679, 954, 999, 1000, 1111, 1222, 1333,
        1444,
    ]);
    c.bench_function("linear_search", |b| {
        b.iter(|| search::linear_search::linear_search(&mut arr, &76))
    });
    c.bench_function("binary_search", |b| {
        b.iter(|| search::binary_search::_binary_search(&mut arr, 54))
    });
}

criterion_group!(bencher, benchmark);
criterion_main!(bencher);
