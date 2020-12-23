use emulator_8086_lib as lib;
use lib::LexerHelper;

/// Helper for displaying errors
/// Returns line number, start of the line, and end of the line
pub fn get_err_pos(l: &LexerHelper, pos: usize) -> (usize, usize, usize) {
    let (line, _) = l.get_newline_before(pos);
    let (start, end) = l.get_bounds(pos);
    // line is 0 based, so +1
    (line + 1, start, end)
}
