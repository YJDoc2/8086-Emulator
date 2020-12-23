use super::preprocessor_util::Label;
use std::collections::HashMap;

/// As lalrpop gives error if tried to return function from a token,
/// we define these types so we can use those
pub type ByteOpBinary = fn(&mut crate::vm::VM, u8, u8) -> u8;
pub type WordOpBinary = fn(&mut crate::vm::VM, u16, u16) -> u16;
pub type ByteOpUnary = fn(&mut crate::vm::VM, &mut u8) -> Result<(), ()>;
pub type WordOpUnary = fn(&mut crate::vm::VM, &mut u16) -> Result<(), ()>;

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
    /// For string repeats
    REPEAT,
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

/// Helper function to check if number has even parity
pub fn has_even_parity(v: u8) -> bool {
    // uses the trick from here : https://stackoverflow.com/questions/21617970/how-to-check-if-value-has-even-parity-of-bits-or-odd
    let mut val = v;
    val ^= val >> 4;
    val ^= val >> 2;
    val ^= val >> 1;
    return !val & 1 == 1;
}

#[test]
fn test_even_parity() {
    assert!(has_even_parity(3));
    assert!(!has_even_parity(0b11011010));
    assert!(has_even_parity(0b11011011));
}
