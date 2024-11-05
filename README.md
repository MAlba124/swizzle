# Swizzle

SIMD accelerated image swizzling routines.

## Performance

#### RGBA to BGRA

```
test tests::bench_serial_rgba_to_bgra     ... bench:  16,276,480.60 ns/iter (+/- 7,499,036.91)
test tests::bench_vectorized_rgba_to_bgra ... bench:   5,451,023.30 ns/iter (+/- 584,312.97)
```

On average the verctorized version is 3 times faster than the non vectorized one.
