use emulator_8086_lib as lib;
#[allow(unused_imports)]
use lib::instructions::bit_manipulation::*;
#[allow(unused_imports)]
use lib::util::data_util::*;
use lib::util::preprocessor_util::{Label, LabelType};
pub fn main() {
    let mut vm = lib::vm::VM::new();
    let p = lib::interpreter::interpreter::InterpreterParser::new();
    let mut context = lib::util::interpreter_util::Context::default();
    context
        .label_map
        .insert("l1".to_owned(), Label::new(LabelType::DATA, 0, 0));
    context
        .label_map
        .insert("l2".to_owned(), Label::new(LabelType::DATA, 0, 2));

    //let base = vm.arch.ss as usize * 0x10;
    vm.arch.sp = 4;
    vm.arch.ax = 0xF0F0;
    let o = p.parse(1, &mut vm, &mut context, "push ax");
    println!("{:?}", o);
}
