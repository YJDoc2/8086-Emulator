pub const LOWER_BYTE: u8 = u8::MAX;
pub const LOWER_NIBBLE: u8 = 0b00001111;

pub const FLAG_OVERFLOW: u16 = 1 << 11;
pub const FLAG_DIRECTION: u16 = 1 << 10;
pub const FLAG_INTERRUPT: u16 = 1 << 9;
pub const FLAG_TRAP: u16 = 1 << 8;

pub const FLAG_SIGN: u16 = 1 << 7;
pub const FLAG_ZERO: u16 = 1 << 6;
pub const FLAG_AUX_CARRY: u16 = 1 << 4;
pub const FLAG_PARITY: u16 = 1 << 2;
pub const FLAG_CARRY: u16 = 1 << 0;

/// 8086 Hardware Architecture
/// Contains flags and various Registers
#[allow(non_camel_case_types)]
#[derive(Default)]
pub struct i8086 {
    pub flag: u16, // flags

    pub ax: u16, // accumulator
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,

    pub sp: u16, // Stack Pointer
    pub bp: u16, // Base Pointer
    pub si: u16, // Source Index
    pub di: u16, // Destination Index

    pub ip: u16, // Instruction Pointer

    pub cs: u16, // Code Segment
    pub ds: u16, // Data Segment
    pub ss: u16, // Stack Segment
    pub es: u16, // Extra Segment
}
