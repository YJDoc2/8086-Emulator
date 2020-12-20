#[allow(unused_imports)]
use crate::interpreter::interpreter;
#[allow(unused_imports)]
use crate::util::data_util::*;
#[allow(unused_imports)]
use crate::util::flag_util::*;
#[allow(unused_imports)]
use crate::util::interpreter_util::*;
#[allow(unused_imports)]
use crate::vm::VM;

#[test]
fn test_singleton() {
    // aaa
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
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
