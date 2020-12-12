#[allow(unused_imports)]
use crate::interpreter::interpreter;
#[allow(unused_imports)]
use crate::util::flag_util::*;
#[allow(unused_imports)]
use crate::util::interpreter_util::*;
#[allow(unused_imports)]
use crate::util::preprocessor_util::{Label, LabelType};
#[allow(unused_imports)]
use crate::vm::VM;

#[test]
fn test_call_ret_instructions() {
    let mut vm = VM::new();
    let mut context = Context::default();

    context.fn_map.insert("f1".to_owned(), 5);
    context.fn_map.insert("f2".to_owned(), 12);

    let p = interpreter::InterpreterParser::new();

    let o = p.parse(1, &mut vm, &mut context, "call f1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    assert_eq!(context.call_stack[0], 1);

    let o = p.parse(5, &mut vm, &mut context, "call f2");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(12));
    assert_eq!(context.call_stack.len(), 2);
    assert_eq!(context.call_stack[0], 1);
    assert_eq!(context.call_stack[1], 5);

    let o = p.parse(12, &mut vm, &mut context, "call f3");
    assert!(o.is_err());

    let o = p.parse(12, &mut vm, &mut context, "ret");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    assert_eq!(context.call_stack.len(), 1);
    assert_eq!(context.call_stack[0], 1);

    let o = p.parse(5, &mut vm, &mut context, "ret");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(1));
    assert_eq!(context.call_stack.len(), 0);

    let o = p.parse(2, &mut vm, &mut context, "ret");
    assert!(o.is_err());
}

#[allow(dead_code)]
fn setup() -> (VM, Context, interpreter::InterpreterParser) {
    let vm = VM::new();
    let mut context = Context::default();

    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::CODE, 0, 5));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::CODE, 0, 12));
    let p = interpreter::InterpreterParser::new();

    return (vm, context, p);
}

#[test]
fn test_jmp() {
    let (mut vm, mut context, p) = setup();

    context
        .label_map
        .insert("l3".to_owned(), Label::new(LabelType::DATA, 0, 3));

    // test unconditional jump and invalid label error conditions
    // as invalid label errors are checked in same place for all jumps following tests should be enough
    let o = p.parse(1, &mut vm, &mut context, "jmp l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    let o = p.parse(1, &mut vm, &mut context, "jmp l3");
    assert!(o.is_err());
    let o = p.parse(1, &mut vm, &mut context, "jmp l4");
    assert!(o.is_err());
}

#[test]
fn test_ja() {
    let (mut vm, mut context, p) = setup();
    let o = p.parse(1, &mut vm, &mut context, "ja l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    set_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "ja l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    set_flag(&mut vm.arch.flag, Flags::ZERO);
    let o = p.parse(1, &mut vm, &mut context, "ja l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    set_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "ja l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
}

#[test]
fn test_jump_carry() {
    let (mut vm, mut context, p) = setup();

    // carry = 0
    let o = p.parse(1, &mut vm, &mut context, "jae l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jb l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jnc l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jc l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    // carry = 1
    set_flag(&mut vm.arch.flag, Flags::CARRY);

    let o = p.parse(1, &mut vm, &mut context, "jae l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jnc l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jb l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jc l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
}

#[test]
fn test_jmp_zero() {
    let (mut vm, mut context, p) = setup();

    // zero = 0
    let o = p.parse(1, &mut vm, &mut context, "je l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jne l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    // zero = 1
    set_flag(&mut vm.arch.flag, Flags::ZERO);
    let o = p.parse(1, &mut vm, &mut context, "je l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jne l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
}

#[test]
fn test_jbe() {
    let (mut vm, mut context, p) = setup();

    let o = p.parse(1, &mut vm, &mut context, "jbe l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    set_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "jbe l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    set_flag(&mut vm.arch.flag, Flags::ZERO);
    let o = p.parse(1, &mut vm, &mut context, "jbe l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    set_flag(&mut vm.arch.flag, Flags::CARRY);
    let o = p.parse(1, &mut vm, &mut context, "jbe l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
}

#[test]
fn test_jg_jle() {
    let (mut vm, mut context, p) = setup();

    // sf = of , zf = 0
    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jle l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    set_flag(&mut vm.arch.flag, Flags::SIGN);
    set_flag(&mut vm.arch.flag, Flags::OVERFLOW);

    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jle l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    unset_flag(&mut vm.arch.flag, Flags::SIGN);
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);

    // sf != of, zf = 0
    set_flag(&mut vm.arch.flag, Flags::SIGN);
    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    let o = p.parse(1, &mut vm, &mut context, "jle l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    unset_flag(&mut vm.arch.flag, Flags::SIGN);
    set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jle l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    // sf = of , zf = 1
    set_flag(&mut vm.arch.flag, Flags::ZERO);

    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jle l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    set_flag(&mut vm.arch.flag, Flags::SIGN);
    set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jle l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    unset_flag(&mut vm.arch.flag, Flags::SIGN);
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);

    // sf != of, zf = 1
    set_flag(&mut vm.arch.flag, Flags::SIGN);
    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jle l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    unset_flag(&mut vm.arch.flag, Flags::SIGN);
    set_flag(&mut vm.arch.flag, Flags::OVERFLOW);

    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jle l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
}

#[test]
fn test_jge_jl() {
    let (mut vm, mut context, p) = setup();

    // sf = of
    let o = p.parse(1, &mut vm, &mut context, "jge l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    let o = p.parse(1, &mut vm, &mut context, "jl l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    set_flag(&mut vm.arch.flag, Flags::SIGN);
    set_flag(&mut vm.arch.flag, Flags::OVERFLOW);

    let o = p.parse(1, &mut vm, &mut context, "jge l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jl l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    unset_flag(&mut vm.arch.flag, Flags::SIGN);
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);

    // sf != of
    set_flag(&mut vm.arch.flag, Flags::SIGN);
    let o = p.parse(1, &mut vm, &mut context, "jge l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jl l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    unset_flag(&mut vm.arch.flag, Flags::SIGN);
    set_flag(&mut vm.arch.flag, Flags::OVERFLOW);

    let o = p.parse(1, &mut vm, &mut context, "jg l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jl l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
}

#[test]
fn test_of_pf_sf() {
    let (mut vm, mut context, p) = setup();

    // of = 0 pf = 0 sf = 0
    let o = p.parse(1, &mut vm, &mut context, "jno l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jnp l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jns l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jo l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jp l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "js l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    set_flag(&mut vm.arch.flag, Flags::PARITY);
    set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    set_flag(&mut vm.arch.flag, Flags::SIGN);

    let o = p.parse(1, &mut vm, &mut context, "jno l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jnp l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jns l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);

    let o = p.parse(1, &mut vm, &mut context, "jo l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "jp l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    let o = p.parse(1, &mut vm, &mut context, "js l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
}

#[test]
fn test_jcxz() {
    let (mut vm, mut context, p) = setup();

    let o = p.parse(1, &mut vm, &mut context, "jcxz l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));

    vm.arch.cx = 5;

    let o = p.parse(1, &mut vm, &mut context, "jcxz l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
}

#[test]
fn test_loop() {
    let (mut vm, mut context, p) = setup();

    vm.arch.cx = 2;

    let o = p.parse(1, &mut vm, &mut context, "loop l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    assert_eq!(vm.arch.cx, 1);

    let o = p.parse(1, &mut vm, &mut context, "loop l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.cx, 0);

    let o = p.parse(1, &mut vm, &mut context, "loop l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    assert_eq!(vm.arch.cx, u16::MAX);
}

#[test]
fn test_loope() {
    let (mut vm, mut context, p) = setup();

    vm.arch.cx = 3;

    let o = p.parse(1, &mut vm, &mut context, "loope l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.cx, 2);

    set_flag(&mut vm.arch.flag, Flags::ZERO);

    let o = p.parse(1, &mut vm, &mut context, "loope l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    assert_eq!(vm.arch.cx, 1);

    let o = p.parse(1, &mut vm, &mut context, "loope l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.cx, 0);

    let o = p.parse(1, &mut vm, &mut context, "loope l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    assert_eq!(vm.arch.cx, u16::MAX);
}

#[test]
fn test_loopne() {
    let (mut vm, mut context, p) = setup();

    vm.arch.cx = 3;

    let o = p.parse(1, &mut vm, &mut context, "loopne l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::JMP(5));
    assert_eq!(vm.arch.cx, 2);

    set_flag(&mut vm.arch.flag, Flags::ZERO);

    let o = p.parse(1, &mut vm, &mut context, "loopne l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.cx, 1);

    let o = p.parse(1, &mut vm, &mut context, "loopne l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.cx, 0);

    let o = p.parse(1, &mut vm, &mut context, "loopne l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.cx, u16::MAX);
}

#[test]
fn test_int() {
    let (mut vm, mut context, p) = setup();

    let o = p.parse(1, &mut vm, &mut context, "int 3");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::INT(3));

    let o = p.parse(1, &mut vm, &mut context, "int 16");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::INT(0x10));

    let o = p.parse(1, &mut vm, &mut context, "int 33");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::INT(0x21));

    let o = p.parse(1, &mut vm, &mut context, "int 10");
    assert!(o.is_err());

    let o = p.parse(1, &mut vm, &mut context, "int 15");
    assert!(o.is_err());
}
