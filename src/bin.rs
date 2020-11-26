use emulator_8086_lib as lib;

pub fn main() {
    let mut ctx = lib::util::preprocessor_util::Context::default();
    let mut out = Vec::new();
    let p = lib::preprocessor::preprocessor::codeParser::new();
    let o = p.parse(&mut ctx, &mut out, "111111");
    println!("{:?}", o);
    println!("{:?}", out);
}
