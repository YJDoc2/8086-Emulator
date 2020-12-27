#[allow(unused_imports)]
use super::data_parser;
#[allow(unused_imports)]
use crate::vm::VM;

#[test]
fn test_db_number() {
    let mut vm = VM::new();
    let mut ctr = 0;
    let p = data_parser::DataParser::new();

    let o = p.parse(&mut vm, &mut ctr, "db 5");
    assert!(o.is_ok());
    assert_eq!(vm.mem[vm.arch.ds as usize * 0x10], 5);

    ctr = 0;
    let o = p.parse(&mut vm, &mut ctr, "db -1");
    assert!(o.is_ok());
    assert_eq!(vm.mem[vm.arch.ds as usize * 0x10], 255);

    // First test for numbered array, so we can check if non-numbered array actually zeros the memory
    ctr = 0;
    let o = p.parse(&mut vm, &mut ctr, "db [50 , 5]");
    assert!(o.is_ok());
    assert_eq!(
        &vm.mem[vm.arch.ds as usize * 0x10..vm.arch.ds as usize * 0x10 + 6],
        &[50, 50, 50, 50, 50, 0]
    );

    ctr = 0;
    let o = p.parse(&mut vm, &mut ctr, "db [5]");
    assert!(o.is_ok());
    assert_eq!(
        &vm.mem[vm.arch.ds as usize * 0x10..vm.arch.ds as usize * 0x10 + 6],
        &[0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_db_string() {
    let mut vm = VM::new();
    let mut ctr = 0;
    let p = data_parser::DataParser::new();
    let o = p.parse(&mut vm, &mut ctr, "db \"ABCD\"");
    assert!(o.is_ok());
    // -2 is done for starting address to check if by any chance the " is not stored
    // even though it should work same without -2, as ctr is zero
    assert_eq!(
        &vm.mem[vm.arch.ds as usize * 0x10..vm.arch.ds as usize * 0x10 + 6],
        &[65, 66, 67, 68, 0, 0]
    );
}

#[test]
fn test_dw_number() {
    let mut vm = VM::new();
    let mut ctr = 0;
    let p = data_parser::DataParser::new();
    // 43981 = 0xABCD
    let o = p.parse(&mut vm, &mut ctr, "dw 43981");
    assert!(o.is_ok());
    assert_eq!(
        &vm.mem[vm.arch.ds as usize * 0x10..vm.arch.ds as usize * 0x10 + 3],
        &[205, 171, 0]
    );

    ctr = 0;
    let o = p.parse(&mut vm, &mut ctr, "dw -1");
    assert!(o.is_ok());
    assert_eq!(vm.mem[vm.arch.ds as usize * 0x10], 255);
    assert_eq!(vm.mem[vm.arch.ds as usize * 0x10 + 1], 255);

    // First test for numbered array, so we can check if non-numbered array actually zeros the memory
    ctr = 0;
    let o = p.parse(&mut vm, &mut ctr, "dw [43981 , 5]");
    assert!(o.is_ok());
    assert_eq!(
        &vm.mem[vm.arch.ds as usize * 0x10..vm.arch.ds as usize * 0x10 + 12],
        &[205, 171, 205, 171, 205, 171, 205, 171, 205, 171, 0, 0]
    );

    ctr = 0;
    let o = p.parse(&mut vm, &mut ctr, "dw [5]");
    assert!(o.is_ok());
    assert_eq!(
        &vm.mem[vm.arch.ds as usize * 0x10..vm.arch.ds as usize * 0x10 + 12],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn test_dw_string() {
    let mut vm = VM::new();
    let mut ctr = 0;
    let p = data_parser::DataParser::new();
    let o = p.parse(&mut vm, &mut ctr, "dw \"ABCD\"");
    assert!(o.is_ok());
    // -2 is done for starting address to check if by any chance the " is not stored
    // even though it should work same without -2, as ctr is zero
    assert_eq!(
        &vm.mem[vm.arch.ds as usize * 0x10..vm.arch.ds as usize * 0x10 + 12],
        &[65, 0, 66, 0, 67, 0, 68, 0, 0, 0, 0,0]
    );
}

#[test]
fn test_set_directive() {
    let mut vm = VM::new();
    let mut ctr = 0;
    let p = data_parser::DataParser::new();
    let o = p.parse(&mut vm, &mut ctr, "set 4352");
    assert!(o.is_ok());
    let o = p.parse(&mut vm, &mut ctr, "dw \"ABCD\"");
    assert!(o.is_ok());

    // as the ds is modified by set, it has to be manually reset
    vm.arch.ds = 0x1000;
    // -2 is done for starting address to check if by any chance the " is not stored
    // even though it should work same without -2, as ctr is zero
    assert_eq!(
        &vm.mem[(vm.arch.ds as usize + 0x100) * 0x10..(vm.arch.ds as usize + 0x100) * 0x10 + 10],
        &[65, 0, 66, 0, 67, 0, 68, 0, 0,0]
    );
}
