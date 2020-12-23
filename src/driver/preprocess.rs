use super::error_helper::get_err_pos;
use emulator_8086_lib as lib;
use lalrpop_util::ParseError;
use lib::LexerHelper;
use lib::{Preprocessor, PreprocessorContext, PreprocessorOutput};

/// Function run the preprocessor and return the generated result
/// Returns Result
///     Ok contains Lexer helper for the input, Preprocessor context, Preprocessor output
///     Err contains error string
pub fn preprocess(
    input: &str,
) -> Result<(LexerHelper, PreprocessorContext, PreprocessorOutput), String> {
    // create parameters
    let mut ctx = PreprocessorContext::default();
    let mut out = PreprocessorOutput::default();
    // create preprocessor
    let preprocessor = Preprocessor::new();
    let helper = LexerHelper::new(input);

    // parse
    match preprocessor.parse(&mut ctx, &mut out, input) {
        Err(e) => {
            // if error type is UnrecognizedToken,
            // it can be actual unknown token, or the piggybacked custom error
            if let ParseError::UnrecognizedToken {
                token: (ref start, ref token, _),
                ref expected,
            } = e
            {
                // get error position
                let (line, lstart, lend) = get_err_pos(&helper, *start);
                let pos_str = format!("{}:{} : {}", line, lend - start, &input[lstart..lend]);

                if token.1 == "" {
                    // error is custom, piggybacked on UnrecognizedToken type
                    return Err(format!("Syntax Error at {} :\n{}", pos_str, expected[0]));
                } else {
                    // actual unrecognized token
                    return Err(format!(
                        "Syntax Error at {} :\nUnexpected Token : {}",
                        pos_str, token
                    ));
                }
            } else {
                // some other type of error
                return Err(format!("Syntax Error :\n{}", e));
            };
        }
        Ok(_) => {
            return Ok((helper, ctx, out));
        }
    }
}
