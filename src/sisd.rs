const SERIAL_BATCH_WIDTH: usize = 16;

macro_rules! swizzle_4_wide {
    ($src:expr, $dst:expr, $idxs:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len() && $idxs.len() == 4);

        let end = 0;
        // TODO: Check if this is more performant
        // let end = ($src.len() / SERIAL_BATCH_WIDTH) * SERIAL_BATCH_WIDTH;
        // (0..end).step_by(SERIAL_BATCH_WIDTH).for_each(|i| {
        //     let vec = &$src[i..i + SERIAL_BATCH_WIDTH];
        //     #[rustfmt::skip]
        //     let swizzled = &[ // Hardcoded SERIAL_BATCH_WIDTH = 16
        //         vec[$idxs[0]], vec[$idxs[1]], vec[$idxs[2]], vec[$idxs[3]],
        //         vec[$idxs[0] + 4], vec[$idxs[1] + 4], vec[$idxs[2] + 4], vec[$idxs[3] + 4],
        //         vec[$idxs[0] + 8], vec[$idxs[1] + 8], vec[$idxs[2] + 8], vec[$idxs[3] + 8],
        //         vec[$idxs[0] + 12], vec[$idxs[1] + 12], vec[$idxs[2] + 12], vec[$idxs[3] + 12],
        //     ];
        //     $dst[i..i + SERIAL_BATCH_WIDTH].copy_from_slice(swizzled);
        // });

        (end..$src.len()).step_by(4).for_each(|i| {
            let (a, b, c, d) = ($src[i + 0], $src[i + 1], $src[i + 2], $src[i + 3]);
            $dst[i + $idxs[0]] = a;
            $dst[i + $idxs[1]] = b;
            $dst[i + $idxs[2]] = c;
            $dst[i + $idxs[3]] = d;
        });
    };
}

macro_rules! change_alpha_fourth {
    ($src:expr, $dst:expr, $to:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len());
        (0..$src.len()).step_by(4).for_each(|i| {
            $dst[i] = $src[i];
            $dst[i + 1] = $src[i + 1];
            $dst[i + 2] = $src[i + 2];
            $dst[i + 3] = $to;
        })
    };
}

macro_rules! change_alpha_fourth_and_swizzle {
    ($src:expr, $dst:expr, $to:expr, $idxs:expr) => {
        assert!($src.len() % 4 == 0 && $src.len() == $dst.len() && $idxs.len() == 4);
        (0..$src.len()).step_by(4).for_each(|i| {
            let (a, b, c) = ($src[i + 0], $src[i + 1], $src[i + 2]);
            $dst[i + $idxs[0]] = a;
            $dst[i + $idxs[1]] = b;
            $dst[i + $idxs[2]] = c;
            $dst[i + $idxs[3]] = $to;
        })
    };
}

#[inline(always)]
pub fn rgba_to_bgra_inplace(src: &mut [u8]) {
    swizzle_4_wide!(src, src, [2, 1, 0, 3]);
}

#[inline(always)]
pub fn rgba_to_bgra(src: &[u8], dst: &mut [u8]) {
    swizzle_4_wide!(src, dst, [2, 1, 0, 3]);
}

#[inline(always)]
pub fn rgba32_to_bgra_inplace(src: &mut [u8]) {
    #[cfg(target_endian = "little")]
    swizzle_4_wide!(src, src, [1, 2, 3, 0]);
    #[cfg(target_endian = "big")]
    swizzle_4_wide!(src, src, [2, 1, 0, 3]);
}

#[inline(always)]
pub fn rgba32_to_rgba_inplace(src: &mut [u8]) {
    #[cfg(target_endian = "little")]
    swizzle_4_wide!(src, src, [3, 2, 1, 0]);
}

#[inline(always)]
pub fn bgra_to_rgba_inplace(src: &mut [u8]) {
    swizzle_4_wide!(src, src, [2, 1, 0, 3]);
}

#[inline(always)]
pub fn bgra_to_rgba(src: &[u8], dst: &mut [u8]) {
    swizzle_4_wide!(src, dst, [2, 1, 0, 3]);
}

#[inline(always)]
pub fn rgb0_to_rgbx_inplace(src: &mut [u8]) {
    change_alpha_fourth!(src, src, 255);
}

#[inline(always)]
pub fn rgb0_to_rgbx(src: &[u8], dst: &mut [u8]) {
    change_alpha_fourth!(src, dst, 255);
}

#[inline(always)]
pub fn bgr0_to_bgrx_inplace(src: &mut [u8]) {
    change_alpha_fourth!(src, src, 255);
}

#[inline(always)]
pub fn bgr0_to_bgrx(src: &[u8], dst: &mut [u8]) {
    change_alpha_fourth!(src, dst, 255);
}

#[inline(always)]
pub fn rgb0_to_bgrx_inplace(src: &mut [u8]) {
    change_alpha_fourth_and_swizzle!(src, src, 255, [2, 1, 0, 3]);
}

#[inline(always)]
pub fn rgb0_to_bgrx(src: &[u8], dst: &mut [u8]) {
    change_alpha_fourth_and_swizzle!(src, dst, 255, [2, 1, 0, 3]);
}

#[inline(always)]
pub fn bgr0_to_rgbx_inplace(src: &mut [u8]) {
    change_alpha_fourth_and_swizzle!(src, src, 255, [2, 1, 0, 3]);
}

#[inline(always)]
pub fn bgr0_to_rgbx(src: &[u8], dst: &mut [u8]) {
    change_alpha_fourth_and_swizzle!(src, dst, 255, [2, 1, 0, 3]);
}


#[cfg(test)]
mod tests {
    use super::*;

    crate::common::impl_tests!{}
}
