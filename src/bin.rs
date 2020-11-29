use emulator_8086_lib as lib;

pub fn main() {
    let mut ctx = lib::util::preprocessor_util::Context::default();
    let mut out = lib::util::preprocessor_util::Output::default();
    let p = lib::preprocessor::preprocessor::code_directivesParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "macro mcname (a) { DB [a] JNE lbl  }\nmcname(5)",
    );
    println!("{:?}", o);
    println!("{:?}", out);
}
