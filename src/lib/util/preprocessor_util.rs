use std::collections::HashMap;

pub enum LabelType {
    DATA,
    CODE,
}

/// This will provide various needed data structures to store
/// metadata about code to preprocessor
/// label_map is for mapping label names to the (position in input ,position in output produced)
/// macro_map is for internal use, for storing macros at processing time, so they can be used for substitution later
/// source_map does reverse mapping of line number in output to in original code
#[derive(Default)]
pub struct Context {
    pub data_counter: u16,
    pub label_map: HashMap<String, (LabelType, u16, u16)>,
    pub macro_map: HashMap<String, String>,
    pub source_map: HashMap<u16, u16>,
}

impl Context {
    pub fn clear(&mut self) {
        self.data_counter = 0;
        self.label_map.clear();
        self.macro_map.clear();
        self.source_map.clear();
    }
}
// TODO add how both are used in the three stages
/// This is used to store the output of preprocessor
/// data stores data commands
/// code stores code commands
#[derive(Default, Debug)]
pub struct Output {
    pub data: Vec<String>,
    pub code: Vec<String>,
}

impl Output {
    pub fn clear(&mut self) {
        self.data.clear();
        self.code.clear();
    }
}

/// Helper macro for generating errors in preprocessor
/// as LALRPOP does not support user error of types other than &str, and does not have position reporting in it out of hte box
/// We use UnrecognizedToken error as out error
/// This macro generates that error based on  :
/// start and end position of toke : usize
/// tok : actual token &str
/// err : Error String
#[macro_export]
macro_rules! preprocessor_error {
    (  $s:expr,$e:expr,$tok:expr,$err:expr ) => {{
        Err(ParseError::UnrecognizedToken {
            token: ($s, Token($tok.len(), $tok), $e),
            expected: vec![$err],
        })
    }};
}
