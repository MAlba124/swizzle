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
macro_rules! idx_order {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        [
            $a + (4 * 0), $b + (4 * 0), $c + (4 * 0), $d + (4 * 0),
            $a + (4 * 1), $b + (4 * 1), $c + (4 * 1), $d + (4 * 1),
            $a + (4 * 2), $b + (4 * 2), $c + (4 * 2), $d + (4 * 2),
            $a + (4 * 3), $b + (4 * 3), $c + (4 * 3), $d + (4 * 3),

            $a + (4 * 4), $b + (4 * 4), $c + (4 * 4), $d + (4 * 4),
            $a + (4 * 5), $b + (4 * 5), $c + (4 * 5), $d + (4 * 5),
            $a + (4 * 6), $b + (4 * 6), $c + (4 * 6), $d + (4 * 6),
            $a + (4 * 7), $b + (4 * 7), $c + (4 * 7), $d + (4 * 7),

            $a + (4 * 8), $b + (4 * 8), $c + (4 * 8), $d + (4 * 8),
            $a + (4 * 9), $b + (4 * 9), $c + (4 * 9), $d + (4 * 9),
            $a + (4 * 10), $b + (4 * 10), $c + (4 * 10), $d + (4 * 10),
            $a + (4 * 11), $b + (4 * 11), $c + (4 * 11), $d + (4 * 11),

            $a + (4 * 12), $b + (4 * 12), $c + (4 * 12), $d + (4 * 12),
            $a + (4 * 13), $b + (4 * 13), $c + (4 * 13), $d + (4 * 13),
            $a + (4 * 14), $b + (4 * 14), $c + (4 * 14), $d + (4 * 14),
            $a + (4 * 15), $b + (4 * 15), $c + (4 * 15), $d + (4 * 15),
        ]
    }
}

const RGBA_TO_BGRA_SWIZZLE_IDXS: [usize; VECTOR_WIDTH * 4] = idx_order!(2, 1, 0, 3);
const BGRA_TO_RGBA_SWIZZLE_IDXS: [usize; VECTOR_WIDTH * 4] = idx_order!(2, 1, 0, 3);

macro_rules! swizzle_4_wide_inline {
    ($src:expr, $idxs:expr) => {
        assert!($src.len() % 4 == 0);

        let n_pixels = $src.len() / 4;

        let mut i = 0;
        while i + VECTOR_WIDTH < n_pixels {
            simd_swizzle!(
                u8x64::from_slice(&$src[i * 4..(i + VECTOR_WIDTH) * 4]),
                $idxs
            )
            .copy_to_slice(&mut $src[i * 4..(i + VECTOR_WIDTH) * 4]);
            i += VECTOR_WIDTH;
        }

        for j in i..n_pixels {
            let (r, g, b, a) = (
                $src[j * 4 + 0],
                $src[j * 4 + 1],
                $src[j * 4 + 2],
                $src[j * 4 + 3],
            );
            $src[j * 4 + 0] = b;
            $src[j * 4 + 1] = g;
            $src[j * 4 + 2] = r;
            $src[j * 4 + 3] = a;
        }
    };
}

macro_rules! swizzle_4_wide {
    ($src:expr, $dst:expr, $idxs:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len());

        let n_pixels = $src.len() / 4;

        let mut i = 0;
        while i + VECTOR_WIDTH < n_pixels {
            simd_swizzle!(
                u8x64::from_slice(&$src[i * 4..(i + VECTOR_WIDTH) * 4]),
                $idxs
            )
            .copy_to_slice(&mut $dst[i * 4..(i + VECTOR_WIDTH) * 4]);
            i += VECTOR_WIDTH;
        }

        for j in i..n_pixels {
            $dst[j * 4 + 0] = $src[j * 4 + 2];
            $dst[j * 4 + 1] = $src[j * 4 + 1];
            $dst[j * 4 + 2] = $src[j * 4 + 0];
            $dst[j * 4 + 3] = $src[j * 4 + 3];
        }
    };
}

pub fn rgba_to_bgra_inline(src: &mut [u8]) {
    swizzle_4_wide_inline!(src, RGBA_TO_BGRA_SWIZZLE_IDXS);
}

pub fn rgba_to_bgra(src: &[u8], dst: &mut [u8]) {
    swizzle_4_wide!(src, dst, RGBA_TO_BGRA_SWIZZLE_IDXS);
}

pub fn bgra_to_rgba_inline(src: &mut [u8]) {
    swizzle_4_wide_inline!(src, BGRA_TO_RGBA_SWIZZLE_IDXS);
}

pub fn bgra_to_rgba(src: &[u8], dst: &mut [u8]) {
    swizzle_4_wide!(src, dst, BGRA_TO_RGBA_SWIZZLE_IDXS);
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
        for _ in 0..width * heigh {
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
        let rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        let mut bgra = vec![0; width * height * 4];
        rgba_to_bgra(&rgba_img, &mut bgra);
        assert_eq!(bgra, correct_bgra);
    }

    #[test]
    fn test_bgra_to_rgba_inline() {
        let (width, height) = (1920, 1080);
        let mut bgra = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_rgba = generate_xxxx_image(width, height, 100, 222, 111, 255);
        bgra_to_rgba_inline(&mut bgra);
        assert_eq!(bgra, correct_rgba);
    }

    #[test]
    fn test_bgra_to_rgba() {
        let (width, height) = (1920, 1080);
        let bgra = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_rgba = generate_xxxx_image(width, height, 100, 222, 111, 255);
        let mut rgba = vec![0; width * height * 4];
        bgra_to_rgba(&bgra, &mut rgba);
        assert_eq!(rgba, correct_rgba);
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
