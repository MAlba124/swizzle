use criterion::{criterion_group, criterion_main, Criterion};
use image_swizzle::{rgba_to_bgra, rgba_to_bgra_inplace};

fn generate_xxxx_image(width: usize, height: usize, x1: u8, x2: u8, x3: u8, x4: u8) -> Vec<u8> {
    assert!((width * height * 4) % 4 == 0);
    let mut xxxx = Vec::with_capacity(width * height);
    for _ in 0..width * height {
        xxxx.push(x1);
        xxxx.push(x2);
        xxxx.push(x3);
        xxxx.push(x4);
    }
    xxxx
}

fn criterion_benchmark(c: &mut Criterion) {
    {
        let rgba = generate_xxxx_image(4096, 4096, 0, 111, 222, 255);
        let mut bgra = vec![0; 4096 * 4096 * 4];
        c.bench_function("rgba_to_bgra", |b| b.iter(|| rgba_to_bgra(&rgba, &mut bgra)));
    }

    {
        let mut data = generate_xxxx_image(4096, 4096, 0, 111, 222, 255);
        c.bench_function("rgba_to_bgra_inplace", |b| b.iter(|| rgba_to_bgra_inplace(&mut data)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
