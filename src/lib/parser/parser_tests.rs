use super::{lexer, parser};

// TODO
// do original lexer test into parser + assembler

macro_rules! lex {
    ($str:expr) => {{
        let lx = lexer::Lexer::new($str.to_string());
        lx.lex()
    }};
}

macro_rules! parse {
    ($str:expr) => {{
        let tokens = lex!($str).unwrap();
    }};
}

#[test]
fn test_macro_parse() {
    let tokens = lex!("macro macname (a) -> DB <-\n").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_ok());
    assert_eq!(p.ctx.macros.len(), 1);
    assert_eq!(p.ctx.macros[0].name, "macname");
    assert_eq!(p.ctx.macros[0].params.len(), 1);
    assert_eq!(p.ctx.macros[0].params[0], "a");
    assert_eq!(p.ctx.macros[0].code.len(), 1);

    let tokens = lex!("macro macname () -> DB <-").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_ok());
    assert_eq!(p.ctx.macros.len(), 1);
    assert_eq!(p.ctx.macros[0].params.len(), 0);

    let tokens = lex!("macro macname (a,b,c) -> DB <-\n").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_ok());
    assert_eq!(p.ctx.macros.len(), 1);
    assert_eq!(p.ctx.macros[0].params.len(), 3);

    let tokens =
        lex!("macro macname (a) -> DB \"abc\" ;\nADD AX,BX ;hello comment\n <-\n").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_ok());
    assert_eq!(p.ctx.macros.len(), 1);
    assert_eq!(p.ctx.macros[0].code.len(), 8);

    let tokens = lex!("macro macname1 ()-><-;\nmacro macname2 (a) -> DB a<-;\n").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_ok());
    assert_eq!(p.ctx.macros.len(), 2);
    assert_eq!(p.ctx.macros[0].params.len(), 0);
    assert_eq!(p.ctx.macros[1].params.len(), 1);
}

#[test]
// Ideally we should also check error count and error message,
// but for now we are onl checking that it returns error, and does not panic
fn test_macro_parse_errors() {
    let tokens = lex!("macro macname (a,b,c)").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_err());

    let tokens = lex!("macro macname (a,b,c").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_err());

    let tokens = lex!("macro macname (a,b,c) ->\n").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_err());

    // NOTE that because we sync at newlines, we cannot do error recovery after newlines,
    // i.e. next lines will generate unexpected extra errors
    let tokens = lex!("macro macname (a,\n\n").unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_err());

    // check sync recovery
    let tokens = lex!(
        "macro macname1 ()-><-;\nmacro macname2 (a -> DB a<-;\nmacro macname3 (b,c)->DW DB<-;"
    )
    .unwrap();
    let mut p = parser::Parser::new(tokens);
    let parsed = p.parse();
    assert!(parsed.is_err());
    assert_eq!(p.ctx.macros.len(), 2);
    assert_eq!(p.ctx.macros[0].params.len(), 0);
    assert_eq!(p.ctx.macros[0].name, "macname1");
    assert_eq!(p.ctx.macros[1].params.len(), 2);
    assert_eq!(p.ctx.macros[1].name, "macname3");
}
