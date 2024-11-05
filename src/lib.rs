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

//! SIMD accelerated image swizzling routines

// TODO: Do conversion depending on system endianess

#![feature(portable_simd)]
#![feature(test)]

extern crate test;

use std::simd::{self, simd_swizzle, u8x64};

const VECTOR_WIDTH: usize = 16 * 4;

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

#[rustfmt::skip]
macro_rules! xxx0_to_xxxx_mask {
    () => {
        simd::Mask::from_array([
            true, true, true, false,
            true, true, true, false,
            true, true, true, false,
            true, true, true, false,

            true, true, true, false,
            true, true, true, false,
            true, true, true, false,
            true, true, true, false,

            true, true, true, false,
            true, true, true, false,
            true, true, true, false,
            true, true, true, false,

            true, true, true, false,
            true, true, true, false,
            true, true, true, false,
            true, true, true, false,
        ])
    }
}

const RGBA_TO_BGRA_SWIZZLE_IDXS: [usize; VECTOR_WIDTH] = idx_order!(2, 1, 0, 3);
const BGRA_TO_RGBA_SWIZZLE_IDXS: [usize; VECTOR_WIDTH] = idx_order!(2, 1, 0, 3);
#[rustfmt::skip]
const XXX0_TO_XXXX_OR: u8x64 = u8x64::from_array([
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,

    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,

    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,

    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
]);

macro_rules! swizzle_4_wide {
    ($src:expr, $dst:expr, $idxs:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len());

        let end = ($src.len() / VECTOR_WIDTH ) * VECTOR_WIDTH;
        (0..end).step_by(VECTOR_WIDTH).for_each(|i| {
            simd_swizzle!(
                u8x64::from_slice(&$src[i..i + VECTOR_WIDTH]),
                $idxs
            )
            .copy_to_slice(&mut $dst[i..i + VECTOR_WIDTH]);
        });

        (end..$src.len()).step_by(4).for_each(|i| {
            let (a, b, c, d) = (
                $src[i + 0],
                $src[i + 1],
                $src[i + 2],
                $src[i + 3],
            );
            $dst[i + $idxs[0]] = a;
            $dst[i + $idxs[1]] = b;
            $dst[i + $idxs[2]] = c;
            $dst[i + $idxs[3]] = d;
        });
    };
}

macro_rules! apply_mask_4_wide {
    ($src:expr, $dst:expr, $mask:expr, $or:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len());

        let end = ($src.len() / VECTOR_WIDTH) * VECTOR_WIDTH;
        (0..end).step_by(VECTOR_WIDTH).for_each(|i| {
            u8x64::load_select(&$src[i..i + VECTOR_WIDTH], $mask, XXX0_TO_XXXX_OR)
                .copy_to_slice(&mut $dst[i..i + VECTOR_WIDTH]);
        });

        (end..$src.len()).step_by(4).for_each(|i| {
            let (a, b, c) = (
                $src[i + 0],
                $src[i + 1],
                $src[i + 2],
            );
            $dst[i + 0] = a;
            $dst[i + 1] = b;
            $dst[i + 2] = c;
            $dst[i + 3] = 255;
        });
    };
}

macro_rules! apply_x_mask_and_swizzle_4_wide {
    ($src:expr, $dst:expr, $or:expr, $idxs:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len());

        let mask = xxx0_to_xxxx_mask!();
        let end = ($src.len() / VECTOR_WIDTH) * VECTOR_WIDTH;
        (0..end).step_by(VECTOR_WIDTH).for_each(|i| {
            simd_swizzle!(
                u8x64::load_select(&$src[i..i + VECTOR_WIDTH], mask, XXX0_TO_XXXX_OR),
                $idxs
            )
            .copy_to_slice(&mut $dst[i..i + VECTOR_WIDTH]);
        });

        (end..$src.len()).step_by(4).for_each(|i| {
            let (a, b, c) = ($src[i + 0], $src[i + 1], $src[i + 2]);
            $dst[i + $idxs[0]] = a;
            $dst[i + $idxs[1]] = b;
            $dst[i + $idxs[2]] = c;
            $dst[i + $idxs[3]] = 255;
        });
    };
}

#[allow(dead_code)]
#[inline]
fn serial_swizzle_4_wide_inplace(src: &mut [u8], idxs: &[usize; 4]) {
    assert!(src.len() % 4 == 0);

    (0..src.len()).step_by(4).for_each(|i| {
        let (a, b, c, d) = (src[i], src[i + 1], src[i + 2], src[i + 3]);
        src[i + idxs[0]] = a;
        src[i + idxs[1]] = b;
        src[i + idxs[2]] = c;
        src[i + idxs[3]] = d;
    });
}

#[allow(dead_code)]
#[inline]
fn serial_swizzle_4_wide(src: &[u8], dst: &mut [u8], idxs: &[usize; 4]) {
    assert!(src.len() % 4 == 0);

    (0..src.len()).step_by(4).for_each(|i| {
        dst[i + idxs[0]] = src[i];
        dst[i + idxs[1]] = src[i + 1];
        dst[i + idxs[2]] = src[i + 2];
        dst[i + idxs[3]] = src[i + 3];
    });
}

/// Convert RGBA data to BGRA while overwriting the old RGBA data in `src`.
///
/// ```rust
/// use image_swizzle::rgba_to_bgra_inplace;
/// let mut rgba = [1, 2, 3, 255];
/// rgba_to_bgra_inplace(&mut rgba);
/// assert_eq!(rgba, [3, 2, 1, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4.
#[inline]
pub fn rgba_to_bgra_inplace(src: &mut [u8]) {
    swizzle_4_wide!(src, src, RGBA_TO_BGRA_SWIZZLE_IDXS);
}

/// Convert RGBA data to BGRA and store the result to `dst`.
///
/// ```rust
/// use image_swizzle::rgba_to_bgra;
/// let rgba = [1, 2, 3, 255];
/// let mut bgra = [0; 4];
/// rgba_to_bgra(&rgba, &mut bgra);
/// assert_eq!(bgra, [3, 2, 1, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4 or if `dst.len` is not equal to `src.len`.
#[inline]
pub fn rgba_to_bgra(src: &[u8], dst: &mut [u8]) {
    swizzle_4_wide!(src, dst, RGBA_TO_BGRA_SWIZZLE_IDXS);
}

/// Convert BGRA data to RGBA while overwriting the old BGRA data in `src`.
///
/// ```rust
/// use image_swizzle::bgra_to_rgba_inplace;
/// let mut bgra = [3, 2, 1, 255];
/// bgra_to_rgba_inplace(&mut bgra);
/// assert_eq!(bgra, [1, 2, 3, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4.
#[inline]
pub fn bgra_to_rgba_inplace(src: &mut [u8]) {
    swizzle_4_wide!(src, src, BGRA_TO_RGBA_SWIZZLE_IDXS);
}

/// Convert BGRA data to RGBA and store the result to `dst`.
///
/// ```rust
/// use image_swizzle::bgra_to_rgba;
/// let bgra = [3, 2, 1, 255];
/// let mut rgba = [0; 4];
/// bgra_to_rgba(&bgra, &mut rgba);
/// assert_eq!(rgba, [1, 2, 3, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4 or if `dst.len` is not equal to `src.len`.
#[inline]
pub fn bgra_to_rgba(src: &[u8], dst: &mut [u8]) {
    swizzle_4_wide!(src, dst, BGRA_TO_RGBA_SWIZZLE_IDXS);
}

/// Convert RGB0 data to RGBX while overwriting the old RGB0 data in `src`.
///
/// ```rust
/// use image_swizzle::rgb0_to_rgbx_inplace;
/// let mut rgb0 = [1, 2, 3, 0];
/// rgb0_to_rgbx_inplace(&mut rgb0);
/// assert_eq!(rgb0, [1, 2, 3, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4.
#[inline]
pub fn rgb0_to_rgbx_inplace(src: &mut [u8]) {
    apply_mask_4_wide!(src, src, xxx0_to_xxxx_mask!(), XXX0_TO_XXXX_OR);
}

/// Convert RGB0 data to RGBX and store the result to `dst`.
///
/// ```rust
/// use image_swizzle::rgb0_to_rgbx;
/// let rgb0 = [1, 2, 3, 0];
/// let mut rgbx = [0; 4];
/// rgb0_to_rgbx(&rgb0, &mut rgbx);
/// assert_eq!(rgbx, [1, 2, 3, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4 or if `dst.len` is not equal to `src.len`.
#[inline]
pub fn rgb0_to_rgbx(src: &[u8], dst: &mut [u8]) {
    apply_mask_4_wide!(src, dst, xxx0_to_xxxx_mask!(), XXX0_TO_XXXX_OR);
}

/// Convert BGR0 data to BGRX while overwriting the old BGR0 data in `src`.
///
/// ```rust
/// use image_swizzle::bgr0_to_bgrx_inplace;
/// let mut bgr0 = [3, 2, 1, 0];
/// bgr0_to_bgrx_inplace(&mut bgr0);
/// assert_eq!(bgr0, [3, 2, 1, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4.
#[inline]
pub fn bgr0_to_bgrx_inplace(src: &mut [u8]) {
    apply_mask_4_wide!(src, src, xxx0_to_xxxx_mask!(), XXX0_TO_XXXX_OR);
}

/// Convert BGR0 data to BGRX and store the result to `dst`.
///
/// ```rust
/// use image_swizzle::bgr0_to_bgrx;
/// let bgr0 = [3, 2, 1, 0];
/// let mut bgrx = [0; 4];
/// bgr0_to_bgrx(&bgr0, &mut bgrx);
/// assert_eq!(bgrx, [3, 2, 1, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4 or if `dst.len` is not equal to `src.len`.
#[inline]
pub fn bgr0_to_bgrx(src: &[u8], dst: &mut [u8]) {
    apply_mask_4_wide!(src, dst, xxx0_to_xxxx_mask!(), XXX0_TO_XXXX_OR);
}

/// Convert RGB0 data to BGRX while overwriting the old RGB0 data in `src`.
///
/// ```rust
/// use image_swizzle::rgb0_to_bgrx_inplace;
/// let mut rgb0 = [1, 2, 3, 0];
/// rgb0_to_bgrx_inplace(&mut rgb0);
/// assert_eq!(rgb0, [3, 2, 1, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4.
#[inline]
pub fn rgb0_to_bgrx_inplace(src: &mut [u8]) {
    apply_x_mask_and_swizzle_4_wide!(src, src, XXX0_TO_XXX_OR, RGBA_TO_BGRA_SWIZZLE_IDXS);
}

/// Convert RGB0 data to BGRX and store the result to `dst`.
///
/// ```rust
/// use image_swizzle::rgb0_to_bgrx;
/// let rgb0 = [1, 2, 3, 0];
/// let mut bgrx = [0; 4];
/// rgb0_to_bgrx(&rgb0, &mut bgrx);
/// assert_eq!(bgrx, [3, 2, 1, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4 or if `dst.len` is not equal to `src.len`.
#[inline]
pub fn rgb0_to_bgrx(src: &[u8], dst: &mut [u8]) {
    apply_x_mask_and_swizzle_4_wide!(src, dst, XXX0_TO_XXXX_OR, RGBA_TO_BGRA_SWIZZLE_IDXS);
}

/// Convert BGR0 data to RGBX while overwriting the old BGR0 data in `src`.
///
/// ```rust
/// use image_swizzle::bgr0_to_rgbx_inplace;
/// let mut bgr0 = [3, 2, 1, 0];
/// bgr0_to_rgbx_inplace(&mut bgr0);
/// assert_eq!(bgr0, [1, 2, 3, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4.
#[inline]
pub fn bgr0_to_rgbx_inplace(src: &mut [u8]) {
    apply_x_mask_and_swizzle_4_wide!(src, src, XXX0_TO_XXXX_OR, BGRA_TO_RGBA_SWIZZLE_IDXS);
}

/// Convert BGR0 data to RGBX and store the result to `dst`.
///
/// ```rust
/// use image_swizzle::bgr0_to_rgbx;
/// let bgr0 = [3, 2, 1, 0];
/// let mut bgrx = [0; 4];
/// bgr0_to_rgbx(&bgr0 , &mut bgrx);
/// assert_eq!(bgrx, [1, 2, 3, 255]);
/// ```
///
/// Panics if `src.len` is not multiple of a 4 or if `dst.len` is not equal to `src.len`.
#[inline]
pub fn bgr0_to_rgbx(src: &[u8], dst: &mut [u8]) {
    apply_x_mask_and_swizzle_4_wide!(src, dst, XXX0_TO_XXXX_OR, BGRA_TO_RGBA_SWIZZLE_IDXS);
}

// TODO:
// pub fn rgb_to_rgbx_inplace(src: &mut [u8]) {}
// pub fn rgb_to_rgbx(src: &mut [u8], dst: &mut [u8]) {}
// pub fn rgb_to_bgrx_inplace(src: &mut [u8]) {}
// pub fn rgb_to_bgrx(src: &mut [u8], dst: &mut [u8]) {}
// pub fn bgr_to_rgbx_inplace(src: &mut [u8]) {}
// pub fn bgr_to_rgbx(src: &mut [u8], dst: &mut [u8]) {}
// pub fn bgr_to_bgrx_inplace(src: &mut [u8]) {}
// pub fn bgr_to_bgrx(src: &mut [u8], dst: &mut [u8]) {}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    fn generate_xxxx_image(width: usize, heigh: usize, x1: u8, x2: u8, x3: u8, x4: u8) -> Vec<u8> {
        assert!((width * heigh * 4) % 4 == 0);
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
    fn test_rgba_to_bgra_inplace() {
        let (width, height) = (1920, 1080);
        let mut rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        rgba_to_bgra_inplace(&mut rgba_img);
        assert_eq!(rgba_img, correct_bgra);
    }

    #[test]
    fn test_rgba_to_bgra_inplace_short_lane_count() {
        let (width, height) = (3, 3);
        let mut rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        rgba_to_bgra_inplace(&mut rgba_img);
        assert_eq!(rgba_img, correct_bgra);
    }

    #[test]
    fn test_rgba_to_bgra_inplace_combi() {
        let (width, height) = (5, 5);
        let mut rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        rgba_to_bgra_inplace(&mut rgba_img);
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
    #[should_panic]
    fn test_panic_rgba_to_bgra() {
        let (width, height) = (1920, 1080);
        let rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        let mut bgra = vec![0; width * height * 4 - 10];
        rgba_to_bgra(&rgba_img, &mut bgra);
        assert_eq!(bgra, correct_bgra);
    }

    #[test]
    fn test_bgra_to_rgba_inplace() {
        let (width, height) = (1920, 1080);
        let mut bgra = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_rgba = generate_xxxx_image(width, height, 100, 222, 111, 255);
        bgra_to_rgba_inplace(&mut bgra);
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

    #[test]
    fn test_serial_rgba_to_bgra_inplace() {
        let (width, height) = (1920, 1080);
        let mut rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        serial_swizzle_4_wide_inplace(&mut rgba_img, &[2, 1, 0, 3]);
        assert_eq!(rgba_img, correct_bgra);
    }

    #[test]
    fn test_serial_rgba_to_bgra() {
        let (width, height) = (1920, 1080);
        let rgba_img = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let correct_bgra = generate_xxxx_image(width, height, 100, 222, 111, 255);
        let mut bgra = vec![0; width * height * 4];
        serial_swizzle_4_wide(&rgba_img, &mut bgra, &[2, 1, 0, 3]);
        assert_eq!(bgra, correct_bgra);
    }

    #[test]
    fn test_rgb0_to_rgbx_inplace() {
        let (width, height) = (1920, 1080);
        let mut rgb0_img = generate_xxxx_image(width, height, 111, 222, 100, 0);
        let correct_rgbx = generate_xxxx_image(width, height, 111, 222, 100, 255);
        rgb0_to_rgbx_inplace(&mut rgb0_img);
        assert_eq!(rgb0_img, correct_rgbx);
    }

    #[test]
    fn test_rgb0_to_rgbx() {
        let (width, height) = (1920, 1080);
        let rgb0_img = generate_xxxx_image(width, height, 111, 222, 100, 0);
        let correct_rgbx = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let mut rgbx = vec![0; width * height * 4];
        rgb0_to_rgbx(&rgb0_img, &mut rgbx);
        assert_eq!(rgbx, correct_rgbx);
    }

    #[test]
    fn test_bgr0_to_bgrx_inplace() {
        let (width, height) = (10, 10);
        let mut bgr0_img = generate_xxxx_image(width, height, 111, 222, 100, 0);
        let correct_bgrx = generate_xxxx_image(width, height, 111, 222, 100, 255);
        bgr0_to_bgrx_inplace(&mut bgr0_img);
        assert_eq!(bgr0_img, correct_bgrx);
    }

    #[test]
    fn test_bgr0_to_bgrx() {
        let (width, height) = (10, 10);
        let bgr0_img = generate_xxxx_image(width, height, 111, 222, 100, 0);
        let correct_bgrx = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let mut bgrx = vec![0; width * height * 4];
        bgr0_to_bgrx(&bgr0_img, &mut bgrx);
        assert_eq!(bgrx, correct_bgrx);
    }

    #[test]
    fn test_rgb0_to_bgrx_inplace() {
        let (width, height) = (1920, 1080);
        let mut rgb0_img = generate_xxxx_image(width, height, 111, 222, 100, 0);
        let correct_bgrx = generate_xxxx_image(width, height, 100, 222, 111, 255);
        rgb0_to_bgrx_inplace(&mut rgb0_img);
        assert_eq!(rgb0_img, correct_bgrx);
    }

    #[test]
    fn test_rgb0_to_bgrx() {
        let (width, height) = (1920, 1080);
        let rgb0_img = generate_xxxx_image(width, height, 111, 222, 100, 0);
        let correct_bgrx = generate_xxxx_image(width, height, 100, 222, 111, 255);
        let mut bgrx = vec![0; width * height * 4];
        rgb0_to_bgrx(&rgb0_img, &mut bgrx);
        assert_eq!(bgrx, correct_bgrx);
    }

    #[test]
    fn test_bgr0_to_rgbx_inplace() {
        let (width, height) = (4, 4);
        let mut bgr0_img = generate_xxxx_image(width, height, 111, 222, 100, 0);
        let correct_rgbx = generate_xxxx_image(width, height, 100, 222, 111, 255);
        bgr0_to_rgbx_inplace(&mut bgr0_img);
        assert_eq!(bgr0_img, correct_rgbx);
    }

    #[test]
    fn test_bgr0_to_rgbx() {
        let (width, height) = (1920, 1080);
        let bgr0_img = generate_xxxx_image(width, height, 111, 222, 100, 0);
        let correct_rgbx = generate_xxxx_image(width, height, 100, 222, 111, 255);
        let mut rgbx = vec![0; width * height * 4];
        bgr0_to_rgbx(&bgr0_img, &mut rgbx);
        assert_eq!(rgbx, correct_rgbx);
    }

    // Benchmarks

    #[bench]
    fn bench_serial_rgba_to_bgra(b: &mut Bencher) {
        let (width, height) = (4096, 2160);
        let rgba = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let mut bgra = vec![0; width * height * 4];
        let idxs = [2, 1, 0, 3];
        b.iter(|| {
            serial_swizzle_4_wide(&rgba, &mut bgra, &idxs);
        });
    }

    #[bench]
    fn bench_vectorized_rgba_to_bgra(b: &mut Bencher) {
        let (width, height) = (4096, 2160);
        let rgba = generate_xxxx_image(width, height, 111, 222, 100, 255);
        let mut bgra = vec![0; width * height * 4];
        b.iter(|| {
            rgba_to_bgra(&rgba, &mut bgra);
        });
    }
}
