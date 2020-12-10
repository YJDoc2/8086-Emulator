use crate::vm::MB;

#[inline]
pub fn make_valid_address<T: Copy + Into<usize>>(v: T) -> usize {
    return v.into() % MB as usize;
}

#[inline]
pub fn inc_addr<T: std::ops::Add<Output = T> + Copy + Into<usize>>(v: T, inc: T) -> usize {
    return (v + inc).into() % MB as usize;
}

pub struct Address;

impl Address {
    #[inline]
    pub fn is_valid(addr: u32) -> bool {
        addr < MB
    }

    #[inline]
    pub fn calculate_from_offset<T: Copy + Into<usize>, V: Copy + Into<usize>>(
        base: T,
        offset: V,
    ) -> usize {
        make_valid_address(base.into() * 0x10 + offset.into())
    }
}

// TODO Decide output format
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self)
    }
}
