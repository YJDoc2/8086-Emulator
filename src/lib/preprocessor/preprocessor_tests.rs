#[allow(unused_imports)]
use super::preprocessor;

#[test]
fn test_data_directives() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = preprocessor::PreprocessorParser::new();
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
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "macro mcname (a) ->DB [a]<- \nmcname(5)",
    );
    assert!(o.is_ok());
    assert_eq!(out.data.len(), 1);
    out.clear();
    ctx.clear();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "macro mcname (a) -> DB [a] <-\nmcname(hello)",
    );
    assert!(o.is_err());
    assert_eq!(out.data.len(), 0);
}

#[test]
fn test_control_opcode() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "stc CMC HLT");
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
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "JMP _test JGE go");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 2);
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "RET INTO");
    assert!(o.is_err());
}

#[test]
fn test_procedures() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "def f { STI CMC }");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 3); // One extra for added ret
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "def f { STI CMC } CALL f");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 4); // One extra for added ret
}

#[test]
fn test_offset() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "DB [0;5] name: DB [2] DB OFFSET name");
    assert!(o.is_ok());
    assert_eq!(out.data.len(), 3);
    assert_eq!(out.data[2], "db 5");
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "def f { STI CMC } DB offset f");
    assert!(o.is_err());
}

#[test]
fn test_not() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "NOT AX NOT byte [BX]");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 2);
    let o = p.parse(&mut ctx, &mut out, "NOT byte [BP,SI] not word [bx,DI,-6]");
    assert!(o.is_ok());
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "l:DB 5 NOT BYTE l");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 1);
}

#[test]
fn test_binary_logical() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "AND AX,CX OR AL, BYTE [BX] XOR WORD [BP], AX",
    );
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 3);

    let o = p.parse(&mut ctx, &mut out, "OR AX,0x52");
    assert!(o.is_ok());
    let o = p.parse(&mut ctx, &mut out, "l:DB 8 OR BYTE l,0x52");
    assert!(o.is_ok());

    let o = p.parse(&mut ctx, &mut out, "OR AL,[BX]");
    assert!(o.is_err());
    let o = p.parse(&mut ctx, &mut out, "OR [BP],[BX]");
    assert!(o.is_err());
}

#[test]
fn test_shift_rotate() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "l:DB 6 SAL AX,5 SHL CX,CL SAR byte [BP],CL SHR BYTE l, 0b1101",
    );
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 4);
}

#[test]
fn test_print() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "l:DB [7;0] _t:DB 0 print flags print reg print mem 0x0FFFF-> 0x100FF print mem 0xFFF00:50 print mem : offset _t",
    );
    assert!(o.is_ok());
    assert_eq!(out.data.len(), 2);
    assert_eq!(out.code.len(), 5);
    let o = p.parse(&mut ctx, &mut out, "print mem 0xFFFFF:50");
    assert!(o.is_err());
}

#[test]
fn test_arithmetic() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "l:DB [7;0] _t:DB 0 ADD AX , -5 IMUL CX DIV byte l DAA CBW",
    );
    assert!(o.is_ok());
    assert_eq!(out.data.len(), 2);
    assert_eq!(out.code.len(), 5);
}

#[test]
fn test_string_instructions() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "MOVS byte REP LODS word");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 2);
    let o = p.parse(&mut ctx, &mut out, "MOVS byte REP LODS word rep cmps byte");
    assert!(o.is_err());
    let o = p.parse(&mut ctx, &mut out, "REPZ cmps byte");
    assert!(o.is_ok());
}

#[test]
fn test_data_transfer_unary() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "LAHF popf xlat");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 3);
}

#[test]
fn test_data_transfer_load() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "l:DW 0 LEA AX , word l lea dx , word [SI,7]",
    );
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 2);
    let o = p.parse(&mut ctx, &mut out, "LES");
    assert!(o.is_err());
    let o = p.parse(&mut ctx, &mut out, "LDS");
    assert!(o.is_err());
}

#[test]
fn test_data_transfer_push_pop() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "l:DW [5;7] push CS push word l");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 2);
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "l:DW [5;7] pop ES pop word l");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 2);
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "pop CS");
    assert!(o.is_err());
}

#[test]
fn test_data_transfer_xchg_in_out() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(&mut ctx, &mut out, "l:DW 5 xchg AL,CL xchg word l, si");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 2);
    out.clear();
    ctx.clear();
    let o = p.parse(&mut ctx, &mut out, "in AX,0x51");
    assert!(o.is_err());
    let o = p.parse(&mut ctx, &mut out, "out 0x51,AX");
    assert!(o.is_err());
}

#[test]
fn test_data_transfer_mov() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "l:DW 5 mov AX,CX mov DL,CL mov DX,word l",
    );
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 3);
    out.clear();
    ctx.clear();

    let o = p.parse(&mut ctx, &mut out, "l:DW 5 mov word l, 0x5FFF");
    assert!(o.is_ok());
    assert_eq!(out.code.len(), 1);
    out.clear();
    ctx.clear();
}

#[test]
fn test_macro() {
    let mut ctx = crate::util::preprocessor_util::Context::default();
    let mut out = crate::util::preprocessor_util::Output::default();
    let p = crate::preprocessor::preprocessor::PreprocessorParser::new();
    let o = p.parse(
        &mut ctx,
        &mut out,
        "MACRO a(_)-> c(_) <- MACRO b(_) ->c(_)<- macro c(_) ->a(_)b(_)<- c(_)",
    );
    assert!(o.is_err());
    let o = p.parse(
        &mut ctx,
        &mut out,
        "MACRO a(q)-> ADD AX,q <- MACRO b(k,q) -> k (q)<- b(b,5)",
    );
    assert!(o.is_err());
    let o = p.parse(
        &mut ctx,
        &mut out,
        "MACRO a(q)-> ADD AX,q <- MACRO b(k) ->a(k)<- b(5)",
    );
    assert!(o.is_ok());

    let o = p.parse(
        &mut ctx,
        &mut out,
        "MACRO a(q)-> ADD AX,q <- MACRO b(k,q) -> k (q)<- b(a,5)",
    );
    assert!(o.is_ok());
}
