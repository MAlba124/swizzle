use std::simd::{self, simd_swizzle, u8x16, u8x4};

#[rustfmt::skip]
macro_rules! idx_order {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        [
            $a          , $b          , $c          , $d          ,
            $a +  4     , $b +  4     , $c +  4     , $d +  4     ,
            $a + (4 * 2), $b + (4 * 2), $c + (4 * 2), $d + (4 * 2),
            $a + (4 * 3), $b + (4 * 3), $c + (4 * 3), $d + (4 * 3),
        ]
    }
}

const VECTOR_WIDTH: usize = 16;
const RGBA_TO_BGRA_SWIZZLE_IDXS: [usize; VECTOR_WIDTH] = idx_order!(2, 1, 0, 3);
const RGBA_TO_BGRA_SWIZZLE_IDXS_SHORT: [usize; 4] = [2, 1, 0, 3];
const BGRA_TO_RGBA_SWIZZLE_IDXS: [usize; VECTOR_WIDTH] = idx_order!(2, 1, 0, 3);
const BGRA_TO_RGBA_SWIZZLE_IDXS_SHORT: [usize; 4] = [2, 1, 0, 3];
#[rustfmt::skip]
const XXX0_TO_XXXX_OR: u8x16 = u8x16::from_array([
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
    0u8, 0u8, 0u8, 255u8,
]);
const XXX0_TO_XXXX_OR_SHORT: u8x4 = u8x4::from_array([0u8, 0u8, 0u8, 255u8]);

macro_rules! swizzle_4_wide {
    ($src:expr, $dst:expr, $idxs:expr, $idxs_short:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len());

        let end = ($src.len() / VECTOR_WIDTH) * VECTOR_WIDTH;
        (0..end).step_by(VECTOR_WIDTH).for_each(|i| {
            simd_swizzle!(u8x16::from_slice(&$src[i..i + VECTOR_WIDTH]), $idxs)
                .copy_to_slice(&mut $dst[i..i + VECTOR_WIDTH]);
        });

        (end..$src.len()).step_by(4).for_each(|i| {
            simd_swizzle!(u8x4::from_slice(&$src[i..i + 4]), $idxs_short)
                .copy_to_slice(&mut $dst[i..i + 4]);
        });
    };
}

macro_rules! apply_mask_4_wide {
    ($src:expr, $dst:expr, $mask:expr, $mask_short:expr, $or:expr, $or_short:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len());

        let end = ($src.len() / VECTOR_WIDTH) * VECTOR_WIDTH;
        (0..end).step_by(VECTOR_WIDTH).for_each(|i| {
            u8x16::load_select(&$src[i..i + VECTOR_WIDTH], $mask, $or)
                .copy_to_slice(&mut $dst[i..i + VECTOR_WIDTH]);
        });

        (end..$src.len()).step_by(4).for_each(|i| {
            u8x4::load_select(&$src[i..i + 4], $mask_short, $or_short)
                .copy_to_slice(&mut $dst[i..i + 4]);
        });
    };
}

macro_rules! apply_x_mask_and_swizzle_4_wide {
    ($src:expr, $dst:expr, $or:expr, $or_short:expr, $idxs:expr, $idxs_short:expr) => {
        println!("assert!({} % 4 == 0 && {} == {});", $src.len(), $src.len(), $dst.len());
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len());

        #[rustfmt::skip]
        let mask = simd::Mask::<i8, 16>::from_array([
            true, true, true, false,
            true, true, true, false,
            true, true, true, false,
            true, true, true, false,
        ]);
        let mask_short = simd::Mask::<i8, 4>::from_array([
            true, true, true, false,
        ]);

        let end = ($src.len() / VECTOR_WIDTH) * VECTOR_WIDTH;
        (0..end).step_by(VECTOR_WIDTH).for_each(|i| {
            simd_swizzle!(
                u8x16::load_select(&$src[i..i + VECTOR_WIDTH], mask, $or),
                $idxs
            )
            .copy_to_slice(&mut $dst[i..i + VECTOR_WIDTH]);
        });

        (end..$src.len()).step_by(4).for_each(|i| {
            simd_swizzle!(
                u8x4::load_select(&$src[i..i + 4], mask_short, $or_short),
                $idxs_short
            )
            .copy_to_slice(&mut $dst[i..i + 4]);
        });
    };
}


#[inline(always)]
pub fn rgba_to_bgra_inplace(src: &mut [u8]) {
    swizzle_4_wide!(
        src,
        src,
        RGBA_TO_BGRA_SWIZZLE_IDXS,
        RGBA_TO_BGRA_SWIZZLE_IDXS_SHORT
    );
}

#[inline(always)]
pub fn rgba_to_bgra(src: &[u8], dst: &mut [u8]) {
    swizzle_4_wide!(
        src,
        dst,
        RGBA_TO_BGRA_SWIZZLE_IDXS,
        RGBA_TO_BGRA_SWIZZLE_IDXS_SHORT
    );
}

#[inline(always)]
pub fn bgra_to_rgba_inplace(src: &mut [u8]) {
    swizzle_4_wide!(
        src,
        src,
        BGRA_TO_RGBA_SWIZZLE_IDXS,
        BGRA_TO_RGBA_SWIZZLE_IDXS_SHORT
    );
}

#[inline(always)]
pub fn bgra_to_rgba(src: &[u8], dst: &mut [u8]) {
    swizzle_4_wide!(
        src,
        dst,
        BGRA_TO_RGBA_SWIZZLE_IDXS,
        BGRA_TO_RGBA_SWIZZLE_IDXS_SHORT
    );
}

#[inline(always)]
pub fn rgb0_to_rgbx_inplace(src: &mut [u8]) {
    #[rustfmt::skip]
    let mask = simd::Mask::<i8, 16>::from_array([
        true, true, true, false,
        true, true, true, false,
        true, true, true, false,
        true, true, true, false,
    ]);
    let mask_short = simd::Mask::<i8, 4>::from_array([
        true, true, true, false,
    ]);
    apply_mask_4_wide!(
        src,
        src,
        mask,
        mask_short,
        XXX0_TO_XXXX_OR,
        XXX0_TO_XXXX_OR_SHORT
    );
}

#[inline(always)]
pub fn rgb0_to_rgbx(src: &[u8], dst: &mut [u8]) {
    #[rustfmt::skip]
    let mask = simd::Mask::<i8, 16>::from_array([
        true, true, true, false,
        true, true, true, false,
        true, true, true, false,
        true, true, true, false,
    ]);
    let mask_short = simd::Mask::<i8, 4>::from_array([
        true, true, true, false,
    ]);
    apply_mask_4_wide!(
        src,
        dst,
        mask,
        mask_short,
        XXX0_TO_XXXX_OR,
        XXX0_TO_XXXX_OR_SHORT
    );
}

#[inline(always)]
pub fn bgr0_to_bgrx_inplace(src: &mut [u8]) {
    #[rustfmt::skip]
    let mask = simd::Mask::<i8, 16>::from_array([
        true, true, true, false,
        true, true, true, false,
        true, true, true, false,
        true, true, true, false,
    ]);
    let mask_short = simd::Mask::<i8, 4>::from_array([
        true, true, true, false,
    ]);
    apply_mask_4_wide!(
        src,
        src,
        mask,
        mask_short,
        XXX0_TO_XXXX_OR,
        XXX0_TO_XXXX_OR_SHORT
    );
}

#[inline(always)]
pub fn bgr0_to_bgrx(src: &[u8], dst: &mut [u8]) {
    #[rustfmt::skip]
    let mask = simd::Mask::<i8, 16>::from_array([
        true, true, true, false,
        true, true, true, false,
        true, true, true, false,
        true, true, true, false,
    ]);
    let mask_short = simd::Mask::<i8, 4>::from_array([
        true, true, true, false,
    ]);
    apply_mask_4_wide!(
        src,
        dst,
        mask,
        mask_short,
        XXX0_TO_XXXX_OR,
        XXX0_TO_XXXX_OR_SHORT
    );
}

#[inline(always)]
pub fn rgb0_to_bgrx_inplace(src: &mut [u8]) {
    apply_x_mask_and_swizzle_4_wide!(
        src,
        src,
        XXX0_TO_XXXX_OR,
        XXX0_TO_XXXX_OR_SHORT,
        RGBA_TO_BGRA_SWIZZLE_IDXS,
        RGBA_TO_BGRA_SWIZZLE_IDXS_SHORT
    );
}

#[inline(always)]
pub fn rgb0_to_bgrx(src: &[u8], dst: &mut [u8]) {
    apply_x_mask_and_swizzle_4_wide!(
        src,
        dst,
        XXX0_TO_XXXX_OR,
        XXX0_TO_XXXX_OR_SHORT,
        RGBA_TO_BGRA_SWIZZLE_IDXS,
        RGBA_TO_BGRA_SWIZZLE_IDXS_SHORT
    );
}

#[inline(always)]
pub fn bgr0_to_rgbx_inplace(src: &mut [u8]) {
    apply_x_mask_and_swizzle_4_wide!(
        src,
        src,
        XXX0_TO_XXXX_OR,
        XXX0_TO_XXXX_OR_SHORT,
        BGRA_TO_RGBA_SWIZZLE_IDXS,
        BGRA_TO_RGBA_SWIZZLE_IDXS_SHORT
    );
}

#[inline(always)]
pub fn bgr0_to_rgbx(src: &[u8], dst: &mut [u8]) {
    apply_x_mask_and_swizzle_4_wide!(
        src,
        dst,
        XXX0_TO_XXXX_OR,
        XXX0_TO_XXXX_OR_SHORT,
        BGRA_TO_RGBA_SWIZZLE_IDXS,
        BGRA_TO_RGBA_SWIZZLE_IDXS_SHORT
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    crate::common::impl_tests!{}
}
