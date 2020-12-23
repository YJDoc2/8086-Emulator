use super::error_helper::get_err_pos;
use emulator_8086_lib as lib;
use lalrpop_util::ParseError;
use lib::LexerHelper;
use lib::{Preprocessor, PreprocessorContext, PreprocessorOutput};

pub fn preprocess(
    input: &str,
) -> Result<(LexerHelper, PreprocessorContext, PreprocessorOutput), String> {
    let mut ctx = PreprocessorContext::default();
    let mut out = PreprocessorOutput::default();
    let preprocessor = Preprocessor::new();
    let helper = LexerHelper::new(input);
    match preprocessor.parse(&mut ctx, &mut out, input) {
        Err(e) => {
            if let ParseError::UnrecognizedToken {
                token: (ref start, ref token, _),
                ref expected,
            } = e
            {
                let (line, pos, lstart, lend) = get_err_pos(&helper, *start);
                let pos_str = format!("{}:{} : {}", line, pos - start, &input[lstart..lend]);
                if token.1 == "" {
                    return Err(format!("Syntax Error at {} :\n{}", pos_str, expected[0]));
                } else {
                    return Err(format!(
                        "Syntax Error at {} :\nUnexpected Token : {}",
                        pos_str, token
                    ));
                }
            } else {
                return Err(format!("Syntax Error :\n{}", e));
            };
        }
        Ok(_) => {
            return Ok((helper, ctx, out));
        }
    }
}
