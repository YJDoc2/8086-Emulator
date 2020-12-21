#[allow(unused_imports)]
use crate::interpreter::interpreter;
#[allow(unused_imports)]
use crate::util::data_util::*;
#[allow(unused_imports)]
use crate::util::flag_util::*;
#[allow(unused_imports)]
use crate::util::interpreter_util::*;
#[allow(unused_imports)]
use crate::util::preprocessor_util::{Label, LabelType};
#[allow(unused_imports)]
use crate::vm::VM;

#[test]
fn test_not() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    let base = vm.arch.ds as usize * 0x10;
    vm.mem[base] = 255;
    vm.mem[base + 1] = 0;
    vm.mem[base + 2] = 0;
    vm.mem[base + 3] = 255;

    let o = p.parse(1, &mut vm, &mut context, "not byte l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base], 0);
    assert_eq!(vm.mem[base + 1], 0);

    let o = p.parse(1, &mut vm, &mut context, "not byte l2");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 2], 255);
    assert_eq!(vm.mem[base + 3], 255);

    let o = p.parse(1, &mut vm, &mut context, "not word [1]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 1], 255);
    assert_eq!(vm.mem[base + 2], 0);

    vm.arch.bx = 1;

    let o = p.parse(1, &mut vm, &mut context, "not word [bx]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 1], 0);
    assert_eq!(vm.mem[base + 2], 255);

    vm.arch.si = 1;

    let o = p.parse(1, &mut vm, &mut context, "not word [si]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 1], 255);
    assert_eq!(vm.mem[base + 2], 0);

    vm.arch.ss = 0x50; // by default ss = ds = 0, so we change ss to verify that ds does not change
    let stack_base = vm.arch.ss as usize * 0x10;
    vm.mem[stack_base] = 0;
    vm.mem[stack_base + 1] = 255;

    let o = p.parse(1, &mut vm, &mut context, "not word [bp]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    // should not affect ds
    assert_eq!(vm.mem[base + 1], 255);
    assert_eq!(vm.mem[base + 2], 0);
    // should affect ss
    assert_eq!(vm.mem[stack_base], 255);
    assert_eq!(vm.mem[stack_base + 1], 0);

    let o = p.parse(1, &mut vm, &mut context, "not byte [bp,di,0]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    // should affect ss
    assert_eq!(vm.mem[stack_base], 0);
    assert_eq!(vm.mem[stack_base + 1], 0);

    let o = p.parse(1, &mut vm, &mut context, "not word [bp,di,2]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    // should affect ss
    assert_eq!(vm.mem[stack_base + 1], 0);
    assert_eq!(vm.mem[stack_base + 2], 255);
    assert_eq!(vm.mem[stack_base + 3], 255);
    assert_eq!(vm.mem[stack_base + 4], 0);
}

#[test]
fn test_binary_logical() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    let base = vm.arch.ds as usize * 0x10;

    const V1: u8 = 155;
    const V2: u8 = 122;
    const V3: u16 = 0xF0F0;

    vm.mem[base] = V1;
    vm.mem[base + 1] = V2;
    vm.mem[base + 2] = V2;
    vm.mem[base + 3] = V1;

    vm.arch.ax = V3; // 0x F0F0
    vm.arch.cx = !V3; // 0x 0F0F

    // reg,reg
    let o = p.parse(1, &mut vm, &mut context, "or al,cl");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(get_byte_reg(&vm, ByteReg::AL), 0xFF);
    assert_eq!(get_byte_reg(&vm, ByteReg::AH), 0xF0);
    assert_eq!(get_byte_reg(&vm, ByteReg::CL), 0x0F);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    // ax = 0x F0FF
    // cx = 0x 0F0F
    let o = p.parse(1, &mut vm, &mut context, "and cx,ax");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0xF0FF);
    assert_eq!(vm.arch.cx, 0xF);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    // ax = 0x F0FF
    // cx = 0x 000F
    let o = p.parse(1, &mut vm, &mut context, "xor cx,cx");
    assert!(o.is_ok());
    assert_eq!(vm.arch.cx, 0);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    // reg , mem
    vm.arch.ax = V3; // 0x F0F0
    vm.arch.cx = !V3; // 0x 0F0F

    let o = p.parse(1, &mut vm, &mut context, "or cl, byte [0]");
    assert!(o.is_ok());
    assert_eq!(get_byte_reg(&vm, ByteReg::CL), 0xF | V1);
    assert_eq!(vm.mem[base], V1);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    let o = p.parse(1, &mut vm, &mut context, "xor ax, word l1");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0xF0F0 ^ 0x7A9B);
    assert_eq!(vm.mem[base], V1);
    assert_eq!(vm.mem[base + 1], V2);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::PARITY));

    // mem , reg
    vm.arch.ax = V3; // 0x F0F0
    vm.arch.cx = !V3; // 0x 0F0F

    let o = p.parse(1, &mut vm, &mut context, "and byte [0],cl");
    assert!(o.is_ok());
    assert_eq!(get_byte_reg(&vm, ByteReg::CL), 0x0F);
    assert_eq!(vm.mem[base], V1 & 0x0F);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::PARITY));

    let o = p.parse(1, &mut vm, &mut context, "xor word l2 ,ax");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, V3);
    assert_eq!(vm.mem[base + 2], V2 ^ 0xF0);
    assert_eq!(vm.mem[base + 3], V1 ^ 0xF0);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::PARITY));

    // reg , imm
    let o = p.parse(1, &mut vm, &mut context, "and al,15");
    assert!(o.is_ok());
    assert_eq!(get_byte_reg(&vm, ByteReg::AL), 0x00);
    assert_eq!(get_byte_reg(&vm, ByteReg::AH), 0xF0);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    let o = p.parse(1, &mut vm, &mut context, "or cx,20303"); //0x 4F4F
    assert!(o.is_ok());
    assert_eq!(vm.arch.cx, 0x0F0F | 0x4F4F);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::PARITY));

    // mem, imm
    vm.mem[base] = V1;
    vm.mem[base + 1] = V2;
    vm.mem[base + 2] = V2;
    vm.mem[base + 3] = V1;

    let o = p.parse(1, &mut vm, &mut context, "and byte [bx],15");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 155 & 15);
    assert_eq!(vm.mem[base + 1], 122);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::PARITY));

    let o = p.parse(1, &mut vm, &mut context, "xor word l2 ,20303"); // 0x 4F4F
    assert!(o.is_ok());
    assert_eq!(vm.mem[base + 2], V2 ^ 0x4F);
    assert_eq!(vm.mem[base + 3], V1 ^ 0x4F);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    // test instruction

    vm.arch.ax = V3; // 0x F0F0
    vm.arch.cx = !V3; // 0x F0F0
    vm.mem[base] = V1;
    vm.mem[base + 1] = V2;

    let o = p.parse(1, &mut vm, &mut context, "test cx,ax");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, V3);
    assert_eq!(vm.arch.cx, !V3);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    let o = p.parse(1, &mut vm, &mut context, "test word [bx],15");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], V1);
    assert_eq!(vm.mem[base + 1], V2);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::PARITY));
}

#[test]
fn test_shifts() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    vm.arch.ax = 0xF0F0; // ah = 11110000 al = 11110000

    // reg
    let o = p.parse(1, &mut vm, &mut context, "sal ah,5");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(get_byte_reg(&vm, ByteReg::AH), 0b0);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    vm.arch.cx = 1;
    let o = p.parse(1, &mut vm, &mut context, "shr al,cl");
    assert!(o.is_ok());
    assert_eq!(get_byte_reg(&vm, ByteReg::AL), 0b01111000);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    vm.arch.cx = 0xF0F0; // 1111000011110000
    let o = p.parse(1, &mut vm, &mut context, "sar cx,9");
    assert!(o.is_ok());
    assert_eq!(vm.arch.cx, 0b1111111111111000);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::PARITY));

    // mem
    let base = vm.arch.ds as usize * 0x10;

    vm.mem[base] = 0b11110000;
    vm.mem[base + 1] = 0b00001111;
    vm.mem[base + 2] = 0b00001111;
    vm.mem[base + 3] = 0b11110000;

    vm.arch.cx = 2;
    let o = p.parse(1, &mut vm, &mut context, "sar byte l1,cl");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 0b11111100);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    let o = p.parse(1, &mut vm, &mut context, "sal word [bx,2],3");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base + 2], 0b01111000);
    assert_eq!(vm.mem[base + 3], 0b10000000);
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY)); // only lower 8 bytes are considered for parity
}

#[test]
fn test_rotates() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    vm.arch.ax = 0xF0F0; // ah = 11110000 al = 11110000

    // reg
    let o = p.parse(1, &mut vm, &mut context, "rol ah,5");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(get_byte_reg(&vm, ByteReg::AH), 0b00011110);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));

    vm.arch.bx = 0xF0F0;
    let o = p.parse(1, &mut vm, &mut context, "ror bx,12");
    assert!(o.is_ok());
    assert_eq!(vm.arch.bx, 0xF0F);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));

    // mem
    let base = vm.arch.ds as usize * 0x10;

    vm.mem[base] = 0b11110000;
    vm.mem[base + 1] = 0b00001111;
    vm.mem[base + 2] = 0b00001111;
    vm.mem[base + 3] = 0b11110000;
    vm.arch.cx = 3;

    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "rcl byte l1,cl");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 0b10000011);
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));

    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    vm.arch.bx = 0;
    let o = p.parse(1, &mut vm, &mut context, "rcr word [bx,2],6");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base + 2], 0b11000000);
    assert_eq!(vm.mem[base + 3], 0b01111011);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
}
