use emulator_8086_lib as lib;

pub fn main() {
    let mut ctx = lib::util::preprocessor_util::Context::default();
    let mut out = lib::util::preprocessor_util::Output::default();
    let p = lib::preprocessor::preprocessor::CodeParser::new();
    let o = p.parse(&mut ctx, &mut out, "DB [5;0] name: DB [2] DB OFFSET name");
    for i in ctx.label_map.keys() {
        println!("{}", i);
    }
    println!("{:?}", o);
    println!("{:?}", out);
}
