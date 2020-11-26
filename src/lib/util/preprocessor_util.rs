use std::collections::HashMap;

#[derive(Default)]
pub struct Context {
    pub label_map: HashMap<String, u16>,
    pub macro_map: HashMap<String, String>,
    pub source_map: HashMap<u16, u16>,
}

#[macro_export]
macro_rules! preprocessor_error {
    (  $s:expr,$e:expr,$tok:expr,$err:expr ) => {{
        Err(ParseError::UnrecognizedToken {
            token: ($s, Token($tok.len(), $tok), $e),
            expected: vec![$err],
        })
    }};
}
