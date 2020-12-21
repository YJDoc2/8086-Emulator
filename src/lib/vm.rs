use crate::arch::i8086;
use std::boxed::Box;

pub const MB: u32 = 1 * 1024 * 1024;

// check https://retrocomputing.stackexchange.com/questions/2927/did-the-intel-8086-8088-not-guarantee-the-value-of-sssp-immediately-after-reset
pub const DEFAULT_FLAG: u16 = 0xF000;
pub const CODE_SEG: u16 = 0xFFFF;

// TODO what to use for mem
pub struct VM {
    pub arch: i8086,
    pub mem: Box<[u8; MB as usize]>,
}

impl VM {
    pub fn new() -> VM {
        let mut ret = VM {
            arch: i8086::default(),
            mem: Box::new([0; MB as usize]),
        };

        // Set default values for registers
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
