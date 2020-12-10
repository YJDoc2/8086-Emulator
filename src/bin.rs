use emulator_8086_lib as lib;

pub fn main() {
    let mut vm = lib::vm::VM::new();
    let mut ctr = 0;
    let p = lib::data_parser::data_parser::DataParser::new();
    let _ = p.parse(&mut vm, &mut ctr, "set 4352");
    let o = p.parse(&mut vm, &mut ctr, "dw \"ABCD\"");
    println!("{:?}", o);
    println!(
        "{:?}",
        &vm.mem[(vm.arch.ds as usize) * 0x10..(vm.arch.ds as usize) * 0x10 + 10],
    );
}
