use emulator_8086_lib as lib;

pub fn main() {
    let mut ctx = lib::util::preprocessor_util::Context::default();
    let mut out = lib::util::preprocessor_util::Output::default();
    let p = lib::preprocessor::preprocessor::data_directivesParser::new();
    let o = p.parse(&mut ctx, &mut out, "test: DB [5;6]");
    println!("{:?}", o);
    println!("{:?}", out);
}
