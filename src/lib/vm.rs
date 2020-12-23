use crate::arch::i8086;
use std::boxed::Box;

/// MB is defined here and should be imported to all other places from here
/// so we can change size of memory from here
pub const MB: u32 = 1 * 1024 * 1024;

// check https://retrocomputing.stackexchange.com/questions/2927/did-the-intel-8086-8088-not-guarantee-the-value-of-sssp-immediately-after-reset
pub const DEFAULT_FLAG: u16 = 0xF000;
pub const CODE_SEG: u16 = 0xFFFF;

/// VM structure, this contains core arch, which contains registers and flags
/// as well as memory separate from arch
pub struct VM {
    pub arch: i8086,
    pub mem: Box<[u8; MB as usize]>,
}

impl VM {
    /// create new VM
    pub fn new() -> VM {
        let mut ret = VM {
            arch: i8086::default(),
            mem: Box::new([0; MB as usize]),
        };

        // Set default values for registers and flags
        ret.arch.flag = DEFAULT_FLAG;
        ret.arch.cs = CODE_SEG;
        return ret;
    }
}

#[test]
fn test_vm_init() {
    // This test is for checking if the vm is successfully created ,
    // as there are records of large array in box generating stack overflow
    let _ = VM::new();
    assert!(true);
}
