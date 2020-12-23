use crate::vm::MB;

#[inline]
/// just a % MB  function
pub fn make_valid_address<T: Copy + Into<usize>>(v: T) -> usize {
    return v.into() % MB as usize;
}

#[inline]
/// to increment address wrapping above MB
pub fn inc_addr<T: std::ops::Add<Output = T> + Copy + Into<usize>>(v: T, inc: T) -> usize {
    return (v + inc).into() % MB as usize;
}

/// just an empty structure to give namespace to important addressing related functions
/// as currently this contains only one function, may be removed later
pub struct Address;

impl Address {
    #[inline]
    /// calculate usize address from base and offset
    /// using base * 0x10 + offset
    pub fn calculate_from_offset<T: Copy + Into<usize>, V: Copy + Into<usize>>(
        base: T,
        offset: V,
    ) -> usize {
        make_valid_address(base.into() * 0x10 + offset.into())
    }
}

// We don't use this, but for contingency reasons
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self)
    }
}
