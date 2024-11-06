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
// TODO: Add simd as feature
// TODO: Use criterion for benchmarking

#![cfg_attr(feature = "nightly", feature(portable_simd))]

#[cfg(feature = "nightly")]
mod simd;
#[cfg(not(feature = "nightly"))]
mod sisd;

pub(crate) mod common;

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
    #[cfg(feature = "nightly")]
    simd::rgba_to_bgra_inplace(src);
    #[cfg(not(feature = "nightly"))]
    sisd::rgba_to_bgra_inplace(src);
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
    #[cfg(feature = "nightly")]
    simd::rgba_to_bgra(src, dst);
    #[cfg(not(feature = "nightly"))]
    sisd::rgba_to_bgra(src, dst);
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
    #[cfg(feature = "nightly")]
    simd::bgra_to_rgba_inplace(src);
    #[cfg(not(feature = "nightly"))]
    sisd::bgra_to_rgba_inplace(src);
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
    #[cfg(feature = "nightly")]
    simd::bgra_to_rgba(src, dst);
    #[cfg(not(feature = "nightly"))]
    sisd::bgra_to_rgba(src, dst);
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
    #[cfg(feature = "nightly")]
    simd::rgb0_to_rgbx_inplace(src);
    #[cfg(not(feature = "nightly"))]
    sisd::rgb0_to_rgbx_inplace(src);
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
    #[cfg(feature = "nightly")]
    simd::rgb0_to_rgbx(src, dst);
    #[cfg(not(feature = "nightly"))]
    sisd::rgb0_to_rgbx(src, dst);
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
    #[cfg(feature = "nightly")]
    simd::bgr0_to_bgrx_inplace(src);
    #[cfg(not(feature = "nightly"))]
    sisd::bgr0_to_bgrx_inplace(src);
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
    #[cfg(feature = "nightly")]
    simd::bgr0_to_bgrx(src, dst);
    #[cfg(not(feature = "nightly"))]
    sisd::bgr0_to_bgrx(src, dst);
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
    #[cfg(feature = "nightly")]
    simd::rgb0_to_bgrx_inplace(src);
    #[cfg(not(feature = "nightly"))]
    sisd::rgb0_to_bgrx_inplace(src);
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
    #[cfg(feature = "nightly")]
    simd::rgb0_to_bgrx(src, dst);
    #[cfg(not(feature = "nightly"))]
    sisd::rgb0_to_bgrx(src, dst);
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
    #[cfg(feature = "nightly")]
    simd::bgr0_to_rgbx_inplace(src);
    #[cfg(not(feature = "nightly"))]
    sisd::bgr0_to_rgbx_inplace(src);
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
    #[cfg(feature = "nightly")]
    simd::bgr0_to_rgbx(src, dst);
    #[cfg(not(feature = "nightly"))]
    sisd::bgr0_to_rgbx(src, dst);
}

// TODO:
// pub fn rgb_to_rgbx(src: &mut [u8], dst: &mut [u8]) {}
// pub fn rgb_to_bgrx(src: &mut [u8], dst: &mut [u8]) {}
// pub fn bgr_to_rgbx(src: &mut [u8], dst: &mut [u8]) {}
// pub fn bgr_to_bgrx(src: &mut [u8], dst: &mut [u8]) {}

#[cfg(test)]
mod tests {
    use super::*;

    common::impl_tests!{}
}
