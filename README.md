[![Latest release](https://img.shields.io/crates/v/image_swizzle)](https://crates.io/crates/image_swizzle)
[![Docs](https://docs.rs/image_swizzle/badge.svg)](https://docs.rs/image_swizzle)

# Swizzle

SIMD accelerated image swizzling routines.

## Performance

#### RGBA to BGRA

Following shows benchmarks for frames of size 4096x4096.

```
test tests::bench_big_serial_rgba_to_bgra             ... bench:  31,355,763.50 ns/iter (+/- 1,238,430.78)
test tests::bench_big_vectorized_rgba_to_bgra         ... bench:  10,754,762.50 ns/iter (+/- 1,092,212.55) # 2.9x faster
test tests::bench_big_serial_rgba_to_bgra_inplace     ... bench:  29,412,045.30 ns/iter (+/- 1,112,664.50)
test tests::bench_big_vectorized_rgba_to_bgra_inplace ... bench:   8,075,968.10 ns/iter (+/- 874,224.50) # 3.6x faster
```

Following shows benchmarks for frames of size 4096x2160.

```
test tests::bench_serial_rgba_to_bgra                 ... bench:  16,567,283.20 ns/iter (+/- 1,388,114.34)
test tests::bench_vectorized_rgba_to_bgra             ... bench:   5,318,125.25 ns/iter (+/- 565,093.18) # 3.1x faster
test tests::bench_serial_rgba_to_bgra_inplace         ... bench:  15,619,490.90 ns/iter (+/- 631,683.88)
test tests::bench_vectorized_rgba_to_bgra_inplace     ... bench:   4,195,717.60 ns/iter (+/- 507,956.24) # 3.7x faster
```
