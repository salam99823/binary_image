# binary_image

A Rust library for handling binary images (black and white).
This library provides efficient structures and operations for
manipulating binary pixel data, making it suitable for image processing tasks.

## Features

- **Efficient Storage**: Compact representation of binary pixel data for minimal memory usage.
- **Pixel Manipulation**: Support for basic pixel operations such as setting, getting, and modifying pixel values.
- **Arithmetic Operations**: Perform logical operations on binary images, such as AND, OR, and XOR.
- **Image Creation**: Easily create binary images from raw data or other formats.
- **View Support**: Access pixel data through safe views, preventing out-of-bounds errors.
- **Conversion**: Convert binary images to and from other pixel formats (e.g., RGB, RGBA).
- **Iterators**: Provide iterators for easy traversal of pixel data.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
# replace "*" with the most recent version of binary_image
binary_image = "*"
```

## License

- MIT License
- Apache License 2.0
