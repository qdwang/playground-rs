use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn f1(a: i32) -> i32 {
    a.clamp(0, 65535)
}
fn f2(a: i32) -> i32 {
    std::cmp::min(a, 65535)
}
fn f3(x : i32) -> i32 {
    ((65535 - x) >> 31 | x) & 65535
}
fn f4(x : i32) -> i32 {
    if x & !0xffff != 0 {
        (!x >> 31) & 0xffff
    } else {
        x
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let data = 523233i32;

    c.bench_function("f1", |b| b.iter(|| f1(black_box(data))));
    c.bench_function("f2", |b| b.iter(|| f2(black_box(data))));
    c.bench_function("f3", |b| b.iter(|| f3(black_box(data))));
    c.bench_function("f4", |b| b.iter(|| f4(black_box(data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
