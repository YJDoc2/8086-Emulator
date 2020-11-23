use std::collections::HashMap;

/// This Is supposed to be used as return from interpretation of single instruction
pub enum State {
    /// HLT will indicate that interpretation is to be stopped
    HLT,
    /// This will be useful for the `print` instruction (Experimental)
    /// This can provide a consumer independent way to run print,
    /// consumer can decide what to do with data returned
    OUTPUT(Vec<u8>),
    /// As Jump instructions will only support labeled jumps,
    /// This will return the label name
    JMP(String),
    /// To indicate next instruction should be given to interpreter
    NEXT,
}

/// Context for Interpreter
/// This will store maps for label -> line number, macros and functions etc.
/// Experimental, might be changed later
//TODO decide if u16 will be enough for label_map
//TODO decide how to implement macros and functions
//TODO decide how to process var_map, and if it is even required
pub struct Context {
    pub label_map: HashMap<String, u16>,
    pub macro_map: HashMap<String, Vec<String>>,
    pub fn_map: HashMap<String, Vec<String>>,
    pub var_map: HashMap<String, String>,
}
