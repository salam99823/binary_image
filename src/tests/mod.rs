use image::{ImageBuffer, Luma};

use super::*;
mod pixel;

static DATA: [u8; 16] = [
    1, 0, 0, 0, //
    0, 1, 1, 0, //
    0, 1, 0, 0, //
    0, 0, 0, 1, //
];

fn test_view<I: GenericImageView<Pixel = Bit>>(image: &I) {
    for ((_, _, pixel1), pixel2) in image.pixels().zip(DATA) {
        assert!(!(*pixel1 ^ (pixel2 > 0)));
    }
}

#[test]
fn test_binary_image() {
    let image = BinaryImage::from_raw(4, 4, &DATA);

    assert_eq!(image.width(), 4);
    assert_eq!(image.height(), 4);
    test_view(&image);
}

#[test]
fn test_binary_view() {
    let image: ImageBuffer<Luma<u8>, &[u8]> = ImageBuffer::from_raw(4, 4, DATA.as_ref()).unwrap();
    let view = BinaryView::Ref(&image);

    assert_eq!(view.width(), 4);
    assert_eq!(view.height(), 4);
    test_view(&view);
}
