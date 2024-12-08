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
    let neigbors = Neighbors::get_neighbors(&binary_image, 1, 1);

    assert_eq!(neigbors.bits(), 0b1010_0001);

    let neigbors = Neighbors::get_neighbors(&binary_image, 2, 2);

    assert_eq!(neigbors.bits(), 0b0101_1001);

    let neigbors = Neighbors::get_neighbors(&binary_image, 0, 0);

    assert_eq!(neigbors.bits(), 0b0000_1000);
}

#[test]
fn test_binary_image_creation() {
    let binary_image = BinaryImage::from_raw(4, 4, &DATA);

    assert_eq!(binary_image.width(), 4);
    assert_eq!(binary_image.height(), 4);
    assert!(*binary_image.get_pixel(0, 0));
    assert!(*binary_image.get_pixel(2, 1));
    assert!(*binary_image.get_pixel(1, 2));
    assert!(*binary_image.get_pixel(3, 3));
}

#[test]
fn test_binary_view() {
    let binary_image = BinaryImage::from_raw(4, 4, &DATA);
    let view = BinaryView(&binary_image);

    assert_eq!(view.width(), 4);
    assert_eq!(view.height(), 4);
    assert!(*view.get_pixel(0, 0));
    assert!(*view.get_pixel(2, 1));
    assert!(*view.get_pixel(1, 2));
    assert!(*view.get_pixel(3, 3));
}

#[test]
fn test_binary_raw_view() {
    let raw_view = BinaryRawView::new(4, 4, &DATA);

    assert_eq!(raw_view.width(), 4);
    assert_eq!(raw_view.height(), 4);
    assert!(*raw_view.get_pixel(0, 0));
    assert!(*raw_view.get_pixel(2, 1));
    assert!(*raw_view.get_pixel(1, 2));
    assert!(*raw_view.get_pixel(3, 3));
}
