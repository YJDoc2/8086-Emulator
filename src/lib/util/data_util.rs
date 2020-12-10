const LOWER_BYTE: i16 = 255;

#[inline]
pub fn separate_bytes(val: i16) -> (u8, u8) {
    let lb = (val & LOWER_BYTE) as u8;
    let hb = ((val & !LOWER_BYTE) >> 8) as u8;
    (hb, lb)
}

#[test]
fn test_separate_byte() {
    let (hb, lb) = separate_bytes(0b0000111111110000);
    assert_eq!(lb, 0b11110000);
    assert_eq!(hb, 0b00001111);
}
