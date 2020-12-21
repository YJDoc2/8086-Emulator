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
fn test_singleton() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    // aaa
    vm.arch.ax = 0x000F;
    let o = p.parse(1, &mut vm, &mut context, "aaa");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 0x0105);
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));

    // aad
    vm.arch.flag = 0;
    vm.arch.ax = 0x0105;
    let o = p.parse(1, &mut vm, &mut context, "aad");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x000F);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    // aam
    vm.arch.flag = 0;
    vm.arch.ax = 0x00FF;
    let o = p.parse(1, &mut vm, &mut context, "aam");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x1905);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    // aas
    vm.arch.flag = 0;
    vm.arch.ax = 0x00FF;
    let o = p.parse(1, &mut vm, &mut context, "aas");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0xFF09);
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));

    // daa
    vm.arch.flag = 0;
    vm.arch.ax = 0x007D;
    let o = p.parse(1, &mut vm, &mut context, "daa");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x83);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));

    // das
    vm.arch.flag = 0;
    vm.arch.ax = 0x00EE;
    let o = p.parse(1, &mut vm, &mut context, "das");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x88);
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));

    // cbw
    vm.arch.flag = 0;
    vm.arch.ax = 0x00FF;
    let o = p.parse(1, &mut vm, &mut context, "cbw");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0xFFFF);
    assert_eq!(vm.arch.flag, 0);
    vm.arch.ax = 0x001F;
    let o = p.parse(1, &mut vm, &mut context, "cbw");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x001F);

    // cwd
    vm.arch.flag = 0;
    vm.arch.ax = 0xFFFF;
    let o = p.parse(1, &mut vm, &mut context, "cwd");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0xFFFF);
    assert_eq!(vm.arch.dx, 0xFFFF);
    assert_eq!(vm.arch.flag, 0);
    vm.arch.ax = 0x0FFF;
    vm.arch.dx = 5;
    let o = p.parse(1, &mut vm, &mut context, "cwd");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x0FFF);
    assert_eq!(vm.arch.dx, 0);
}

#[test]
fn test_unary_arithmetic() {
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

    // dec
    vm.arch.ax = 0x00FF;
    let o = p.parse(1, &mut vm, &mut context, "dec al");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x00FE);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    vm.arch.dx = 0;
    let o = p.parse(1, &mut vm, &mut context, "dec dx");
    assert!(o.is_ok());
    assert_eq!(vm.arch.dx, 0xFFFF);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));

    // inc
    let o = p.parse(1, &mut vm, &mut context, "inc byte [0]");
    // inc 255
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 0x00);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    // inc 0xFF00
    let o = p.parse(1, &mut vm, &mut context, "inc word l2");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base + 2], 0x01);
    assert_eq!(vm.mem[base + 3], 0xFF);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(!get_flag_state(vm.arch.flag, Flags::AUX_CARRY));

    // neg
    vm.arch.bx = 0x00FF;
    let o = p.parse(1, &mut vm, &mut context, "neg bl");
    assert!(o.is_ok());
    assert_eq!(vm.arch.bx, 0x0000);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));
    let o = p.parse(1, &mut vm, &mut context, "neg bx");
    assert!(o.is_ok());
    assert_eq!(vm.arch.bx, 0xFFFF);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
    assert!(get_flag_state(vm.arch.flag, Flags::PARITY));

    // mul
    vm.mem[base] = 4;
    vm.arch.ax = 0x0004;
    let o = p.parse(1, &mut vm, &mut context, "mul byte l1");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x0010);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    vm.arch.bx = 0x0010;
    let o = p.parse(1, &mut vm, &mut context, "mul bx");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x0100);
    assert_eq!(vm.arch.dx, 0x0000);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));

    // imul
    vm.mem[base] = -4 as i8 as u8;
    vm.arch.ax = 0x0004;
    let o = p.parse(1, &mut vm, &mut context, "imul byte l1");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax as i16, -16);
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    vm.arch.bx = 16;
    let o = p.parse(1, &mut vm, &mut context, "imul bx");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax as i16, -256);
    assert_eq!(vm.arch.dx, 0xFFFF);
    // these will be unset, as the setting condition is of overflow, the dx should not be 0xFFFF
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));

    // div
    vm.mem[base] = 4;
    vm.arch.ax = 0x0004;
    let o = p.parse(1, &mut vm, &mut context, "div byte l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 0x0001);
    vm.arch.bx = 0x0000;
    let o = p.parse(1, &mut vm, &mut context, "div bl");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::INT(0));

    vm.arch.bx = 5;
    vm.arch.ax = 8;
    vm.arch.dx = 0;
    let o = p.parse(1, &mut vm, &mut context, "div bx");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x0001);
    assert_eq!(vm.arch.dx, 0x0003);

    // idiv
    vm.mem[base] = -4 as i16 as u8;
    vm.arch.ax = 0x0006;
    let o = p.parse(1, &mut vm, &mut context, "idiv byte l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(get_byte_reg(&vm, ByteReg::AL) as i8, -1);
    assert_eq!(get_byte_reg(&vm, ByteReg::AH) as i8, 2);
    vm.arch.bx = 0x0000;
    let o = p.parse(1, &mut vm, &mut context, "idiv bl");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::INT(0));

    vm.arch.bx = 5;
    vm.arch.ax = 8;
    vm.arch.dx = 0;
    let o = p.parse(1, &mut vm, &mut context, "idiv bx");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x0001);
    assert_eq!(vm.arch.dx, 0x0003);
}

#[test]
fn test_binary_arithmetic() {
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

    // add
    vm.arch.ax = 0x00FF;
    vm.arch.bx = 0x0001;
    let o = p.parse(1, &mut vm, &mut context, "add al,bl");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(get_byte_reg(&vm, ByteReg::AL), 0);
    assert_eq!(vm.arch.bx, 1);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));

    vm.arch.ax = 0x0101;
    let o = p.parse(1, &mut vm, &mut context, "add word [si],ax");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 0x0101);
    assert_eq!(vm.mem[base], 0);
    assert_eq!(vm.mem[base + 1], 2);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));

    // adc
    vm.arch.ax = 0x00FF;
    vm.mem[base] = 2;
    set_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "adc byte l1,al");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 2);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));

    vm.arch.ax = 0x0101;
    vm.arch.bx = 0xFFFF;
    set_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "adc bx,ax");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x0101);
    assert_eq!(vm.arch.bx, 0x0101);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));

    // sub
    vm.arch.ax = 0x0000;
    vm.mem[base] = 1;
    let o = p.parse(1, &mut vm, &mut context, "sub al,byte [0]");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 1);
    assert_eq!(vm.arch.ax, 0x00FF);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));

    vm.arch.ax = 0x0202;
    vm.mem[base] = 0x00;
    vm.mem[base + 1] = 0x00;
    let o = p.parse(1, &mut vm, &mut context, "sub word l1,ax");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 0xFE);
    assert_eq!(vm.mem[base + 1], 0xFD);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));

    // sbb
    vm.arch.ax = 0x0000;
    vm.mem[base] = 1;
    set_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "sbb al,byte [0]");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 1);
    assert_eq!(vm.arch.ax, 0x00FE);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));

    vm.arch.ax = 0x0202;
    vm.mem[base] = 0x00;
    vm.mem[base + 1] = 0x00;
    set_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "sbb word l1,ax");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 0xFD);
    assert_eq!(vm.mem[base + 1], 0xFD);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));

    // cmp
    vm.arch.ax = 0x0001;
    vm.mem[base] = 1;
    let o = p.parse(1, &mut vm, &mut context, "cmp al,byte [0]");
    assert!(o.is_ok());
    assert_eq!(vm.mem[base], 1);
    assert_eq!(vm.arch.ax, 0x0001);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(!get_flag_state(vm.arch.flag, Flags::SIGN));

    vm.arch.ax = 0x0202;
    vm.mem[base] = 0x00;
    vm.mem[base + 1] = 0x00;
    let o = p.parse(1, &mut vm, &mut context, "cmp word l1,ax");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x0202);
    assert_eq!(vm.mem[base], 0x00);
    assert_eq!(vm.mem[base + 1], 0x00);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert!(!get_flag_state(vm.arch.flag, Flags::OVERFLOW));
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::AUX_CARRY));
    assert!(get_flag_state(vm.arch.flag, Flags::SIGN));
}
