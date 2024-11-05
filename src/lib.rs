// Copyright C 2024 Marcus Lian Hanestad <marlhan@proton.me>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the “Software”), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(portable_simd)]
#![feature(test)]

extern crate test;

use std::simd::{simd_swizzle, u8x64};

const VECTOR_WIDTH: usize = 16;

#[rustfmt::skip]
const RGBA_TO_BGRA_SWIZZLE_IDXS: [usize; VECTOR_WIDTH * 4] = [
    2 + (4 * 0), 1 + (4 * 0), 0 + (4 * 0), 3 + (4 * 0),
    2 + (4 * 1), 1 + (4 * 1), 0 + (4 * 1), 3 + (4 * 1),
    2 + (4 * 2), 1 + (4 * 2), 0 + (4 * 2), 3 + (4 * 2),
    2 + (4 * 3), 1 + (4 * 3), 0 + (4 * 3), 3 + (4 * 3),

    2 + (4 * 4), 1 + (4 * 4), 0 + (4 * 4), 3 + (4 * 4),
    2 + (4 * 5), 1 + (4 * 5), 0 + (4 * 5), 3 + (4 * 5),
    2 + (4 * 6), 1 + (4 * 6), 0 + (4 * 6), 3 + (4 * 6),
    2 + (4 * 7), 1 + (4 * 7), 0 + (4 * 7), 3 + (4 * 7),

    2 + (4 * 8), 1 + (4 * 8), 0 + (4 * 8), 3 + (4 * 8),
    2 + (4 * 9), 1 + (4 * 9), 0 + (4 * 9), 3 + (4 * 9),
    2 + (4 * 10), 1 + (4 * 10), 0 + (4 * 10), 3 + (4 * 10),
    2 + (4 * 11), 1 + (4 * 11), 0 + (4 * 11), 3 + (4 * 11),

    2 + (4 * 12), 1 + (4 * 12), 0 + (4 * 12), 3 + (4 * 12),
    2 + (4 * 13), 1 + (4 * 13), 0 + (4 * 13), 3 + (4 * 13),
    2 + (4 * 14), 1 + (4 * 14), 0 + (4 * 14), 3 + (4 * 14),
    2 + (4 * 15), 1 + (4 * 15), 0 + (4 * 15), 3 + (4 * 15),
];

pub fn rgba_to_bgra_inline(src: &mut [u8]) {
    assert!(src.len() % 4 == 0);

    let n_pixels = src.len() / 4;

    let mut i = 0;
    while i + VECTOR_WIDTH < n_pixels {
        simd_swizzle!(
            u8x64::from_slice(&src[i * 4..(i + VECTOR_WIDTH) * 4]),
            RGBA_TO_BGRA_SWIZZLE_IDXS
        )
        .copy_to_slice(&mut src[i * 4..(i + VECTOR_WIDTH) * 4]);
        i += VECTOR_WIDTH;
    }

    for j in i..n_pixels {
        let (r, g, b, a) = (
            src[j * 4 + 0],
            src[j * 4 + 1],
            src[j * 4 + 2],
            src[j * 4 + 3],
        );
        src[j * 4 + 0] = b;
        src[j * 4 + 1] = g;
        src[j * 4 + 2] = r;
        src[j * 4 + 3] = a;
    }
}

pub fn rgba_to_bgra(src: &[u8], dst: &mut [u8]) {
    assert!(src.len() % 4 == 0 && src.len() == dst.len());

    let n_pixels = src.len() / 4;

    let mut i = 0;
    while i + VECTOR_WIDTH < n_pixels {
        simd_swizzle!(
            u8x64::from_slice(&src[i * 4..(i + VECTOR_WIDTH) * 4]),
            RGBA_TO_BGRA_SWIZZLE_IDXS
        )
        .copy_to_slice(&mut dst[i * 4..(i + VECTOR_WIDTH) * 4]);
        i += VECTOR_WIDTH;
    }

    for j in i..n_pixels {
        dst[j * 4 + 0] = src[j * 4 + 2];
        dst[j * 4 + 1] = src[j * 4 + 1];
        dst[j * 4 + 2] = src[j * 4 + 0];
        dst[j * 4 + 3] = src[j * 4 + 3];
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    fn scalar_rgba_to_bgra(rgba: &[u8], bgra: &mut [u8]) {
        assert!(rgba.len() % 4 == 0 && rgba.len() == bgra.len());
        for i in (0..rgba.len()).step_by(4) {
            bgra[i + 0] = rgba[i + 2];
            bgra[i + 1] = rgba[i + 1];
            bgra[i + 2] = rgba[i + 0];
            bgra[i + 3] = rgba[i + 3];
        }
    }

    fn generate_xxxx_image(width: usize, heigh: usize, x1: u8, x2: u8, x3: u8, x4: u8) -> Vec<u8> {
        assert!((width * heigh) % 4 == 0);
        let mut xxxx = Vec::with_capacity(width * heigh);
        for i in 0..width * heigh {
            xxxx.push(x1);
            xxxx.push(x2);
            xxxx.push(x3);
            xxxx.push(x4);
        }
        xxxx
    }

    #[test]
    fn test_rgba_to_bgra_inline() {
        let (width, height) = (1920, 1080);
        let mut rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        rgba_to_bgra_inline(&mut rgba_img);
        assert_eq!(rgba_img, correct_bgra);
    }

    #[test]
    fn test_rgba_to_bgra() {
        let (width, height) = (1920, 1080);
        let mut rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        let mut bgra = vec![0; width * height * 4];
        rgba_to_bgra(&rgba_img, &mut bgra);
        assert_eq!(bgra, correct_bgra);
    }

    #[bench]
    fn bench_scalar_rgba_to_bgra(b: &mut Bencher) {
        let (width, height) = (4096, 2160);
        let rgba = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let mut bgra = vec![0; width * height * 4];
        b.iter(|| {
            scalar_rgba_to_bgra(&rgba, &mut bgra);
        });
    }

    #[bench]
    fn bench_vector_rgba_to_bgra(b: &mut Bencher) {
        let (width, height) = (4096, 2160);
        let rgba = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let mut bgra = vec![0; width * height * 4];
        b.iter(|| {
            rgba_to_bgra(&rgba, &mut bgra);
        });
    }
}
