use emulator_8086_lib as lib;

pub fn main() {
    let mut ctx = lib::util::preprocessor_util::Context::default();
    let mut out = lib::util::preprocessor_util::Output::default();
    let p = lib::preprocessor::preprocessor::CodeParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "l:DB [7;0] _t:DB 0 print flags print reg print mem 0x0FFFF-> 0x100FF print mem 0xFFFFF:50 print mem : offset _t",
    );
    println!("{:?}", o);
    println!("{:?}", out);
}
