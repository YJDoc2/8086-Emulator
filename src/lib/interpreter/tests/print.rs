#[allow(unused_imports)]
use crate::interpreter::interpreter;
#[allow(unused_imports)]
use crate::util::interpreter_util::*;
#[allow(unused_imports)]
use crate::vm::VM;

#[test]
fn test_print() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    let o = p.parse(1, &mut vm, &mut context, "print flags");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::PRINT);

    let o = p.parse(1, &mut vm, &mut context, "print reg");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::PRINT);

    let o = p.parse(1, &mut vm, &mut context, "print mem 0->150");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::PRINT);

    let o = p.parse(1, &mut vm, &mut context, "print mem 50:150");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::PRINT);

    let o = p.parse(1, &mut vm, &mut context, "print mem :150");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::PRINT);
}
