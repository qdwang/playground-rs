use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

fn iter_fn<'a, T: Iterator<Item = &'a u8>>(iter: T) -> u64 {
    iter.fold(0u64, |acc, x| acc + *x as u64)
}
fn index_fn(data: &[u8]) -> u64 {
    let mut sum = 0u64;
    for x in data {
        sum += *x as u64;
    }
    sum
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let data: Vec<u8> = (0..1024 * 1024).map(|_| rng.gen()).collect();
    c.bench_function("Iterator", |b| b.iter(|| iter_fn(data.iter())));
    c.bench_function("Index", |b| b.iter(|| index_fn(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
