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

#[test]
fn test_binary_image_creation() {
    let image = BinaryImage::from_raw(4, 4, &DATA);

    assert_eq!(image.width(), 4);
    assert_eq!(image.height(), 4);
    assert!(*image.get_pixel(0, 0));
    assert!(*image.get_pixel(2, 1));
    assert!(*image.get_pixel(1, 2));
    assert!(*image.get_pixel(3, 3));
}

#[test]
fn test_view() {
    let image = BinaryImage::from_raw(4, 4, &DATA);
    let view = BinaryView(&image);

    assert_eq!(view.width(), 4);
    assert_eq!(view.height(), 4);
    assert!(*view.get_pixel(0, 0));
    assert!(*view.get_pixel(2, 1));
    assert!(*view.get_pixel(1, 2));
    assert!(*view.get_pixel(3, 3));
}

#[test]
fn test_view_raw() {
    let image: image::ImageBuffer<image::Luma<u8>, &[u8]> =
        image::ImageBuffer::from_raw(4, 4, &DATA[..]).unwrap();
    let view = BinaryView(&image);

    assert_eq!(view.width(), 4);
    assert_eq!(view.height(), 4);
    assert!(*view.get_pixel(0, 0));
    assert!(*view.get_pixel(2, 1));
    assert!(*view.get_pixel(1, 2));
    assert!(*view.get_pixel(3, 3));
}
