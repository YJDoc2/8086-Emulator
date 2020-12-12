#[allow(unused_imports)]
use crate::interpreter::interpreter;
#[allow(unused_imports)]
use crate::util::flag_util::*;
#[allow(unused_imports)]
use crate::util::interpreter_util::*;
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
