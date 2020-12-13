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
