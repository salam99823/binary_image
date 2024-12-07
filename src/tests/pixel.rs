use super::*;
use num_traits::Num;

#[test]
fn test_bit_operations() {
    let bit1 = Bit(true);
    let bit2 = Bit(false);

    // Test addition (logical OR)
    assert_eq!(bit1 + bit2, Bit(true));

    // Test subtraction (logical XOR)
    assert_eq!(bit1 - bit2, Bit(true));

    // Test multiplication (logical AND)
    assert_eq!(bit1 * Bit(true), Bit(true));
    assert_eq!(bit1 * bit2, Bit(false));

    // Test division (AND with debug assertion)
    assert_eq!(bit1 / Bit(true), Bit(true));

    // Test conversion from string
    assert_eq!(Bit::from_str_radix("1", 2).unwrap(), Bit(true));
    assert_eq!(Bit::from_str_radix("0", 2).unwrap(), Bit(false));
}
