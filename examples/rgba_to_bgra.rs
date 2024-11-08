fn main() {
    let (width, height) = (4096, 4096);
    let mut frame = Vec::new();
    for _ in 0..width * height {
        frame.extend([000, 111, 222, 255]);
    }

    image_swizzle::rgba_to_bgra_inplace(&mut frame);
}
