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
    assert_eq!(vm.mem[base + 3], 0);

    let o = p.parse(1, &mut vm, &mut context, "not word [1]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.mem[base + 1], 0);
    assert_eq!(vm.mem[base + 2], 255);
}
