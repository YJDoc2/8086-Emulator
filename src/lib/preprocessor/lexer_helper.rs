/// A Helper to map the character Index to newlines
/// Used as helper, because LALRPOP does not directly give line number, but character number
/// So this is used to initially go over complete input and store occurrences of newlines
/// which can later be used to check the newline that occurred before given character
#[derive(Default)]
pub struct LexerHelper {
    pub temp_line: usize,
    newline_list: Vec<usize>,
}

impl LexerHelper {
    /// Create new Lexer Helper, by scanning input and storing occurrences of newlines
    /// Arguments
    /// input : &str which is input
    pub fn new(input: &str) -> Self {
        let mut l = LexerHelper::default();
        for (i, c) in input.chars().enumerate() {
            if c == '\n' {
                l.newline_list.push(i);
            }
        }
        l
    }

    /// Function to get the number of newline char, which will correspond to line number,
    /// and the place of the newline char.
    /// Arguments :
    /// i : u16 place before which the newline is to be checked
    /// Returns :
    /// (line number, newline char index) : (u16,u16)
    pub fn get_newline_before(&self, i: usize) -> (usize, usize) {
        for (idx, v) in self.newline_list.iter().enumerate() {
            if *v > i {
                return (idx, *v);
            }
        }
        let max = self.newline_list.len();
        return (max, self.newline_list[max - 1]);
    }

    /// get bounds of line containing the given position
    /// returns character position of character after the newline character which is before given character
    /// and newline character after the given character
    pub fn get_bounds(&self, pos: usize) -> (usize, usize) {
        let mut i = 0;
        for (idx, v) in self.newline_list.iter().enumerate() {
            if *v > pos {
                break;
            }
            i = idx;
        }
        // error on first line, so we return 0-> first newline
        if i == 0 {
            return (0, self.newline_list[0]);
        } else {
            // else we return from the last newline char position + 1 -> newline char at error line position
            // 1 is added to last newline position to skip the newline char itself
            return (self.newline_list[i - 1] + 1, self.newline_list[i]);
        }
    }
}

#[test]
fn test_lexer_helper() {
    let input = "5\nabc\ntest9";
    let lh = LexerHelper::new(input);
    assert_eq!(lh.get_newline_before(6), (2, 5)); // 2nd newline, at 5th position (0 based)

    let input = "5\n\n5F\natestb";
    let lh = LexerHelper::new(input);
    assert_eq!(lh.get_newline_before(18), (3, 5)); // 3rd newline, at 5th position (0 based)
}
