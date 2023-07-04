use super::lexer;
use crate::parser::lexer::{NumberKind, NumberType, Token, TokenType, TokenValue};

// TODO
// do original lexer test into parser + assembler

macro_rules! lex {
    ($str:expr) => {{
        let lx = lexer::Lexer::new($str.to_string());
        lx.lex()
    }};
}

macro_rules! token {
    ($line:expr,$offset:expr,$typ:expr) => {
        Token {
            offset: $offset,
            line: $line,
            typ: $typ,
            value: TokenValue::None,
        }
    };
    ($line:expr,$offset:expr,$typ:expr,$val:expr) => {
        Token {
            offset: $offset,
            line: $line,
            typ: $typ,
            value: $val,
        }
    };
}

#[test]
fn test_basic_lexing() {
    let o = lex!("set 0x45 SET 6");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 5);
    assert_eq!(out[0], token!(1, 1, TokenType::Set));
    assert_eq!(
        out[1],
        token!(
            1,
            5,
            TokenType::Number,
            TokenValue::Number {
                value: 69,
                kind: NumberKind::Hexadecimal,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[2], token!(1, 10, TokenType::Set));
    assert_eq!(
        out[3],
        token!(
            1,
            14,
            TokenType::Number,
            TokenValue::Number {
                value: 6,
                kind: NumberKind::Decimal,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[4], token!(1, 15, TokenType::EOF));

    let o = lex!("test: DB 5");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 4);
    assert_eq!(
        out[0],
        token!(
            1,
            1,
            TokenType::Label,
            TokenValue::String("test".to_string())
        )
    );
    assert_eq!(out[1], token!(1, 7, TokenType::DB));
    assert_eq!(
        out[2],
        token!(
            1,
            10,
            TokenType::Number,
            TokenValue::Number {
                value: 5,
                kind: NumberKind::Decimal,
                typ: NumberType::U8
            }
        )
    );

    let o = lex!("DB 5\nDW 0x5");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 6);
    assert_eq!(out[0], token!(1, 1, TokenType::DB));
    assert_eq!(
        out[1],
        token!(
            1,
            4,
            TokenType::Number,
            TokenValue::Number {
                value: 5,
                kind: NumberKind::Decimal,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[2], token!(1, 5, TokenType::EOL));
    assert_eq!(out[3], token!(2, 1, TokenType::DW));
    assert_eq!(
        out[4],
        token!(
            2,
            4,
            TokenType::Number,
            TokenValue::Number {
                value: 5,
                kind: NumberKind::Hexadecimal,
                typ: NumberType::U8
            }
        )
    );

    let o = lex!("dw 0b1110");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 3);
    assert_eq!(out[0], token!(1, 1, TokenType::DW));
    assert_eq!(
        out[1],
        token!(
            1,
            4,
            TokenType::Number,
            TokenValue::Number {
                value: 14,
                kind: NumberKind::Binary,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[2], token!(1, 10, TokenType::EOF));

    let o = lex!("test: DB [5]");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 6);
    assert_eq!(
        out[0],
        token!(
            1,
            1,
            TokenType::Label,
            TokenValue::String("test".to_string())
        )
    );
    assert_eq!(out[1], token!(1, 7, TokenType::DB));
    assert_eq!(out[2], token!(1, 10, TokenType::LeftBracket));
    assert_eq!(
        out[3],
        token!(
            1,
            11,
            TokenType::Number,
            TokenValue::Number {
                value: 5,
                kind: NumberKind::Decimal,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[4], token!(1, 12, TokenType::RightBracket));

    let o = lex!("test: DB [58,6]");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 8);
    assert_eq!(
        out[0],
        token!(
            1,
            1,
            TokenType::Label,
            TokenValue::String("test".to_string())
        )
    );
    assert_eq!(out[1], token!(1, 7, TokenType::DB));
    assert_eq!(out[2], token!(1, 10, TokenType::LeftBracket));
    assert_eq!(
        out[3],
        token!(
            1,
            11,
            TokenType::Number,
            TokenValue::Number {
                value: 58,
                kind: NumberKind::Decimal,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[4], token!(1, 13, TokenType::Comma));
    assert_eq!(
        out[5],
        token!(
            1,
            14,
            TokenType::Number,
            TokenValue::Number {
                value: 6,
                kind: NumberKind::Decimal,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[6], token!(1, 15, TokenType::RightBracket));
}

#[test]
fn test_lexing_spaces() {
    let o = lex!("  mov al,bl\n set 05  \n");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 9);
    assert_eq!(
        out[0],
        token!(1, 3, TokenType::Asm, TokenValue::String("mov".to_string()))
    );
    assert_eq!(
        out[1],
        token!(
            1,
            7,
            TokenType::Identifier,
            TokenValue::String("al".to_string())
        )
    );
    assert_eq!(out[2], token!(1, 9, TokenType::Comma));
    assert_eq!(
        out[3],
        token!(
            1,
            10,
            TokenType::Identifier,
            TokenValue::String("bl".to_string())
        )
    );
    assert_eq!(out[4], token!(1, 12, TokenType::EOL));
    assert_eq!(out[5], token!(2, 2, TokenType::Set));
    assert_eq!(
        out[6],
        token!(
            2,
            6,
            TokenType::Number,
            TokenValue::Number {
                value: 5,
                kind: NumberKind::Decimal,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[7], token!(2, 10, TokenType::EOL));
    assert_eq!(out[8], token!(3, 1, TokenType::EOF));
}

#[test]
fn test_macro_lexing() {
    let o = lex!("macro mcname (a) ->DB [a]<- \nmcname(5)");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 17);
    assert_eq!(out[0], token!(1, 1, TokenType::Macro));
    assert_eq!(
        out[1],
        token!(
            1,
            7,
            TokenType::Identifier,
            TokenValue::String("mcname".to_string())
        )
    );
    assert_eq!(out[2], token!(1, 14, TokenType::LeftParen));
    assert_eq!(
        out[3],
        token!(
            1,
            15,
            TokenType::Identifier,
            TokenValue::String("a".to_string())
        )
    );
    assert_eq!(out[4], token!(1, 16, TokenType::RightParen));
    assert_eq!(out[5], token!(1, 18, TokenType::MacroStart));
    assert_eq!(out[6], token!(1, 20, TokenType::DB));
    assert_eq!(out[7], token!(1, 23, TokenType::LeftBracket));
    assert_eq!(
        out[8],
        token!(
            1,
            24,
            TokenType::Identifier,
            TokenValue::String("a".to_string())
        )
    );
    assert_eq!(out[9], token!(1, 25, TokenType::RightBracket));
    assert_eq!(out[10], token!(1, 26, TokenType::MacroEnd));
    assert_eq!(out[11], token!(1, 29, TokenType::EOL));
    assert_eq!(
        out[12],
        token!(
            2,
            1,
            TokenType::Identifier,
            TokenValue::String("mcname".to_owned())
        )
    );
    assert_eq!(out[13], token!(2, 7, TokenType::LeftParen));
    assert_eq!(
        out[14],
        token!(
            2,
            8,
            TokenType::Number,
            TokenValue::Number {
                value: 5,
                kind: NumberKind::Decimal,
                typ: NumberType::U8
            }
        )
    );
    assert_eq!(out[15], token!(2, 9, TokenType::RightParen));
    assert_eq!(out[16], token!(2, 10, TokenType::EOF));
}

#[test]
fn test_print_lexing() {
    let o = lex!("print mem \nreg flags");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 6);
    assert_eq!(out[0], token!(1, 1, TokenType::Print));
    assert_eq!(out[1], token!(1, 7, TokenType::Mem));
    assert_eq!(out[2], token!(1, 11, TokenType::EOL));
    assert_eq!(out[3], token!(2, 1, TokenType::Reg));
    assert_eq!(out[4], token!(2, 5, TokenType::Flags));
}

#[test]
fn test_comment_lexing() {
    let o = lex!("db ;abcdee\n;111;111\nDW ;");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 4);
    assert_eq!(out[0], token!(1, 1, TokenType::DB));
    assert_eq!(out[1], token!(1, 11, TokenType::EOL));
    assert_eq!(out[2], token!(3, 1, TokenType::DW));
    assert_eq!(out[3], token!(3, 5, TokenType::EOF));
}

#[test]
fn test_negative_number_lexing() {
    let o = lex!("-0xFFFF");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 2);
    assert_eq!(
        out[0],
        token!(
            1,
            1,
            TokenType::Number,
            TokenValue::Number {
                value: 1,
                kind: NumberKind::Hexadecimal,
                typ: NumberType::U16
            }
        )
    );

    let o = lex!("-0b1010");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 2);
    assert_eq!(
        out[0],
        token!(
            1,
            1,
            TokenType::Number,
            TokenValue::Number {
                value: 0xFFF6,
                kind: NumberKind::Binary,
                typ: NumberType::U8
            }
        )
    );
}

#[test]
fn test_lexer_errors() {
    let o = lex!("0xer");
    assert!(o.is_err());
    let err = o.unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].to_string().contains("error in parsing number 'er'"));

    let o = lex!("あ");
    assert!(o.is_err());
    let err = o.unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].to_string().contains("unexpected character 'あ'"));

    let o = lex!("--");
    assert!(o.is_err());
    let err = o.unwrap_err();
    assert_eq!(err.len(), 2); // both - generate their own errors
    assert!(err[0].to_string().contains("unexpected '-'"));
}

#[test]
fn test_multiple_newline_skip() {
    let o = lex!(";abcd\n\n\nDB 5");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 4);
    assert_eq!(out[0], token!(1, 6, TokenType::EOL));
    assert_eq!(out[1], token!(4, 1, TokenType::DB));

    let o = lex!(";abcd\n;abcd  \nDB 5");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 4);
    assert_eq!(out[0], token!(1, 6, TokenType::EOL));
    assert_eq!(out[1], token!(3, 1, TokenType::DB));

    let o = lex!(";abcd\nset \nDB 5");
    assert!(o.is_ok());
    let out = o.unwrap();
    assert_eq!(out.len(), 6);
    assert_eq!(out[0], token!(1, 6, TokenType::EOL));
    assert_eq!(out[1], token!(2, 1, TokenType::Set));
    assert_eq!(out[2], token!(2, 5, TokenType::EOL));
    assert_eq!(out[3], token!(3, 1, TokenType::DB));
}
