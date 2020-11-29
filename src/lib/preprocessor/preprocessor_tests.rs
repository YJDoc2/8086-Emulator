#[allow(unused_imports)]
use super::preprocessor;

#[test]
fn test_data_directives() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = preprocessor::data_directivesParser::new();
    let o = p.parse(&mut ctx, &mut out, "set 0x45 set 6");
    assert!(o.is_ok());
    assert_eq!(out.data.len(), 2);
    assert_eq!(out.code.len(), 0);
    assert_eq!(out.data[0], "set 69");
    assert_eq!(out.data[1], "set 6");

    out.clear();

    let o = p.parse(&mut ctx, &mut out, "test: DB 5");
    assert!(o.is_ok());
    assert_eq!(out.data.len(), 1);
    assert_eq!(out.code.len(), 0);
    assert_eq!(out.data[0], "db 5");
    out.clear();

    let o = p.parse(&mut ctx, &mut out, "test: DB 5 test: DB 0b1110");
    assert!(o.is_err());
    out.clear();
    ctx.clear();

    let o = p.parse(&mut ctx, &mut out, "test: DB [5]");
    assert!(o.is_ok());
    out.clear();
    ctx.clear();

    let o = p.parse(&mut ctx, &mut out, "test: DB [5;6]");
    assert!(o.is_ok());
}

#[test]
fn test_macro_directives() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::code_directivesParser::new();
    let o = p.parse(&mut ctx, &mut out, "macro mcname (a) { DB [a] }\nmcname(5)");
    assert!(o.is_ok());
    assert_eq!(out.data.len(), 1);
    out.clear();
    ctx.clear();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "macro mcname (a) { DB [a] }\nmcname(hello)",
    );
    assert!(o.is_err());
    assert_eq!(out.data.len(), 0);
}

#[test]
fn test_control_opcode() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::CodeParser::new();
    let o = p.parse(&mut ctx, &mut out, "ctc CMC HLT");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 3);
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "ESC LOCK");
    assert!(o.is_err());
}

#[test]
fn test_transfer_opcode() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::CodeParser::new();
    let o = p.parse(&mut ctx, &mut out, "JMP test JGE go");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 2);
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "RET INTO");
    assert!(o.is_err());
}
