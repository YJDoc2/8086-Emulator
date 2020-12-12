#[allow(unused_imports)]
use crate::interpreter::interpreter;
#[allow(unused_imports)]
use crate::util::flag_util::*;
#[allow(unused_imports)]
use crate::util::interpreter_util::*;
#[allow(unused_imports)]
use crate::vm::VM;

#[test]
fn test_control_instructions() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    let o = p.parse(1, &mut vm, &mut context, "stc");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));

    let o = p.parse(1, &mut vm, &mut context, "clc");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(!get_flag_state(vm.arch.flag, Flags::CARRY));

    let o = p.parse(1, &mut vm, &mut context, "cmc");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(get_flag_state(vm.arch.flag, Flags::CARRY));

    let o = p.parse(1, &mut vm, &mut context, "std");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(get_flag_state(vm.arch.flag, Flags::DIRECTION));

    let o = p.parse(1, &mut vm, &mut context, "cld");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(!get_flag_state(vm.arch.flag, Flags::DIRECTION));

    let o = p.parse(1, &mut vm, &mut context, "sti");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(get_flag_state(vm.arch.flag, Flags::INTERRUPT));

    let o = p.parse(1, &mut vm, &mut context, "cli");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert!(!get_flag_state(vm.arch.flag, Flags::DIRECTION));

    let o = p.parse(1, &mut vm, &mut context, "hlt");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::HALT);
}
