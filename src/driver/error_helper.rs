use emulator_8086_lib as lib;
use lib::LexerHelper;

pub fn get_err_pos(l: &LexerHelper, pos: usize, s: &str) -> (usize, usize, usize, usize) {
    let (line, pos) = l.get_newline_before(pos);
    let (start, end) = l.get_line_bounds(pos, s.len());
    (line, pos, start, end)
}
