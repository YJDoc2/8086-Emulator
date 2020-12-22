pub mod arch;
pub mod data_parser;
pub mod instructions;
pub mod interpreter;
pub mod preprocessor;
pub mod util;
pub mod vm;

pub use data_parser::data_parser::DataParser;
pub use interpreter::interpreter::InterpreterParser as Interpreter;
pub use preprocessor::lexer_helper::LexerHelper;
pub use preprocessor::preprocessor::PreprocessorParser as Preprocessor;
pub use util::flag_util::{get_flag_state, Flags};
pub use util::interpreter_util::{Context as InterpreterContext, State};
pub use util::preprocessor_util::{
    Context as PreprocessorContext, Label, LabelType, Output as PreprocessorOutput, SourceMapper,
};
pub use vm::VM;

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
