use crate::vm::MB;

#[derive(Debug)]
pub struct Address(u32);

impl Address {
    // Not much Useful
    pub fn new() -> Self {
        Address(0)
    }

    pub fn is_valid(addr: u32) -> bool {
        addr < MB
    }

    pub fn calculate_from_offset(base: u16, offset: u16) -> Self {
        Address::from((base as u32) * 0x10 + offset as u32)
    }

    pub fn get_usize(&self) -> usize {
        return self.0 as usize;
    }
}

// TODO Decide output format
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

impl From<u32> for Address {
    fn from(addr: u32) -> Self {
        return Address(addr % MB);
    }
}

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        return Address(addr as u32);
    }
}

impl From<u8> for Address {
    fn from(addr: u8) -> Self {
        return Address(addr as u32);
    }
}
