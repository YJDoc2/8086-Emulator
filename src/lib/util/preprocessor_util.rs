use std::collections::{HashMap, HashSet};

pub enum LabelType {
    DATA,
    CODE,
}

pub struct Label {
    pub r#type: LabelType,
    pub source_position: u16,
    pub map: u16,
}

impl Label {
    pub fn new(t: LabelType, pos: u16, map: u16) -> Self {
        Label {
            r#type: t,
            source_position: pos,
            map: map,
        }
    }
}

/// This is helper structure to create source map
/// the source_last and output_next contains last and next entries respectively for source and output code
/// the lock is used to lock from changing the source_last, in cases like macro expansions
/// Not meant for multi threaded use
/// Please read documentation of add_entry method
#[derive(Default)]
pub struct SourceMapper {
    source_last: usize,
    output_next: usize,
    lock: u16,
    source_map: HashMap<usize, usize>,
}

impl SourceMapper {
    /// create a new SourceMapper
    pub fn new() -> Self {
        SourceMapper::default()
    }

    /// increment the lock count
    pub fn lock_source(&mut self) {
        self.lock += 1;
    }

    /// decrement the lock count
    pub fn unlock_source(&mut self) {
        self.lock -= 1;
    }

    /// consume the instance and return source map
    pub fn get_source_map(self) -> HashMap<usize, usize> {
        return self.source_map;
    }

    /// Used to Set the source_last manually, should not be used unless absolutely required,
    /// intended to used for macros, because the source_last at the line of macro invocation will still be pointing
    /// to previous line, and as macro calls the lock, it will map all next entries to the previous line,
    /// so this is used to set the source_last manually
    pub fn set_source(&mut self, source_char: usize) {
        if self.lock != 0 {
            return;
        } else {
            self.source_last = source_char;
        }
    }

    /// Helper function to clear and set source mapper to default
    pub fn clear(&mut self) {
        self.lock = 0;
        self.source_last = 0;
        self.output_next = 0;
        self.source_map.clear();
    }
    /// Adds a output line number -> source char number entry
    /// as the output line number will alway increment by 1, as we are using vector to store it,
    /// this directly increments the output_next by 1 with every entry
    /// This takes in source_char, but if the lock is non-zero i.e.
    /// the lock_source is called, it adds entry of output_next -> source_last
    /// if lock value is zero, it sets the source_last to the source_char,
    /// and adds entry of output_next -> source_char
    /// output_next is always incremented by 1
    pub fn add_entry(&mut self, source_char: usize) {
        let source = if self.lock != 0 {
            self.source_last
        } else {
            self.source_last = source_char;
            source_char
        };
        self.source_map.insert(self.output_next, source);
        self.output_next += 1;
    }
}
#[test]
fn test_source_mapper() {
    let mut sm = SourceMapper::new();
    sm.add_entry(5); // 0
    sm.add_entry(10); // 1
    assert_eq!(sm.lock, 0);
    sm.lock_source();
    assert_eq!(sm.lock, 1);
    sm.add_entry(16); // 2
    sm.add_entry(20); // 3
    sm.lock_source();
    assert_eq!(sm.lock, 2);
    sm.add_entry(23); // 4
    sm.unlock_source();
    assert_eq!(sm.lock, 1);
    sm.unlock_source();
    assert_eq!(sm.lock, 0);
    sm.add_entry(15); // 5
    let t = sm.get_source_map();
    assert_eq!(t.get(&0).unwrap(), &5);
    assert_eq!(t.get(&1).unwrap(), &10);
    assert_eq!(t.get(&2).unwrap(), &10);
    assert_eq!(t.get(&3).unwrap(), &10);
    assert_eq!(t.get(&4).unwrap(), &10);
    assert_eq!(t.get(&5).unwrap(), &15);
}

/// This will provide various needed data structures to store
/// metadata about code to preprocessor
/// label_map is for mapping label names to the (position in input ,position in output produced)
/// macro_map is for internal use, for storing macros at processing time, so they can be used for substitution later
/// source_map does reverse mapping of line number in output to in original code
#[derive(Default)]
pub struct Context {
    pub data_counter: u16,
    pub label_map: HashMap<String, Label>,
    pub macro_map: HashMap<String, String>,
    pub mapper: SourceMapper,
    pub fn_set: HashSet<String>,
    pub undefined_labels: HashSet<String>,
}

impl Context {
    pub fn clear(&mut self) {
        self.data_counter = 0;
        self.label_map.clear();
        self.macro_map.clear();
        self.mapper.clear();
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
