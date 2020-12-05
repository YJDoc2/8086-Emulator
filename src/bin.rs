use emulator_8086_lib as lib;

pub fn main() {
    let mut ctx = lib::util::preprocessor_util::Context::default();
    let mut out = lib::util::preprocessor_util::Output::default();
    let p = lib::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "MACRO a(q)-> ADD AX,q <- MACRO b(k,q) -> k (q)<- b(b,5)",
    );
    if let Err(e) = o {
        println!("{}", e);
    }
    println!("{:?}", out);
}
