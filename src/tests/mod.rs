use image::{GenericImage, ImageBuffer, Luma};

use super::*;
mod pixel;

static DATA: [u8; 16] = [
    1, 0, 0, 0, //
    0, 1, 1, 0, //
    0, 1, 0, 0, //
    0, 0, 0, 1, //
];

#[test]
fn test_neigbors() {
    let binary_image = BinaryImage::from_raw(4, 4, &DATA);

    assert_eq!(
        Neighbors::get_neighbors(&binary_image, 1, 1).bits(),
        0b1010_0001
    );
    assert_eq!(
        Neighbors::get_neighbors(&binary_image, 2, 2).bits(),
        0b0101_1001
    );
    assert_eq!(
        Neighbors::get_neighbors(&binary_image, 0, 0).bits(),
        0b0000_1000
    );
}

fn test_view<I: GenericImageView<Pixel = Bit>>(image: &I) {
    for ((_, _, pixel1), pixel2) in image.pixels().zip(DATA) {
        assert!(!(*pixel1 ^ (pixel2 > 0)));
    }
}

#[test]
fn test_binary_image() {
    let mut image = BinaryImage::from_raw(4, 4, &DATA);

    assert_eq!(image.width(), 4);
    assert_eq!(image.height(), 4);
    test_view(&image);

    image.put_pixel(3, 3, Bit(false));
}

#[test]
fn test_binary_view() {
    let image: ImageBuffer<Luma<u8>, &[u8]> = ImageBuffer::from_raw(4, 4, DATA.as_ref()).unwrap();
    let view = BinaryView::Ref(&image);

    assert_eq!(view.width(), 4);
    assert_eq!(view.height(), 4);
    test_view(&view);
}
