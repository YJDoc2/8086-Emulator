use emulator_8086_lib as lib;
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

    let base = vm.arch.ds as usize * 0x10;

    const V1: u8 = 155;
    const V2: u8 = 122;
    const V3: u16 = 0xF0F0;

    vm.mem[base] = V1;
    vm.mem[base + 1] = V2;
    vm.mem[base + 2] = V2;
    vm.mem[base + 3] = V1;

    vm.arch.ax = V3; // 0x F0F0
    vm.arch.cx = !V3; // 0x 0F0F

    let o = p.parse(1, &mut vm, &mut context, "xor ax, word [0]");
    println!("{:?}", o);
    println!("{:x}", vm.arch.ax);
}
