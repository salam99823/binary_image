/// Checks if the given neighbors state indicates a corner pixel.
///
/// # Arguments
///
/// * `neighbors` - A byte representing the state of neighboring pixels.
///
/// # Returns
///
/// Returns `true` if the pixel is a corner, and `false` otherwise.
pub fn is_corner(neighbors: u8) -> bool {
    !matches!(
        neighbors,
        255
            | 239
            | 238
            | 235
            | 234
            | 223
            | 221
            | 215
            | 213
            | 188..=207
            | 127
            | 123
            | 119
            | 115
            | 48..=63
            | 9
            | 6
            | 0
    )
}
