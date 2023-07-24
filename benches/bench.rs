use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wide::*;

fn sum1(x: &[u16]) -> u128 {
    x.into_iter().map(|x| *x as u128).sum::<u128>()
}
fn sum2(x: &[u16]) -> u128 {
    x.chunks_exact(4)
        .map(|arr| {
            let v = [arr[0] as u32, arr[1] as u32, arr[2] as u32, arr[3] as u32];
            u32x4::from(v)
        })
        .fold(u32x4::ZERO, |acc, x| acc + x)
        .as_array_ref()
        .map(|x| x as u128)
        .into_iter()
        .sum::<u128>()
}

fn criterion_benchmark(c: &mut Criterion) {
    let data: Vec<u16> = (0..4 * 10213u16).collect();

    c.bench_function("sum1", |b| b.iter(|| sum1(black_box(&data))));
    c.bench_function("sum2", |b| b.iter(|| sum2(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
