#[allow(unused_macros)]
macro_rules! impl_tests {
    () => {
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
        fn test_rgba32_to_rgba_inplace() {
            let (width, height) = (10, 10);
            #[cfg(target_endian = "little")]
            let mut rgba_img = generate_xxxx_image(width, height, 255, 222, 111, 0);
            #[cfg(target_endian = "big")]
            todo!();
            let correct = generate_xxxx_image(width, height, 0, 111, 222, 255);
            rgba32_to_rgba_inplace(&mut rgba_img);
            assert_eq!(rgba_img, correct);
        }

        #[test]
        fn test_rgba32_to_bgra_inplace() {
            let (width, height) = (10, 10);
            #[cfg(target_endian = "little")]
            let mut rgba_img = generate_xxxx_image(width, height, 255, 222, 111, 0);
            #[cfg(target_endian = "big")]
            todo!();
            let correct = generate_xxxx_image(width, height, 222, 111, 0, 255);
            rgba32_to_bgra_inplace(&mut rgba_img);
            assert_eq!(rgba_img, correct);
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

        #[test]
        fn test_argb_to_rgba_inplace() {
            let (width, height) = (1920, 1080);
            let (r, g, b, a) = (1, 2, 3, 4);
            let mut argb = generate_xxxx_image(width, height, a, r, g, b);
            let correct_rgba = generate_xxxx_image(width, height, r, g, b, a);
            argb_to_rgba_inplace(&mut argb);
            assert_eq!(argb, correct_rgba);
        }
    }
}

#[allow(unused_imports)]
pub(crate) use impl_tests;
