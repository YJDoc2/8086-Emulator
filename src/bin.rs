use emulator_8086_lib as lib;

pub fn main() {
    let mut ctx = lib::util::preprocessor_util::Context::default();
    let mut out = lib::util::preprocessor_util::Output::default();
    let p = lib::preprocessor::preprocessor::macro_defParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "MACRO mcname (a,b,c) { hello a b;\nhello b c;\nhello c d; }",
    );
    println!("{:?}", o);
    println!("{:?}", out);
}
