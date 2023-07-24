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

fn main() {
    let data: Vec<u16> = (0..4u16 * 500).collect();

    let s1 = sum1(&data);
    let s2 = sum2(&data);

    println!("{:?} {:?}", s1, s2);
}
