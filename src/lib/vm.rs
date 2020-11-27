use crate::arch::i8086;
use std::boxed::Box;

pub const MB: u32 = 1 * 1024 * 1024;

// All these are arbitrary
// Stack should be higher, as on x86 stack grows downwards
pub const DATA_SEG_BEGIN: u16 = 0x1000; // 4096
pub const STACK_SEG_BEGIN: u16 = 0x7000; // 28672

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
        ret.arch.ds = DATA_SEG_BEGIN;
        ret.arch.ss = STACK_SEG_BEGIN;
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
