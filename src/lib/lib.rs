pub mod arch;
pub mod data_parser;
pub mod preprocessor;

pub mod interpreter;
pub mod util;
pub mod vm;

/// Helper macro for generating errors
/// as LALRPOP does not support user error of types other than &str, and does not have position reporting in it out of hte box
/// We use UnrecognizedToken error as out error
/// This macro generates that error based on  :
/// start and end position of toke : usize
/// tok : actual token &str
/// err : Error String
#[macro_export]
macro_rules! error {
    (  $s:expr,$e:expr,$err:expr ) => {{
        Err(ParseError::UnrecognizedToken {
            token: ($s, Token(0, ""), $e),
            expected: vec![$err],
        })
    }};
}
