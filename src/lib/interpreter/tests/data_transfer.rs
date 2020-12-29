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
fn test_seg_override(){
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    vm.arch.es = 0xF;
    vm.mem[0xF*0x10+1] = 0xFF;
    let o = p.parse(1, &mut vm, &mut context, "mov ax, word es:[1]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 0x00FF);

    vm.arch.ss = 0xF;
    vm.arch.si = 10;
    vm.mem[0xF*0x10+10] = 0xFF;
    let o = p.parse(1, &mut vm, &mut context, "mov bx, word ss:[si]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.bx, 0x00FF);

    vm.arch.cs = 0xF;
    vm.arch.bp = 5;
    vm.mem[0xF*0x10+5] = 0xFF;
    let o = p.parse(1, &mut vm, &mut context, "mov ch, byte cs:[bp]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.cx, 0xFF00);

    vm.arch.es = 0xF;
    vm.arch.di = 2;
    vm.arch.bx = 5;
    vm.mem[0xF*0x10+10] = 0xFF;
    let o = p.parse(1, &mut vm, &mut context, "mov dl, byte es:[bx,di,3]");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.dx, 0x00FF);
}

#[test]
fn test_data_singleton() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();

    vm.arch.ax = 0xFFFF;

    let o = p.parse(1, &mut vm, &mut context, "lahf");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 0x00FF);

    vm.arch.ax = 0xFFFF;
    let o = p.parse(1, &mut vm, &mut context, "sahf");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0xFFFF);
    assert_eq!(vm.arch.flag, 0xF0FF);

    vm.arch.sp = 5;
    let o = p.parse(1, &mut vm, &mut context, "pushf");
    assert!(o.is_ok());
    assert_eq!(vm.arch.sp, 3);
    assert_eq!(vm.mem[3], 0xFF);
    assert_eq!(vm.mem[4], 0xF0);

    vm.mem[3] = 0x00;
    vm.mem[4] = 0x0F;
    let o = p.parse(1, &mut vm, &mut context, "popf");
    assert!(o.is_ok());
    assert_eq!(vm.arch.sp, 5);
    assert_eq!(vm.arch.flag, 0x0F00);

    vm.arch.ax = 0x0003;
    vm.arch.bx = 0;
    vm.mem[3] = 0x15;
    let o = p.parse(1, &mut vm, &mut context, "xlat");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x0015);
}

#[test]
fn test_xchg() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    vm.arch.ax = 0xFF00;
    vm.arch.si = 0x00FF;
    let o = p.parse(1, &mut vm, &mut context, "xchg ax,si");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 0x00FF);
    assert_eq!(vm.arch.si, 0xFF00);

    vm.arch.bp = 0xABCD;
    vm.mem[0] = 0xFF;
    vm.mem[1] = 0xAA;
    let o = p.parse(1, &mut vm, &mut context, "xchg word l1,bp");
    assert!(o.is_ok());
    assert_eq!(vm.arch.bp, 0xAAFF);
    assert_eq!(vm.mem[0], 0xCD);
    assert_eq!(vm.mem[1], 0xAB);
}

#[test]
fn test_lea() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    vm.arch.ds = 5;
    let o = p.parse(1, &mut vm, &mut context, "lea ax,word l1 ");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 0);

    vm.arch.ds = 0;
    vm.arch.ss = 10;
    vm.arch.bp = 6;
    // [bp] = ss*0x10+bp = 166 = 0xA6
    let o = p.parse(1, &mut vm, &mut context, "lea ax,word [bp]");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0xA6);

    vm.arch.ds = 10;
    // [bp] = ss*0x10+bp = 166 = 0xA6
    // effective address = 166 - ds *0x10 = 6
    let o = p.parse(1, &mut vm, &mut context, "lea ax,word [bp]");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 6);
}

#[test]
fn test_push_pop() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    vm.arch.cs = 0xABCD;
    vm.arch.sp = 10;
    let o = p.parse(1, &mut vm, &mut context, "push cs");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.cs, 0xABCD);
    assert_eq!(vm.arch.sp, 8);
    assert_eq!(vm.mem[8], 0xCD);
    assert_eq!(vm.mem[9], 0xAB);
    vm.mem[0] = 0xEF;
    vm.mem[1] = 0xCD;

    let o = p.parse(1, &mut vm, &mut context, "push word l1");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.sp, 6);
    assert_eq!(vm.mem[6], 0xEF);
    assert_eq!(vm.mem[7], 0xCD);

    let o = p.parse(1, &mut vm, &mut context, "pop word l2");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.sp, 8);
    assert_eq!(vm.mem[6], 0xEF);
    assert_eq!(vm.mem[7], 0xCD);
    assert_eq!(vm.mem[2], 0xEF);
    assert_eq!(vm.mem[3], 0xCD);

    let o = p.parse(1, &mut vm, &mut context, "pop si");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.si, 0xABCD);
    assert_eq!(vm.arch.sp, 10);
    assert_eq!(vm.mem[8], 0xCD);
    assert_eq!(vm.mem[9], 0xAB);
}

#[test]
fn test_data_mov() {
    let mut vm = VM::new();
    let mut context = Context::default();
    let p = interpreter::InterpreterParser::new();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    // seg reg cannot have immediate movs
    let o = p.parse(1, &mut vm, &mut context, "mov ss,5");
    assert!(o.is_err());
    let o = p.parse(1, &mut vm, &mut context, "mov ax,5");
    assert!(o.is_ok());
    assert_eq!(o.unwrap(), State::NEXT);
    assert_eq!(vm.arch.ax, 5);

    let o = p.parse(1, &mut vm, &mut context, "mov ss,ax");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ss, 5);

    let o = p.parse(1, &mut vm, &mut context, "mov al,-5");
    assert!(o.is_ok());
    assert_eq!(get_byte_reg(&vm, ByteReg::AL) as i8, -5);

    let o = p.parse(1, &mut vm, &mut context, "mov word [0],-1");
    assert!(o.is_ok());
    assert_eq!(vm.mem[0], 0xFF);
    assert_eq!(vm.mem[1], 0xFF);

    let o = p.parse(1, &mut vm, &mut context, "mov al,byte l1");
    assert!(o.is_ok());
    assert_eq!(vm.arch.ax, 0x00FF);

    let o = p.parse(1, &mut vm, &mut context, "mov word l1, ax");
    assert!(o.is_ok());
    assert_eq!(vm.mem[0], 0xFF);
    assert_eq!(vm.mem[1], 0x00);
}
