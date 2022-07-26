use criterion::{black_box, criterion_group, criterion_main, Criterion};
use study_rust::sort::{sort_arr, sort_arr_2};

fn sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box([6, 2, 4, 1, 9, -2, 5]);

    c.bench_function("sorting algorithm", |b| b.iter(|| sort_arr(&mut arr)));
}

fn sort_arr_benchmark2(c: &mut Criterion) {
    let mut arr = black_box([6, 2, 4, 1, 9, -2, 5]);

    c.bench_function("sorting algorithm2", |b| b.iter(|| sort_arr_2(&mut arr)));
}

criterion_group!(benches, sort_arr_benchmark, sort_arr_benchmark2);
criterion_main!(benches);
