// auto-generated: "lalrpop 0.19.1"
// sha256: b88c37c8baad7380d58e4451f19ad09551b24c86872e593d5586648ae62bb794
use crate::util::preprocessor_util as util;
use util::LabelType;
use regex::{Regex,Captures};
use crate::preprocessor_error;
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Code {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::preprocessor_util as util;
    use util::LabelType;
    use regex::{Regex,Captures};
    use crate::preprocessor_error;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(String),
        Variant2(::std::vec::Vec<String>),
        Variant3(usize),
        Variant4(()),
        Variant5(Vec<String>),
        Variant6(u8),
        Variant7(::std::option::Option<String>),
        Variant8(u16),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 21, 22, 0, 23, 0, 0, 24, 25, 0, 26, 0, 0, 0, 0, 0, 0, 27,
        // State 1
        0, 0, 0, 0, 21, 22, 0, 23, 0, 0, 24, 25, 0, 26, 0, 0, 0, 0, 0, 0, 27,
        // State 2
        0, 0, 0, 0, 21, 22, 0, 0, 0, 0, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 32, 33, 34, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 32, 33, 34, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 33, 34, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 33, 34, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 37, 38, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, -34, -34, 0, -34, 0, 0, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, -34,
        // State 18
        0, 0, 0, 0, -35, -35, 0, -35, 0, 0, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35,
        // State 19
        0, 0, 0, 0, -33, -33, 0, -33, 0, 0, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, -33,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, -61, -61, -61, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, -64, -64, -64, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, -68, -68, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, -62, -62, -62, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, -63, -63, -63, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, -67, -67, 0, 0, 0,
        // State 26
        0, 0, 0, 0, -55, -55, 0, 0, 0, 0, -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, -37, -37, 0, -37, 0, 0, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, -37,
        // State 28
        0, 0, 0, 0, -38, -38, 0, -38, 0, 0, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38,
        // State 29
        0, 0, 0, 0, -36, -36, 0, -36, 0, 0, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36,
        // State 30
        0, 0, 0, 0, -40, -40, 0, -40, 0, 0, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40,
        // State 31
        0, 0, 0, 0, -28, -28, 0, -28, 0, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, -28,
        // State 32
        0, 0, 0, 0, -27, -27, 0, -27, 0, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, -27,
        // State 33
        0, 0, 0, 0, -26, -26, 0, -26, 0, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, -26,
        // State 34
        0, 0, 0, 0, -46, -46, 0, -46, 0, 0, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46,
        // State 35
        0, 0, 0, -73, -73, -73, 0, -73, 0, -73, -73, -73, 0, -73, 0, 0, 0, 0, 0, 0, -73,
        // State 36
        0, 0, 0, -72, -72, -72, 0, -72, 0, -72, -72, -72, 0, -72, 0, 0, 0, 0, 0, 0, -72,
        // State 37
        0, 0, 0, -71, -71, -71, 0, -71, 0, -71, -71, -71, 0, -71, 0, 0, 0, 0, 0, 0, -71,
        // State 38
        0, 0, 0, 0, -70, -70, 0, -70, 0, 0, -70, -70, 0, -70, 0, 0, 0, 0, 0, 0, -70,
        // State 39
        0, 0, 0, 0, -39, -39, 0, -39, 0, 0, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39,
        // State 40
        0, 0, 0, 0, -45, -45, 0, -45, 0, 0, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45,
        // State 41
        0, 0, 0, 13, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 14, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 15, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 16, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, -42, -42, 0, -42, 0, 0, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42,
        // State 46
        0, 0, 0, 0, -48, -48, 0, -48, 0, 0, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48,
        // State 47
        0, 0, 0, 0, -41, -41, 0, -41, 0, 0, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41,
        // State 48
        0, 0, 0, 0, -47, -47, 0, -47, 0, 0, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, -44, -44, 0, -44, 0, 0, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44,
        // State 54
        0, 0, 0, 0, -50, -50, 0, -50, 0, 0, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50,
        // State 55
        0, 0, 0, 0, -43, -43, 0, -43, 0, 0, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43,
        // State 56
        0, 0, 0, 0, -49, -49, 0, -49, 0, 0, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 21 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -13,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        -22,
        // State 17
        -34,
        // State 18
        -35,
        // State 19
        -33,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -37,
        // State 28
        -38,
        // State 29
        -36,
        // State 30
        -40,
        // State 31
        -28,
        // State 32
        -27,
        // State 33
        -26,
        // State 34
        -46,
        // State 35
        -73,
        // State 36
        -72,
        // State 37
        -71,
        // State 38
        -70,
        // State 39
        -39,
        // State 40
        -45,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -42,
        // State 46
        -48,
        // State 47
        -41,
        // State 48
        -47,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        -44,
        // State 54
        -50,
        // State 55
        -43,
        // State 56
        -49,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            8 => 16,
            15 => match state {
                6 => 39,
                12 => 49,
                14 => 51,
                _ => 30,
            },
            17 => 1,
            18 => match state {
                1 => 27,
                _ => 17,
            },
            19 => match state {
                1 => 28,
                _ => 18,
            },
            22 => 2,
            27 => match state {
                2 => 6,
                _ => 3,
            },
            28 => match state {
                2 => 7,
                _ => 4,
            },
            30 => 5,
            32 => match state {
                1 => 29,
                _ => 19,
            },
            33 => match state {
                5 => 38,
                7 => 40,
                8 => 41,
                9 => 42,
                10 => 43,
                11 => 44,
                13 => 50,
                15 => 52,
                _ => 34,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###"";""###,
            r###""DB""###,
            r###""DW""###,
            r###""MACRO""###,
            r###""SET""###,
            r###""[""###,
            r###""]""###,
            r###""db""###,
            r###""dw""###,
            r###""macro""###,
            r###""set""###,
            r###""{""###,
            r###"r#"0(b|B)[0-1]+"#"###,
            r###"r#"0(x|X)[0-9A-Fa-f]+"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[_a-zA-Z0-9 \\[\\]]*}"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*:"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input, 's>
    where 
    {
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input (), &'s ())>,
    }
    impl<'input, 's> __state_machine::ParserDefinition for __StateMachine<'input, 's>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ();
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 21 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.context,
                self.out,
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
        's,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(6, _) if true => Some(0),
            Token(7, _) if true => Some(1),
            Token(8, _) if true => Some(2),
            Token(9, _) if true => Some(3),
            Token(10, _) if true => Some(4),
            Token(11, _) if true => Some(5),
            Token(12, _) if true => Some(6),
            Token(13, _) if true => Some(7),
            Token(14, _) if true => Some(8),
            Token(15, _) if true => Some(9),
            Token(16, _) if true => Some(10),
            Token(17, _) if true => Some(11),
            Token(18, _) if true => Some(12),
            Token(19, _) if true => Some(13),
            Token(20, _) if true => Some(14),
            Token(0, _) if true => Some(15),
            Token(1, _) if true => Some(16),
            Token(2, _) if true => Some(17),
            Token(3, _) if true => Some(18),
            Token(4, _) if true => Some(19),
            Token(5, _) if true => Some(20),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        's,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => match __token {
                Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct CodeParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl CodeParser {
        pub fn new() -> CodeParser {
            let __builder = super::__intern_token::new_builder();
            CodeParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            's,
        >(
            &self,
            context: &'s mut util::Context,
            out: &'s mut util::Output,
            input: &'input str,
        ) -> Result<(), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    context,
                    out,
                    input,
                    __phantom: ::std::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<Result<(),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                // __Code = Code => ActionFn(0);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(context, out, input, __sym0);
                return Some(Ok(__nt));
            }
            22 => {
                __reduce22(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                // byte_num = r#"[0-9]+"# => ActionFn(88);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action88::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            26 => {
                // byte_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(89);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action89::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            27 => {
                // byte_num = r#"0(b|B)[0-1]+"# => ActionFn(90);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action90::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            28 => {
                __reduce28(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                // label = r#"[_a-zA-Z][_a-zA-Z0-9]*:"# => ActionFn(39);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action39::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 22)
            }
            55 => {
                __reduce55(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                // macro_use = r#"[_a-zA-Z][_a-zA-Z0-9]*"#, "(", CommaSepList<general_string>, ")" => ActionFn(91);
                assert!(__symbols.len() >= 4);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant5(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = match super::__action91::<>(context, out, input, __sym0, __sym1, __sym2, __sym3) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (4, 24)
            }
            57 => {
                __reduce57(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                // word_num = r#"[0-9]+"# => ActionFn(92);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action92::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            71 => {
                // word_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(93);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action93::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            72 => {
                // word_num = r#"0(b|B)[0-1]+"# => ActionFn(94);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action94::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u16, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",") = general_string, "," => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")* =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action60::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")* = (<general_string> ",")+ => ActionFn(61);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")+ = general_string, "," => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action67::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")+ = (<general_string> ",")+, general_string, "," => ActionFn(68);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action68::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",") = name_string, "," => ActionFn(57);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action57::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")* = (<name_string> ",")+ => ActionFn(56);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")+ = name_string, "," => ActionFn(71);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action71::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")+ = (<name_string> ",")+, name_string, "," => ActionFn(72);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action72::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Code = data_directives => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = general_string => ActionFn(95);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce14<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> =  => ActionFn(96);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action96::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = (<general_string> ",")+, general_string => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = (<general_string> ",")+ => ActionFn(98);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = name_string => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> =  => ActionFn(100);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce19<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = (<name_string> ",")+, name_string => ActionFn(101);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action101::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce20<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = (<name_string> ",")+ => ActionFn(102);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce22<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __code_directives = code_directives => ActionFn(2);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce23<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __data_directives = data_directives => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce24<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __macro_def = macro_def => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce28<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = macro_def => ActionFn(30);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce29<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = macro_use => ActionFn(31);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce30<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = code_directives, macro_def => ActionFn(32);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce31<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = code_directives, macro_use => ActionFn(33);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce32<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = set_directive => ActionFn(5);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce33<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = db_directive => ActionFn(6);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce34<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = dw_directive => ActionFn(7);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce35<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, set_directive => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce36<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, db_directive => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce37<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, dw_directive => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce38<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, byte_num => ActionFn(78);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action78::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce39<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, byte_num => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce40<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, "[", word_num, "]" => ActionFn(79);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action79::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 18)
    }
    pub(crate) fn __reduce41<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, "[", word_num, "]" => ActionFn(17);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action17::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 18)
    }
    pub(crate) fn __reduce42<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, "[", word_num, ";", byte_num, "]" => ActionFn(80);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action80::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 18)
    }
    pub(crate) fn __reduce43<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, "[", word_num, ";", byte_num, "]" => ActionFn(19);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action19::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 18)
    }
    pub(crate) fn __reduce44<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, word_num => ActionFn(81);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action81::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce45<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, word_num => ActionFn(23);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce46<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, "[", word_num, "]" => ActionFn(82);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action82::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 19)
    }
    pub(crate) fn __reduce47<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, "[", word_num, "]" => ActionFn(25);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action25::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 19)
    }
    pub(crate) fn __reduce48<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, "[", word_num, ";", word_num, "]" => ActionFn(83);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action83::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 19)
    }
    pub(crate) fn __reduce49<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, "[", word_num, ";", word_num, "]" => ActionFn(27);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action27::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 19)
    }
    pub(crate) fn __reduce50<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string = name_string => ActionFn(47);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce51<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string = word_num => ActionFn(48);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce52<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string? = general_string => ActionFn(58);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce53<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string? =  => ActionFn(59);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action59::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce55<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // macro_def = quote_macro, name_string, "(", CommaSepList<name_string>, ")", "{", raw_code => ActionFn(34);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action34::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 23)
    }
    pub(crate) fn __reduce57<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce58<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string? = name_string => ActionFn(53);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce59<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 26)
    }
    pub(crate) fn __reduce60<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_db = "DB" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce61<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_db = "db" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce62<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_dw = "dw" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce63<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_dw = "DW" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce64<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_macro = "MACRO" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce65<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_macro = "macro" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce66<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_set = "set" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce67<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_set = "SET" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce68<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // raw_code = r#"[_a-zA-Z0-9 \\[\\]]*}"# => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce69<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // set_directive = quote_set, word_num => ActionFn(11);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action11::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 32)
    }
}
pub use self::__parse__Code::CodeParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__code_directives {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::preprocessor_util as util;
    use util::LabelType;
    use regex::{Regex,Captures};
    use crate::preprocessor_error;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(String),
        Variant2(::std::vec::Vec<String>),
        Variant3(usize),
        Variant4(()),
        Variant5(Vec<String>),
        Variant6(u8),
        Variant7(::std::option::Option<String>),
        Variant8(u16),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 3
        0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 0, 17, 0,
        // State 4
        0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 5
        0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 0, 17, 0,
        // State 6
        0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, -29, 0,
        // State 9
        0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, -30, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0,
        // State 12
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, -31, 0,
        // State 14
        0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, -32, 0,
        // State 15
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, -14, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, -52, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, -73, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -72, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -71, -71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -18, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -16, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, -57, 0,
        // State 28
        0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, 0, -4, 0,
        // State 29
        0, -20, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0,
        // State 32
        0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, 0, -5, 0,
        // State 33
        0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0,
        // State 34
        0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, -56, 0,
        // State 35
        0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, -69, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 21 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -23,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        -29,
        // State 9
        -30,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        -31,
        // State 14
        -32,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -57,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -56,
        // State 35
        -69,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 5,
            5 => 6,
            9 => 17,
            10 => 24,
            16 => 1,
            20 => match state {
                5 => 26,
                _ => 18,
            },
            23 => match state {
                1 => 13,
                _ => 8,
            },
            24 => match state {
                1 => 14,
                _ => 9,
            },
            25 => match state {
                2 => 15,
                4 => 25,
                6 => 29,
                _ => 19,
            },
            29 => 2,
            31 => 34,
            33 => 20,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###"";""###,
            r###""DB""###,
            r###""DW""###,
            r###""MACRO""###,
            r###""SET""###,
            r###""[""###,
            r###""]""###,
            r###""db""###,
            r###""dw""###,
            r###""macro""###,
            r###""set""###,
            r###""{""###,
            r###"r#"0(b|B)[0-1]+"#"###,
            r###"r#"0(x|X)[0-9A-Fa-f]+"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[_a-zA-Z0-9 \\[\\]]*}"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*:"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input, 's>
    where 
    {
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input (), &'s ())>,
    }
    impl<'input, 's> __state_machine::ParserDefinition for __StateMachine<'input, 's>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ();
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 21 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.context,
                self.out,
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
        's,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(6, _) if true => Some(0),
            Token(7, _) if true => Some(1),
            Token(8, _) if true => Some(2),
            Token(9, _) if true => Some(3),
            Token(10, _) if true => Some(4),
            Token(11, _) if true => Some(5),
            Token(12, _) if true => Some(6),
            Token(13, _) if true => Some(7),
            Token(14, _) if true => Some(8),
            Token(15, _) if true => Some(9),
            Token(16, _) if true => Some(10),
            Token(17, _) if true => Some(11),
            Token(18, _) if true => Some(12),
            Token(19, _) if true => Some(13),
            Token(20, _) if true => Some(14),
            Token(0, _) if true => Some(15),
            Token(1, _) if true => Some(16),
            Token(2, _) if true => Some(17),
            Token(3, _) if true => Some(18),
            Token(4, _) if true => Some(19),
            Token(5, _) if true => Some(20),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        's,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => match __token {
                Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct code_directivesParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl code_directivesParser {
        pub fn new() -> code_directivesParser {
            let __builder = super::__intern_token::new_builder();
            code_directivesParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            's,
        >(
            &self,
            context: &'s mut util::Context,
            out: &'s mut util::Output,
            input: &'input str,
        ) -> Result<(), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    context,
                    out,
                    input,
                    __phantom: ::std::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<Result<(),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                // __code_directives = code_directives => ActionFn(2);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(context, out, input, __sym0);
                return Some(Ok(__nt));
            }
            23 => {
                __reduce23(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                // byte_num = r#"[0-9]+"# => ActionFn(88);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action88::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            26 => {
                // byte_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(89);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action89::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            27 => {
                // byte_num = r#"0(b|B)[0-1]+"# => ActionFn(90);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action90::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            28 => {
                __reduce28(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                // label = r#"[_a-zA-Z][_a-zA-Z0-9]*:"# => ActionFn(39);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action39::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 22)
            }
            55 => {
                __reduce55(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                // macro_use = r#"[_a-zA-Z][_a-zA-Z0-9]*"#, "(", CommaSepList<general_string>, ")" => ActionFn(91);
                assert!(__symbols.len() >= 4);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant5(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = match super::__action91::<>(context, out, input, __sym0, __sym1, __sym2, __sym3) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (4, 24)
            }
            57 => {
                __reduce57(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                // word_num = r#"[0-9]+"# => ActionFn(92);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action92::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            71 => {
                // word_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(93);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action93::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            72 => {
                // word_num = r#"0(b|B)[0-1]+"# => ActionFn(94);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action94::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u16, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",") = general_string, "," => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")* =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action60::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")* = (<general_string> ",")+ => ActionFn(61);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")+ = general_string, "," => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action67::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")+ = (<general_string> ",")+, general_string, "," => ActionFn(68);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action68::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",") = name_string, "," => ActionFn(57);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action57::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")* = (<name_string> ",")+ => ActionFn(56);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")+ = name_string, "," => ActionFn(71);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action71::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")+ = (<name_string> ",")+, name_string, "," => ActionFn(72);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action72::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Code = data_directives => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = general_string => ActionFn(95);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce14<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> =  => ActionFn(96);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action96::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = (<general_string> ",")+, general_string => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = (<general_string> ",")+ => ActionFn(98);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = name_string => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> =  => ActionFn(100);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce19<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = (<name_string> ",")+, name_string => ActionFn(101);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action101::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce20<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = (<name_string> ",")+ => ActionFn(102);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce21<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __Code = Code => ActionFn(0);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce23<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __data_directives = data_directives => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce24<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __macro_def = macro_def => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce28<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = macro_def => ActionFn(30);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce29<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = macro_use => ActionFn(31);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce30<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = code_directives, macro_def => ActionFn(32);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce31<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = code_directives, macro_use => ActionFn(33);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce32<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = set_directive => ActionFn(5);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce33<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = db_directive => ActionFn(6);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce34<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = dw_directive => ActionFn(7);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce35<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, set_directive => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce36<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, db_directive => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce37<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, dw_directive => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce38<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, byte_num => ActionFn(78);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action78::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce39<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, byte_num => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce40<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, "[", word_num, "]" => ActionFn(79);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action79::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 18)
    }
    pub(crate) fn __reduce41<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, "[", word_num, "]" => ActionFn(17);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action17::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 18)
    }
    pub(crate) fn __reduce42<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, "[", word_num, ";", byte_num, "]" => ActionFn(80);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action80::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 18)
    }
    pub(crate) fn __reduce43<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, "[", word_num, ";", byte_num, "]" => ActionFn(19);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action19::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 18)
    }
    pub(crate) fn __reduce44<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, word_num => ActionFn(81);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action81::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce45<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, word_num => ActionFn(23);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce46<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, "[", word_num, "]" => ActionFn(82);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action82::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 19)
    }
    pub(crate) fn __reduce47<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, "[", word_num, "]" => ActionFn(25);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action25::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 19)
    }
    pub(crate) fn __reduce48<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, "[", word_num, ";", word_num, "]" => ActionFn(83);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action83::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 19)
    }
    pub(crate) fn __reduce49<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, "[", word_num, ";", word_num, "]" => ActionFn(27);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action27::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 19)
    }
    pub(crate) fn __reduce50<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string = name_string => ActionFn(47);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce51<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string = word_num => ActionFn(48);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce52<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string? = general_string => ActionFn(58);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce53<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string? =  => ActionFn(59);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action59::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce55<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // macro_def = quote_macro, name_string, "(", CommaSepList<name_string>, ")", "{", raw_code => ActionFn(34);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action34::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 23)
    }
    pub(crate) fn __reduce57<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce58<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string? = name_string => ActionFn(53);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce59<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 26)
    }
    pub(crate) fn __reduce60<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_db = "DB" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce61<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_db = "db" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce62<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_dw = "dw" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce63<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_dw = "DW" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce64<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_macro = "MACRO" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce65<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_macro = "macro" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce66<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_set = "set" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce67<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_set = "SET" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce68<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // raw_code = r#"[_a-zA-Z0-9 \\[\\]]*}"# => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce69<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // set_directive = quote_set, word_num => ActionFn(11);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action11::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 32)
    }
}
pub use self::__parse__code_directives::code_directivesParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__data_directives {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::preprocessor_util as util;
    use util::LabelType;
    use regex::{Regex,Captures};
    use crate::preprocessor_error;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(String),
        Variant2(::std::vec::Vec<String>),
        Variant3(usize),
        Variant4(()),
        Variant5(Vec<String>),
        Variant6(u8),
        Variant7(::std::option::Option<String>),
        Variant8(u16),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 20, 21, 0, 22, 0, 0, 23, 24, 0, 25, 0, 0, 0, 0, 0, 0, 26,
        // State 1
        0, 0, 0, 0, 20, 21, 0, 22, 0, 0, 23, 24, 0, 25, 0, 0, 0, 0, 0, 0, 26,
        // State 2
        0, 0, 0, 0, 20, 21, 0, 0, 0, 0, 23, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 31, 32, 33, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 31, 32, 33, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32, 33, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 32, 33, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 36, 37, 0, 0, 0,
        // State 16
        0, 0, 0, 0, -34, -34, 0, -34, 0, 0, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, -34,
        // State 17
        0, 0, 0, 0, -35, -35, 0, -35, 0, 0, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, -35,
        // State 18
        0, 0, 0, 0, -33, -33, 0, -33, 0, 0, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, -33,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, -61, -61, -61, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, -64, 0, 0, 0, 0, 0, 0, -64, -64, -64, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -68, -68, -68, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, -62, 0, 0, 0, 0, 0, 0, -62, -62, -62, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, -63, 0, 0, 0, 0, 0, 0, -63, -63, -63, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -67, -67, -67, 0, 0, 0,
        // State 25
        0, 0, 0, 0, -55, -55, 0, 0, 0, 0, -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, -37, -37, 0, -37, 0, 0, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, -37,
        // State 27
        0, 0, 0, 0, -38, -38, 0, -38, 0, 0, -38, -38, 0, -38, 0, 0, 0, 0, 0, 0, -38,
        // State 28
        0, 0, 0, 0, -36, -36, 0, -36, 0, 0, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, -36,
        // State 29
        0, 0, 0, 0, -40, -40, 0, -40, 0, 0, -40, -40, 0, -40, 0, 0, 0, 0, 0, 0, -40,
        // State 30
        0, 0, 0, 0, -28, -28, 0, -28, 0, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, -28,
        // State 31
        0, 0, 0, 0, -27, -27, 0, -27, 0, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, -27,
        // State 32
        0, 0, 0, 0, -26, -26, 0, -26, 0, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, -26,
        // State 33
        0, 0, 0, 0, -46, -46, 0, -46, 0, 0, -46, -46, 0, -46, 0, 0, 0, 0, 0, 0, -46,
        // State 34
        0, 0, 0, -73, -73, -73, 0, -73, 0, -73, -73, -73, 0, -73, 0, 0, 0, 0, 0, 0, -73,
        // State 35
        0, 0, 0, -72, -72, -72, 0, -72, 0, -72, -72, -72, 0, -72, 0, 0, 0, 0, 0, 0, -72,
        // State 36
        0, 0, 0, -71, -71, -71, 0, -71, 0, -71, -71, -71, 0, -71, 0, 0, 0, 0, 0, 0, -71,
        // State 37
        0, 0, 0, 0, -70, -70, 0, -70, 0, 0, -70, -70, 0, -70, 0, 0, 0, 0, 0, 0, -70,
        // State 38
        0, 0, 0, 0, -39, -39, 0, -39, 0, 0, -39, -39, 0, -39, 0, 0, 0, 0, 0, 0, -39,
        // State 39
        0, 0, 0, 0, -45, -45, 0, -45, 0, 0, -45, -45, 0, -45, 0, 0, 0, 0, 0, 0, -45,
        // State 40
        0, 0, 0, 13, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 14, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 15, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 16, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, -42, -42, 0, -42, 0, 0, -42, -42, 0, -42, 0, 0, 0, 0, 0, 0, -42,
        // State 45
        0, 0, 0, 0, -48, -48, 0, -48, 0, 0, -48, -48, 0, -48, 0, 0, 0, 0, 0, 0, -48,
        // State 46
        0, 0, 0, 0, -41, -41, 0, -41, 0, 0, -41, -41, 0, -41, 0, 0, 0, 0, 0, 0, -41,
        // State 47
        0, 0, 0, 0, -47, -47, 0, -47, 0, 0, -47, -47, 0, -47, 0, 0, 0, 0, 0, 0, -47,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, -44, -44, 0, -44, 0, 0, -44, -44, 0, -44, 0, 0, 0, 0, 0, 0, -44,
        // State 53
        0, 0, 0, 0, -50, -50, 0, -50, 0, 0, -50, -50, 0, -50, 0, 0, 0, 0, 0, 0, -50,
        // State 54
        0, 0, 0, 0, -43, -43, 0, -43, 0, 0, -43, -43, 0, -43, 0, 0, 0, 0, 0, 0, -43,
        // State 55
        0, 0, 0, 0, -49, -49, 0, -49, 0, 0, -49, -49, 0, -49, 0, 0, 0, 0, 0, 0, -49,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 21 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -24,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        -34,
        // State 17
        -35,
        // State 18
        -33,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        -37,
        // State 27
        -38,
        // State 28
        -36,
        // State 29
        -40,
        // State 30
        -28,
        // State 31
        -27,
        // State 32
        -26,
        // State 33
        -46,
        // State 34
        -73,
        // State 35
        -72,
        // State 36
        -71,
        // State 37
        -70,
        // State 38
        -39,
        // State 39
        -45,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        -42,
        // State 45
        -48,
        // State 46
        -41,
        // State 47
        -47,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        -44,
        // State 53
        -50,
        // State 54
        -43,
        // State 55
        -49,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            15 => match state {
                6 => 38,
                12 => 48,
                14 => 50,
                _ => 29,
            },
            17 => 1,
            18 => match state {
                1 => 26,
                _ => 16,
            },
            19 => match state {
                1 => 27,
                _ => 17,
            },
            22 => 2,
            27 => match state {
                2 => 6,
                _ => 3,
            },
            28 => match state {
                2 => 7,
                _ => 4,
            },
            30 => 5,
            32 => match state {
                1 => 28,
                _ => 18,
            },
            33 => match state {
                5 => 37,
                7 => 39,
                8 => 40,
                9 => 41,
                10 => 42,
                11 => 43,
                13 => 49,
                15 => 51,
                _ => 33,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###"";""###,
            r###""DB""###,
            r###""DW""###,
            r###""MACRO""###,
            r###""SET""###,
            r###""[""###,
            r###""]""###,
            r###""db""###,
            r###""dw""###,
            r###""macro""###,
            r###""set""###,
            r###""{""###,
            r###"r#"0(b|B)[0-1]+"#"###,
            r###"r#"0(x|X)[0-9A-Fa-f]+"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[_a-zA-Z0-9 \\[\\]]*}"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*:"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input, 's>
    where 
    {
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input (), &'s ())>,
    }
    impl<'input, 's> __state_machine::ParserDefinition for __StateMachine<'input, 's>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ();
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 21 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.context,
                self.out,
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
        's,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(6, _) if true => Some(0),
            Token(7, _) if true => Some(1),
            Token(8, _) if true => Some(2),
            Token(9, _) if true => Some(3),
            Token(10, _) if true => Some(4),
            Token(11, _) if true => Some(5),
            Token(12, _) if true => Some(6),
            Token(13, _) if true => Some(7),
            Token(14, _) if true => Some(8),
            Token(15, _) if true => Some(9),
            Token(16, _) if true => Some(10),
            Token(17, _) if true => Some(11),
            Token(18, _) if true => Some(12),
            Token(19, _) if true => Some(13),
            Token(20, _) if true => Some(14),
            Token(0, _) if true => Some(15),
            Token(1, _) if true => Some(16),
            Token(2, _) if true => Some(17),
            Token(3, _) if true => Some(18),
            Token(4, _) if true => Some(19),
            Token(5, _) if true => Some(20),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        's,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => match __token {
                Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct data_directivesParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl data_directivesParser {
        pub fn new() -> data_directivesParser {
            let __builder = super::__intern_token::new_builder();
            data_directivesParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            's,
        >(
            &self,
            context: &'s mut util::Context,
            out: &'s mut util::Output,
            input: &'input str,
        ) -> Result<(), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    context,
                    out,
                    input,
                    __phantom: ::std::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<Result<(),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                // __data_directives = data_directives => ActionFn(1);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(context, out, input, __sym0);
                return Some(Ok(__nt));
            }
            24 => {
                __reduce24(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                // byte_num = r#"[0-9]+"# => ActionFn(88);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action88::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            26 => {
                // byte_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(89);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action89::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            27 => {
                // byte_num = r#"0(b|B)[0-1]+"# => ActionFn(90);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action90::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            28 => {
                __reduce28(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                // label = r#"[_a-zA-Z][_a-zA-Z0-9]*:"# => ActionFn(39);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action39::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 22)
            }
            55 => {
                __reduce55(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                // macro_use = r#"[_a-zA-Z][_a-zA-Z0-9]*"#, "(", CommaSepList<general_string>, ")" => ActionFn(91);
                assert!(__symbols.len() >= 4);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant5(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = match super::__action91::<>(context, out, input, __sym0, __sym1, __sym2, __sym3) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (4, 24)
            }
            57 => {
                __reduce57(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                // word_num = r#"[0-9]+"# => ActionFn(92);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action92::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            71 => {
                // word_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(93);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action93::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            72 => {
                // word_num = r#"0(b|B)[0-1]+"# => ActionFn(94);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action94::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u16, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",") = general_string, "," => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")* =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action60::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")* = (<general_string> ",")+ => ActionFn(61);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")+ = general_string, "," => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action67::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")+ = (<general_string> ",")+, general_string, "," => ActionFn(68);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action68::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",") = name_string, "," => ActionFn(57);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action57::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")* = (<name_string> ",")+ => ActionFn(56);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")+ = name_string, "," => ActionFn(71);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action71::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")+ = (<name_string> ",")+, name_string, "," => ActionFn(72);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action72::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Code = data_directives => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = general_string => ActionFn(95);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce14<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> =  => ActionFn(96);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action96::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = (<general_string> ",")+, general_string => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = (<general_string> ",")+ => ActionFn(98);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = name_string => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> =  => ActionFn(100);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce19<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = (<name_string> ",")+, name_string => ActionFn(101);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action101::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce20<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = (<name_string> ",")+ => ActionFn(102);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce21<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __Code = Code => ActionFn(0);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce22<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __code_directives = code_directives => ActionFn(2);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce24<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __macro_def = macro_def => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce28<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = macro_def => ActionFn(30);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce29<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = macro_use => ActionFn(31);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce30<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = code_directives, macro_def => ActionFn(32);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce31<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = code_directives, macro_use => ActionFn(33);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce32<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = set_directive => ActionFn(5);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce33<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = db_directive => ActionFn(6);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce34<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = dw_directive => ActionFn(7);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce35<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, set_directive => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce36<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, db_directive => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce37<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, dw_directive => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce38<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, byte_num => ActionFn(78);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action78::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce39<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, byte_num => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce40<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, "[", word_num, "]" => ActionFn(79);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action79::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 18)
    }
    pub(crate) fn __reduce41<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, "[", word_num, "]" => ActionFn(17);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action17::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 18)
    }
    pub(crate) fn __reduce42<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, "[", word_num, ";", byte_num, "]" => ActionFn(80);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action80::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 18)
    }
    pub(crate) fn __reduce43<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, "[", word_num, ";", byte_num, "]" => ActionFn(19);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action19::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 18)
    }
    pub(crate) fn __reduce44<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, word_num => ActionFn(81);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action81::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce45<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, word_num => ActionFn(23);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce46<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, "[", word_num, "]" => ActionFn(82);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action82::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 19)
    }
    pub(crate) fn __reduce47<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, "[", word_num, "]" => ActionFn(25);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action25::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 19)
    }
    pub(crate) fn __reduce48<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, "[", word_num, ";", word_num, "]" => ActionFn(83);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action83::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 19)
    }
    pub(crate) fn __reduce49<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, "[", word_num, ";", word_num, "]" => ActionFn(27);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action27::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 19)
    }
    pub(crate) fn __reduce50<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string = name_string => ActionFn(47);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce51<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string = word_num => ActionFn(48);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce52<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string? = general_string => ActionFn(58);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce53<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string? =  => ActionFn(59);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action59::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce55<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // macro_def = quote_macro, name_string, "(", CommaSepList<name_string>, ")", "{", raw_code => ActionFn(34);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action34::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 23)
    }
    pub(crate) fn __reduce57<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce58<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string? = name_string => ActionFn(53);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce59<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 26)
    }
    pub(crate) fn __reduce60<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_db = "DB" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce61<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_db = "db" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce62<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_dw = "dw" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce63<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_dw = "DW" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce64<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_macro = "MACRO" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce65<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_macro = "macro" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce66<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_set = "set" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce67<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_set = "SET" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce68<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // raw_code = r#"[_a-zA-Z0-9 \\[\\]]*}"# => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce69<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // set_directive = quote_set, word_num => ActionFn(11);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action11::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 32)
    }
}
pub use self::__parse__data_directives::data_directivesParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__macro_def {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::preprocessor_util as util;
    use util::LabelType;
    use regex::{Regex,Captures};
    use crate::preprocessor_error;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(String),
        Variant2(::std::vec::Vec<String>),
        Variant3(usize),
        Variant4(()),
        Variant5(Vec<String>),
        Variant6(u8),
        Variant7(::std::option::Option<String>),
        Variant8(u16),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0,
        // State 2
        0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0,
        // State 3
        0, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -65, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -66, 0,
        // State 8
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -18, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, -20, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0,
        // State 14
        0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0,
        // State 15
        0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 21 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        -25,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        -56,
        // State 17
        -69,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            5 => 3,
            10 => 10,
            23 => 5,
            25 => match state {
                2 => 11,
                3 => 12,
                _ => 8,
            },
            29 => 1,
            31 => 16,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###"";""###,
            r###""DB""###,
            r###""DW""###,
            r###""MACRO""###,
            r###""SET""###,
            r###""[""###,
            r###""]""###,
            r###""db""###,
            r###""dw""###,
            r###""macro""###,
            r###""set""###,
            r###""{""###,
            r###"r#"0(b|B)[0-1]+"#"###,
            r###"r#"0(x|X)[0-9A-Fa-f]+"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[_a-zA-Z0-9 \\[\\]]*}"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*:"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input, 's>
    where 
    {
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input (), &'s ())>,
    }
    impl<'input, 's> __state_machine::ParserDefinition for __StateMachine<'input, 's>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ();
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 21 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.context,
                self.out,
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
        's,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(6, _) if true => Some(0),
            Token(7, _) if true => Some(1),
            Token(8, _) if true => Some(2),
            Token(9, _) if true => Some(3),
            Token(10, _) if true => Some(4),
            Token(11, _) if true => Some(5),
            Token(12, _) if true => Some(6),
            Token(13, _) if true => Some(7),
            Token(14, _) if true => Some(8),
            Token(15, _) if true => Some(9),
            Token(16, _) if true => Some(10),
            Token(17, _) if true => Some(11),
            Token(18, _) if true => Some(12),
            Token(19, _) if true => Some(13),
            Token(20, _) if true => Some(14),
            Token(0, _) if true => Some(15),
            Token(1, _) if true => Some(16),
            Token(2, _) if true => Some(17),
            Token(3, _) if true => Some(18),
            Token(4, _) if true => Some(19),
            Token(5, _) if true => Some(20),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        's,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => match __token {
                Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct macro_defParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl macro_defParser {
        pub fn new() -> macro_defParser {
            let __builder = super::__intern_token::new_builder();
            macro_defParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            's,
        >(
            &self,
            context: &'s mut util::Context,
            out: &'s mut util::Output,
            input: &'input str,
        ) -> Result<(), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    context,
                    out,
                    input,
                    __phantom: ::std::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<Result<(),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                // __macro_def = macro_def => ActionFn(3);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(context, out, input, __sym0);
                return Some(Ok(__nt));
            }
            25 => {
                // byte_num = r#"[0-9]+"# => ActionFn(88);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action88::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            26 => {
                // byte_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(89);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action89::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            27 => {
                // byte_num = r#"0(b|B)[0-1]+"# => ActionFn(90);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action90::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 15)
            }
            28 => {
                __reduce28(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                __reduce40(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            43 => {
                __reduce43(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            44 => {
                __reduce44(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            45 => {
                __reduce45(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            46 => {
                __reduce46(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            47 => {
                __reduce47(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            48 => {
                __reduce48(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            49 => {
                __reduce49(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            50 => {
                __reduce50(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            51 => {
                __reduce51(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            52 => {
                __reduce52(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            53 => {
                __reduce53(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            54 => {
                // label = r#"[_a-zA-Z][_a-zA-Z0-9]*:"# => ActionFn(39);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action39::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 22)
            }
            55 => {
                __reduce55(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            56 => {
                // macro_use = r#"[_a-zA-Z][_a-zA-Z0-9]*"#, "(", CommaSepList<general_string>, ")" => ActionFn(91);
                assert!(__symbols.len() >= 4);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant5(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = match super::__action91::<>(context, out, input, __sym0, __sym1, __sym2, __sym3) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (4, 24)
            }
            57 => {
                __reduce57(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            58 => {
                __reduce58(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            59 => {
                __reduce59(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            60 => {
                __reduce60(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            61 => {
                __reduce61(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            62 => {
                __reduce62(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            63 => {
                __reduce63(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            64 => {
                __reduce64(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            65 => {
                __reduce65(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            66 => {
                __reduce66(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            67 => {
                __reduce67(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            68 => {
                __reduce68(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            69 => {
                __reduce69(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            70 => {
                // word_num = r#"[0-9]+"# => ActionFn(92);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action92::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            71 => {
                // word_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(93);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action93::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            72 => {
                // word_num = r#"0(b|B)[0-1]+"# => ActionFn(94);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action94::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 33)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u16, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",") = general_string, "," => ActionFn(62);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action62::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")* =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action60::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")* = (<general_string> ",")+ => ActionFn(61);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action61::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")+ = general_string, "," => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action67::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<general_string> ",")+ = (<general_string> ",")+, general_string, "," => ActionFn(68);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action68::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",") = name_string, "," => ActionFn(57);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action57::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")* =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")* = (<name_string> ",")+ => ActionFn(56);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action56::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")+ = name_string, "," => ActionFn(71);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action71::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // (<name_string> ",")+ = (<name_string> ",")+, name_string, "," => ActionFn(72);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action72::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action52::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action49::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Code = data_directives => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce13<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = general_string => ActionFn(95);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action95::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce14<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> =  => ActionFn(96);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action96::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce15<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = (<general_string> ",")+, general_string => ActionFn(97);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action97::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<general_string> = (<general_string> ",")+ => ActionFn(98);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action98::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce17<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = name_string => ActionFn(99);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action99::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> =  => ActionFn(100);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action100::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce19<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = (<name_string> ",")+, name_string => ActionFn(101);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action101::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce20<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // CommaSepList<name_string> = (<name_string> ",")+ => ActionFn(102);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action102::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce21<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __Code = Code => ActionFn(0);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce22<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __code_directives = code_directives => ActionFn(2);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce23<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // __data_directives = data_directives => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = macro_def => ActionFn(30);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce29<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = macro_use => ActionFn(31);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce30<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = code_directives, macro_def => ActionFn(32);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce31<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code_directives = code_directives, macro_use => ActionFn(33);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 16)
    }
    pub(crate) fn __reduce32<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = set_directive => ActionFn(5);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce33<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = db_directive => ActionFn(6);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce34<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = dw_directive => ActionFn(7);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce35<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, set_directive => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce36<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, db_directive => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce37<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // data_directives = data_directives, dw_directive => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce38<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, byte_num => ActionFn(78);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action78::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 18)
    }
    pub(crate) fn __reduce39<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, byte_num => ActionFn(15);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 18)
    }
    pub(crate) fn __reduce40<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, "[", word_num, "]" => ActionFn(79);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action79::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 18)
    }
    pub(crate) fn __reduce41<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, "[", word_num, "]" => ActionFn(17);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action17::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 18)
    }
    pub(crate) fn __reduce42<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = label, quote_db, "[", word_num, ";", byte_num, "]" => ActionFn(80);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action80::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 18)
    }
    pub(crate) fn __reduce43<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db_directive = quote_db, "[", word_num, ";", byte_num, "]" => ActionFn(19);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action19::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 18)
    }
    pub(crate) fn __reduce44<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, word_num => ActionFn(81);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action81::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 19)
    }
    pub(crate) fn __reduce45<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, word_num => ActionFn(23);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce46<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, "[", word_num, "]" => ActionFn(82);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action82::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 19)
    }
    pub(crate) fn __reduce47<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, "[", word_num, "]" => ActionFn(25);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action25::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 19)
    }
    pub(crate) fn __reduce48<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = label, quote_dw, "[", word_num, ";", word_num, "]" => ActionFn(83);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action83::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 19)
    }
    pub(crate) fn __reduce49<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw_directive = quote_dw, "[", word_num, ";", word_num, "]" => ActionFn(27);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action27::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 19)
    }
    pub(crate) fn __reduce50<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string = name_string => ActionFn(47);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce51<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string = word_num => ActionFn(48);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action48::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce52<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string? = general_string => ActionFn(58);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action58::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce53<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // general_string? =  => ActionFn(59);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action59::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce55<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // macro_def = quote_macro, name_string, "(", CommaSepList<name_string>, ")", "{", raw_code => ActionFn(34);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action34::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 23)
    }
    pub(crate) fn __reduce57<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(46);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce58<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string? = name_string => ActionFn(53);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce59<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string? =  => ActionFn(54);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action54::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 26)
    }
    pub(crate) fn __reduce60<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_db = "DB" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce61<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_db = "db" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce62<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_dw = "dw" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce63<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_dw = "DW" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce64<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_macro = "MACRO" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce65<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_macro = "macro" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce66<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_set = "set" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce67<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // quote_set = "SET" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 30)
    }
    pub(crate) fn __reduce68<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // raw_code = r#"[_a-zA-Z0-9 \\[\\]]*}"# => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 31)
    }
    pub(crate) fn __reduce69<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut util::Output,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // set_directive = quote_set, word_num => ActionFn(11);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action11::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 32)
    }
}
pub use self::__parse__macro_def::macro_defParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::util::preprocessor_util as util;
    use util::LabelType;
    use regex::{Regex,Captures};
    use crate::preprocessor_error;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(0(b|B)[0-1]+)", false),
            ("^(0(x|X)[0-9A-Fa-f]+)", false),
            ("^([0-9]+)", false),
            ("^([ 0-9A-\\[\\]_a-z]*\\})", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*:)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(,)", false),
            ("^(;)", false),
            ("^(DB)", false),
            ("^(DW)", false),
            ("^(MACRO)", false),
            ("^(SET)", false),
            ("^(\\[)", false),
            ("^(\\])", false),
            ("^(db)", false),
            ("^(dw)", false),
            ("^(macro)", false),
            ("^(set)", false),
            ("^(\\{)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action1<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action2<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action3<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action4<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action5<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action6<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action7<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action8<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action9<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action10<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action11<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, n, _): (usize, u16, usize),
) -> ()
{
    {
        out.data.push(format!("set {}",n));
    }
}

#[allow(unused_variables)]
fn __action12<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action13<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action14<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, (), usize),
    (_, n, _): (usize, u8, usize),
) -> ()
{
    {
        context.label_map.insert(l,(LabelType::DATA,start as u16,context.data_counter));
        out.data.push(format!("db {}",n));
        // Increment the data counter
        context.data_counter += 1;
    }
}

#[allow(unused_variables)]
fn __action15<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, n, _): (usize, u8, usize),
) -> ()
{
    {
        out.data.push(format!("db {}",n));
        // Increment the data counter
        context.data_counter += 1;
    }
}

#[allow(unused_variables)]
fn __action16<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        context.label_map.insert(l,(LabelType::DATA,start as u16,context.data_counter));
        out.data.push(format!("db [{}]",n));
        // Increment the data counter
        context.data_counter += n;
    }
}

#[allow(unused_variables)]
fn __action17<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        out.data.push(format!("db [{}]",n));
        // Increment the data counter
        context.data_counter += n;
    }
}

#[allow(unused_variables)]
fn __action18<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, u8, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        context.label_map.insert(l,(LabelType::DATA,start as u16,context.data_counter));
        out.data.push(format!("db [{} ; {}]",n,v));
        // Increment the data counter
        context.data_counter += n;
    }
}

#[allow(unused_variables)]
fn __action19<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, u8, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        out.data.push(format!("db [{} ; {}]",n,v));
        // Increment the data counter
        context.data_counter += n;
    }
}

#[allow(unused_variables)]
fn __action20<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action21<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action22<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, (), usize),
    (_, n, _): (usize, u16, usize),
) -> ()
{
    {
        context.label_map.insert(l,(LabelType::DATA,start as u16,context.data_counter));
        out.data.push(format!("dw {}",n));
        // Increment the data counter
        context.data_counter += 2;
    }
}

#[allow(unused_variables)]
fn __action23<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, n, _): (usize, u16, usize),
) -> ()
{
    {
        out.data.push(format!("dw {}",n));
        // Increment the data counter
        context.data_counter += 2;
    }
}

#[allow(unused_variables)]
fn __action24<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        context.label_map.insert(l,(LabelType::DATA,start as u16,context.data_counter));
        out.data.push(format!("dw [{}]",n));
        // Increment the data counter
        context.data_counter += 2*n;
    }
}

#[allow(unused_variables)]
fn __action25<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        out.data.push(format!("dw [{}]",n));
        // Increment the data counter
        context.data_counter += 2*n;
    }
}

#[allow(unused_variables)]
fn __action26<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        context.label_map.insert(l,(LabelType::DATA,start as u16,context.data_counter));
        out.data.push(format!("dw [{} ; {}]",n,v));
        // Increment the data counter
        context.data_counter += 2*n;
    }
}

#[allow(unused_variables)]
fn __action27<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        out.data.push(format!("dw [{} ; {}]",n,v));
        // Increment the data counter
        context.data_counter += 2*n;
    }
}

#[allow(unused_variables)]
fn __action28<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action29<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action30<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action31<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action32<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action33<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
    (_, __1, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action34<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, params, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, String, usize),
) -> ()
{
    {
        let mut r = r.clone();
        // TODO Maybe optimise this later
        // maybe try making single regex of all params and replace, somehow
        for (i,p) in params.iter().enumerate(){
            let pat = format!(r"\b{}\b",p);
            let re = Regex::new(&pat).unwrap();
            r = re.replace_all(&r,|caps:&Captures|{
                format!("{{{}}}",i)
            }).to_string();
        }
        context.macro_map.insert(name,r);
    }
}

#[allow(unused_variables)]
fn __action35<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action36<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action37<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    s[0..s.len()-2].to_owned()
}

#[allow(unused_variables)]
fn __action38<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, params, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<(),__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match context.macro_map.get(l){
            Some(value)=>{
                let mut r = value.clone();
                // TODO Maybe optimise this later
                // maybe try making single regex of all params and replace, somehow
                for (i,p) in params.iter().enumerate(){
                    let pat = format!("{{{}}}",i);
                    r = r.replace(&pat,&p);
                }
                let p = CodeParser::new();
                let o = p.parse(context,out,&r);
                match o{
                    Ok(_)=>Ok(()),
                    Err(e)=>{
                        // TODO better format error, maybe send the error position in formatted string as well?
                        preprocessor_error!(start,end,l,"Error in macro expansion".to_owned())
                    }
                }
            },
            None => preprocessor_error!(start as usize,start +l.len(),l,"Macro not defined".to_owned()),
        }
    }
}

#[allow(unused_variables)]
fn __action39<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> Result<String,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match context.label_map.get(&s[0..s.len()-2]){
            Some((_,pos,_)) => return preprocessor_error!(*pos as usize,*pos as usize+s.len(),s,"Label Already defined".to_owned()),
            None => Ok(s[0..s.len()-2].to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action40<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match u16::from_str_radix(n,10){
            Ok(v) => Ok(v),
            Err(_) => preprocessor_error!(start,end,n,"Invalid Value, must be between 0-65535".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action41<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match u16::from_str_radix(&n[2..],16){
            Ok(v) => Ok(v),
            Err(_) => preprocessor_error!(start,end,n,"Invalid Value, must be between 0-65535".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action42<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match u16::from_str_radix(&n[2..],2){
            Ok(v) => Ok(v),
            Err(_) => preprocessor_error!(start,end,n,"Invalid Value, must be between 0-65535".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action43<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match u8::from_str_radix(n,10){
            Ok(v) => Ok(v),
            Err(_) => preprocessor_error!(start,end,n,"Invalid Value, must be between 0-255".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action44<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match u8::from_str_radix(&n[2..],16){
            Ok(v) => Ok(v),
            Err(_) => preprocessor_error!(start,end,n,"Invalid Value, must be between 0-255".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action45<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match u8::from_str_radix(&n[2..],2){
            Ok(v) => Ok(v),
            Err(_) => preprocessor_error!(start,end,n,"Invalid Value, must be between 0-255".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action46<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, v, _): (usize, &'input str, usize),
) -> String
{
    {
        v.to_owned()
    }
}

#[allow(unused_variables)]
fn __action47<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
fn __action48<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, n, _): (usize, u16, usize),
) -> String
{
    format!("{}",n)
}

#[allow(unused_variables)]
fn __action49<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action50<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action51<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action52<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action53<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action54<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<String>
{
    None
}

#[allow(unused_variables)]
fn __action55<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<String>
{
    vec![]
}

#[allow(unused_variables)]
fn __action56<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
) -> ::std::vec::Vec<String>
{
    v
}

#[allow(unused_variables)]
fn __action57<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
fn __action58<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::option::Option<String>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action59<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<String>
{
    None
}

#[allow(unused_variables)]
fn __action60<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<String>
{
    vec![]
}

#[allow(unused_variables)]
fn __action61<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
) -> ::std::vec::Vec<String>
{
    v
}

#[allow(unused_variables)]
fn __action62<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
fn __action63<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action64<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action65<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action66<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action67<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action62(
        context,
        out,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        context,
        out,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action68<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action62(
        context,
        out,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action69<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action60(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        context,
        out,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action70<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action61(
        context,
        out,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        context,
        out,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action71<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action57(
        context,
        out,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        context,
        out,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action72<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action57(
        context,
        out,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action73<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action55(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        context,
        out,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action74<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, ::std::option::Option<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action56(
        context,
        out,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        context,
        out,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action75<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action76<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action77<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action78<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, (), usize),
    __2: (usize, u8, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action79<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, (), usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u16, usize),
    __4: (usize, &'input str, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action80<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, (), usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u16, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, u8, usize),
    __6: (usize, &'input str, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action81<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, (), usize),
    __2: (usize, u16, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action82<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, (), usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u16, usize),
    __4: (usize, &'input str, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action83<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, (), usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u16, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, u16, usize),
    __6: (usize, &'input str, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
fn __action84<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<String>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, usize, usize),
) -> Result<(),__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action85<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action86<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action87<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action88<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action75(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action89<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action76(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action90<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action77(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action91<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<String>, usize),
    __3: (usize, &'input str, usize),
) -> Result<(),__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action49(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action84(
        context,
        out,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action92<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action85(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action93<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action86(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action94<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action49(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action87(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action95<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action58(
        context,
        out,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        context,
        out,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action96<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action59(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        context,
        out,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action97<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action58(
        context,
        out,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action98<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action59(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action99<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action53(
        context,
        out,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        context,
        out,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action100<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action54(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        context,
        out,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action101<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action53(
        context,
        out,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action102<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action54(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, 's, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, 's, > __ToTriple<'input, 's, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, 's, > __ToTriple<'input, 's, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
