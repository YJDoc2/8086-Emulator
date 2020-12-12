use super::preprocessor_util::Label;
use std::collections::HashMap;

/// This Is supposed to be used as return from interpretation of single instruction
/// and should be used by driver to decide next step
#[derive(PartialEq, Eq, Debug)]
pub enum State {
    /// HLT will indicate that interpretation is to be stopped
    HALT,
    /// For print type statement, interpreter will just pass this
    /// state back, and the driver should take appropriate action
    PRINT,
    /// As Jump instructions will only support labeled jumps,
    /// This will return the respective value in label/function map
    JMP(usize),
    /// To indicate next instruction should be given to interpreter
    NEXT,
    /// For interrupts
    INT(u8),
}

/// Context for Interpreter
/// fn_map, mapping function name to the position in code Vec provided by preprocessor
/// label_map mapping label name to Label struct
/// Both of these are to be taken from Context of preprocessor
/// call_stack is used internally for keeping  track of return locations in case of stack smashing
#[derive(Default)]
pub struct Context {
    pub fn_map: HashMap<String, usize>,
    pub label_map: HashMap<String, Label>,
    pub call_stack: Vec<usize>,
}
