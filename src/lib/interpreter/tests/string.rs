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
fn test_movs() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    let base = vm.arch.ds as usize * 0x10;
    for i in 0..10 {
        vm.mem[base + i] = 10 + i as u8;
    }
    vm.arch.di = 10;
    let o = p.parse(1, &mut vm, &mut context, "movs byte");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 10], 10);
    assert_eq!(vm.mem[base + 11], 0);
    assert_eq!(vm.arch.si, 1);
    assert_eq!(vm.arch.di, 11);

    vm.arch.cx = 5;
    let o = p.parse(1, &mut vm, &mut context, "rep movs word");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::REPEAT);
    assert_eq!(vm.mem[base + 11], 11);
    assert_eq!(vm.mem[base + 12], 12);
    assert_eq!(vm.mem[base + 13], 0);
    assert_eq!(vm.arch.si, 3);
    assert_eq!(vm.arch.di, 13);
    assert_eq!(vm.arch.cx, 4);
}

#[test]
fn test_loads() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    let base = vm.arch.ds as usize * 0x10;
    for i in 0..10 {
        vm.mem[base + i] = 10 + i as u8;
    }
    let o = p.parse(1, &mut vm, &mut context, "lods byte");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(get_byte_reg(&vm, ByteReg::AL), 10);
    assert_eq!(vm.arch.si, 1);
    assert_eq!(vm.arch.di, 0);

    let o = p.parse(1, &mut vm, &mut context, "lods word");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 0x0C0B);
    assert_eq!(vm.arch.si, 3);
    assert_eq!(vm.arch.di, 0);
}

#[test]
fn test_stos() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    let base = vm.arch.ds as usize * 0x10;
    for i in 0..10 {
        vm.mem[base + i] = 10 + i as u8;
    }
    vm.arch.ax = 0xABCD;
    vm.arch.di = 10;
    let o = p.parse(1, &mut vm, &mut context, "stos byte");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 10], 0xCD);
    assert_eq!(vm.arch.si, 0);
    assert_eq!(vm.arch.di, 11);

    let o = p.parse(1, &mut vm, &mut context, "stos word");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 11], 0xCD);
    assert_eq!(vm.mem[base + 12], 0xAB);
    assert_eq!(vm.arch.si, 0);
    assert_eq!(vm.arch.di, 13);
}

#[test]
fn test_cmps() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    let base = vm.arch.ds as usize * 0x10;
    for i in 0..10 {
        vm.mem[base + i] = 10 + i as u8;
    }

    for i in 10..20 {
        vm.mem[base + i] = i as u8;
    }

    vm.arch.di = 10;
    let o = p.parse(1, &mut vm, &mut context, "cmps byte");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 10], 10);
    assert_eq!(vm.arch.si, 1);
    assert_eq!(vm.arch.di, 11);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));

    vm.mem[base + 12] = 15;
    let o = p.parse(1, &mut vm, &mut context, "cmps word");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert_eq!(vm.arch.si, 3);
    assert_eq!(vm.arch.di, 13);

    vm.arch.cx = 5;
    set_flag(&mut vm.arch.flag, Flags::ZERO);
    let o = p.parse(1, &mut vm, &mut context, "repz cmps word");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::REPEAT);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    assert_eq!(vm.arch.si, 5);
    assert_eq!(vm.arch.di, 15);
    assert_eq!(vm.arch.cx, 4);

    vm.arch.si = 1;
    vm.arch.di = 11;
    vm.arch.cx = 5;
    set_flag(&mut vm.arch.flag, Flags::ZERO);
    let o = p.parse(1, &mut vm, &mut context, "repz cmps word");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert_eq!(vm.arch.si, 3);
    assert_eq!(vm.arch.di, 13);
    assert_eq!(vm.arch.cx, 4);
}

#[test]
fn test_scas() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    let base = vm.arch.ds as usize * 0x10;
    for i in 0..10 {
        vm.mem[base + i] = 10 + i as u8;
    }

    vm.arch.di = 0;
    vm.arch.ax = 0x0B0A;
    set_flag(&mut vm.arch.flag, Flags::DIRECTION);
    let o = p.parse(1, &mut vm, &mut context, "scas byte");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.si, 0);
    assert_eq!(vm.arch.di, u16::MAX);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    unset_flag(&mut vm.arch.flag, Flags::DIRECTION);

    vm.mem[base + 1] = 15;
    vm.arch.di = 0;
    let o = p.parse(1, &mut vm, &mut context, "scas word");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert_eq!(vm.arch.di, 2);

    set_flag(&mut vm.arch.flag, Flags::ZERO);
    vm.arch.cx = 5;
    let o = p.parse(1, &mut vm, &mut context, "repnz scas word");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::REPEAT);
    assert!(!get_flag_state(vm.arch.flag, Flags::ZERO));
    assert_eq!(vm.arch.si, 0);
    assert_eq!(vm.arch.di, 4);
    assert_eq!(vm.arch.cx, 4);

    vm.arch.di = 0;
    vm.arch.cx = 5;
    vm.mem[base + 1] = 11;
    set_flag(&mut vm.arch.flag, Flags::ZERO);
    let o = p.parse(1, &mut vm, &mut context, "repnz scas byte");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(get_flag_state(vm.arch.flag, Flags::ZERO));
    assert_eq!(vm.arch.si, 0);
    assert_eq!(vm.arch.di, 1);
    assert_eq!(vm.arch.cx, 4);
}
