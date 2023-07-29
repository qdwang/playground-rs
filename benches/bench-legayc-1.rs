// use criterion::{criterion_group, criterion_main, Criterion};

// fn method1(w: u16, h: u16, data: &[u16]) -> Vec<u16> {
//     let w = w as usize;
//     let h = h as usize;
//     let get = |i: usize| unsafe { *data.get_unchecked(i) };

//     data.iter()
//         .enumerate()
//         .map(|(i, _)| {
//             let x = i % w;
//             let y = i / w;
//             let is_edge = y == 0 || y == h - 1 || x == 0 || x == w - 1;

//             if is_edge {
//                 0
//             } else {
//                 let i = i as usize;
//                 get(i - w - 1)
//                     + get(i - w)
//                     + get(i - w + 1)
//                     + get(i - 1)
//                     + get(i)
//                     + get(i + 1)
//                     + get(i + w - 1)
//                     + get(i + w)
//                     + get(i + w + 1)
//             }
//         })
//         .collect()
// }

// fn method2(w: u16, h: u16, data: &[u16]) -> Vec<u16> {
//     let w = w as usize;
//     let h = h as usize;
//     data.iter()
//         .enumerate()
//         .map(|(i, _)| {
//             let x = i % w;
//             let y = i / w;
//             let is_edge = y == 0 || y == h - 1 || x == 0 || x == w - 1;

//             if is_edge {
//                 0
//             } else {
//                 let i = i as usize;
//                 assert!(i < data.len());
//                 data[i - w - 1]
//                     + data[i - w]
//                     + data[i - w + 1]
//                     + data[i - 1]
//                     + data[i]
//                     + data[i + 1]
//                     + data[i + w - 1]
//                     + data[i + w]
//                     + data[i + w + 1]
//             }
//         })
//         .collect()
// }

// fn method3(w: u16, h: u16, data: &[u16]) -> Vec<u16> {
//     let w = w as usize;
//     let h = h as usize;
//     let get = |i: usize| {
//         assert!(i < data.len());
//         data[i]
//     };

//     data.iter()
//         .enumerate()
//         .map(|(i, _)| {
//             let x = i % w;
//             let y = i / w;
//             let is_edge = y == 0 || y == h - 1 || x == 0 || x == w - 1;

//             if is_edge {
//                 0
//             } else {
//                 let i = i as usize;
//                 get(i - w - 1)
//                     + get(i - w)
//                     + get(i - w + 1)
//                     + get(i - 1)
//                     + get(i)
//                     + get(i + 1)
//                     + get(i + w - 1)
//                     + get(i + w)
//                     + get(i + w + 1)
//             }
//         })
//         .collect()
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     let w = 300u16;
//     let h = 100u16;

//     let data: Vec<u16> = (0..w * h).collect();

//     c.bench_function("method1", |b| b.iter(|| method1(w, h, &data)));
//     c.bench_function("method2", |b| b.iter(|| method2(w, h, &data)));
//     c.bench_function("method3", |b| b.iter(|| method3(w, h, &data)));
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
