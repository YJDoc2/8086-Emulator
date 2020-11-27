// auto-generated: "lalrpop 0.19.1"
// sha256: 837eaaf260935b53d9b81eccba418383c3a2e57c641566a7c98518b8b1981a8
use crate::util::preprocessor_util as util;
use util::LabelType;
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
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 0, 0, 0, 21,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 0, 0, 0, 21,
        // State 2
        0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 26, 27, 28, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 26, 27, 28, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 28, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 28, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20, 0, 0, 0, 0, -20,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, -21, 0, 0, 0, 0, -21,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19, 0, 0, 0, 0, -19,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, -37, -37, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, -23, 0, 0, 0, 0, -23,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, 0, 0, 0, 0, -24,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, -22, 0, 0, 0, 0, -22,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, -26, 0, 0, 0, 0, -26,
        // State 25
        0, 0, 0, 0, 0, 0, -18, 0, 0, -18, -18, -18, 0, 0, 0, 0, -18,
        // State 26
        0, 0, 0, 0, 0, 0, -17, 0, 0, -17, -17, -17, 0, 0, 0, 0, -17,
        // State 27
        0, 0, 0, 0, 0, 0, -16, 0, 0, -16, -16, -16, 0, 0, 0, 0, -16,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, 0, 0, 0, 0, -32,
        // State 29
        0, 0, 0, -48, 0, 0, -48, 0, 0, -48, -48, -48, 0, 0, 0, 0, -48,
        // State 30
        0, 0, 0, -47, 0, 0, -47, 0, 0, -47, -47, -47, 0, 0, 0, 0, -47,
        // State 31
        0, 0, 0, -46, 0, 0, -46, 0, 0, -46, -46, -46, 0, 0, 0, 0, -46,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, -42, -42, -42, 0, 0, 0, 0, -42,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, 0, 0, 0, 0, -25,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, -31, 0, 0, 0, 0, -31,
        // State 35
        0, 0, 0, 12, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 13, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 14, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 15, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, -28, 0, 0, 0, 0, -28,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 0, 0, -34,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, -27, 0, 0, 0, 0, -27,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, 0, 0, 0, -33,
        // State 43
        0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30, -30, 0, 0, 0, 0, -30,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 0, 0, -36,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29, -29, 0, 0, 0, 0, -29,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 0, 0, -35,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 17 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -8,
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
        -13,
        // State 16
        -20,
        // State 17
        -21,
        // State 18
        0,
        // State 19
        -19,
        // State 20
        0,
        // State 21
        -23,
        // State 22
        -24,
        // State 23
        -22,
        // State 24
        -26,
        // State 25
        -18,
        // State 26
        -17,
        // State 27
        -16,
        // State 28
        -32,
        // State 29
        -48,
        // State 30
        -47,
        // State 31
        -46,
        // State 32
        -42,
        // State 33
        -25,
        // State 34
        -31,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        -28,
        // State 40
        -34,
        // State 41
        -27,
        // State 42
        -33,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        -30,
        // State 48
        -36,
        // State 49
        -29,
        // State 50
        -35,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            5 => 15,
            10 => match state {
                5 => 33,
                11 => 43,
                13 => 45,
                _ => 24,
            },
            11 => 1,
            12 => match state {
                1 => 21,
                _ => 16,
            },
            13 => match state {
                1 => 22,
                _ => 17,
            },
            14 => 18,
            17 => match state {
                1 => 23,
                _ => 19,
            },
            20 => match state {
                4 => 32,
                6 => 34,
                7 => 35,
                8 => 36,
                9 => 37,
                10 => 38,
                12 => 44,
                14 => 46,
                _ => 28,
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
            r###""MACRO""###,
            r###""[""###,
            r###""]""###,
            r###""{""###,
            r###""}""###,
            r###"r#"(db|DB) "#"###,
            r###"r#"(dw|DW) "#"###,
            r###"r#"(set|SET) "#"###,
            r###"r#"0(b|B)[0-1]+"#"###,
            r###"r#"0(x|X)[0-9A-Fa-f]+"#"###,
            r###"r#"[0-9]+"#"###,
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
            __action(state, 17 - 1)
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
            Token(8, _) if true => Some(0),
            Token(9, _) if true => Some(1),
            Token(10, _) if true => Some(2),
            Token(11, _) if true => Some(3),
            Token(12, _) if true => Some(4),
            Token(13, _) if true => Some(5),
            Token(14, _) if true => Some(6),
            Token(15, _) if true => Some(7),
            Token(16, _) if true => Some(8),
            Token(0, _) if true => Some(9),
            Token(1, _) if true => Some(10),
            Token(2, _) if true => Some(11),
            Token(3, _) if true => Some(12),
            Token(4, _) if true => Some(13),
            Token(5, _) if true => Some(14),
            Token(6, _) if true => Some(15),
            Token(7, _) if true => Some(16),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => match __token {
                Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) if true => __Symbol::Variant0(__tok0),
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
                // __Code = Code => ActionFn(0);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(context, out, input, __sym0);
                return Some(Ok(__nt));
            }
            13 => {
                __reduce13(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                // byte_num = r#"[0-9]+"# => ActionFn(61);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action61::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
            }
            16 => {
                // byte_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(62);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action62::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
            }
            17 => {
                // byte_num = r#"0(b|B)[0-1]+"# => ActionFn(63);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action63::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
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
                __reduce24(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
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
                // label = r#"[_a-zA-Z][_a-zA-Z0-9]*:"# => ActionFn(27);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action27::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 14)
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
                // word_num = r#"[0-9]+"# => ActionFn(64);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action64::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
            }
            46 => {
                // word_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(65);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action65::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
            }
            47 => {
                // word_num = r#"0(b|B)[0-1]+"# => ActionFn(66);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action66::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
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
        // (<string> ",") = string, "," => ActionFn(42);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(context, out, input, __sym0, __sym1);
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
        // (<string> ",")* =  => ActionFn(40);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action40::<>(context, out, input, &__start, &__end);
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
        // (<string> ",")* = (<string> ",")+ => ActionFn(41);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(context, out, input, __sym0);
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
        // (<string> ",")+ = string, "," => ActionFn(45);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action45::<>(context, out, input, __sym0, __sym1);
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
        // (<string> ",")+ = (<string> ",")+, string, "," => ActionFn(46);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action46::<>(context, out, input, __sym0, __sym1, __sym2);
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
        // @L =  => ActionFn(37);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action37::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 3)
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
        // @R =  => ActionFn(35);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action35::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // Code = data_directives => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
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
        // CommaSepList<string> = string => ActionFn(67);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
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
        // CommaSepList<string> =  => ActionFn(68);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action68::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 6)
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
        // CommaSepList<string> = (<string> ",")+, string => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
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
        // CommaSepList<string> = (<string> ",")+ => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
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
        // __data_directives = data_directives => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
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
        // __macro_def = macro_def => ActionFn(2);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 9)
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
        // data_directives = set_directive => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = db_directive => ActionFn(5);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = dw_directive => ActionFn(6);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = data_directives, set_directive => ActionFn(7);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action7::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // data_directives = data_directives, db_directive => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // data_directives = data_directives, dw_directive => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // db_directive = label, r#"(db|DB) "#, byte_num => ActionFn(52);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
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
        // db_directive = r#"(db|DB) "#, byte_num => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce26<
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
        // db_directive = label, r#"(db|DB) "#, "[", word_num, "]" => ActionFn(53);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action53::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 12)
    }
    pub(crate) fn __reduce27<
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
        // db_directive = r#"(db|DB) "#, "[", word_num, "]" => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 12)
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
        // db_directive = label, r#"(db|DB) "#, "[", word_num, ";", byte_num, "]" => ActionFn(54);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action54::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 12)
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
        // db_directive = r#"(db|DB) "#, "[", word_num, ";", byte_num, "]" => ActionFn(16);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action16::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 12)
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
        // dw_directive = label, r#"(dw|DW) "#, word_num => ActionFn(55);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action55::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 13)
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
        // dw_directive = r#"(dw|DW) "#, word_num => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action18::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 13)
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
        // dw_directive = label, r#"(dw|DW) "#, "[", word_num, "]" => ActionFn(56);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action56::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 13)
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
        // dw_directive = r#"(dw|DW) "#, "[", word_num, "]" => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 13)
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
        // dw_directive = label, r#"(dw|DW) "#, "[", word_num, ";", word_num, "]" => ActionFn(57);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action57::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 13)
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
        // dw_directive = r#"(dw|DW) "#, "[", word_num, ";", word_num, "]" => ActionFn(22);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action22::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 13)
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
        // macro_def = "MACRO", string, "(", CommaSepList<string>, ")", "{", raw_code, "}" => ActionFn(23);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action23::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (8, 15)
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
        // raw_code = string => ActionFn(24);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
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
        // raw_code = raw_code, string => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 16)
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
        // raw_code = raw_code, ";" => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 16)
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
        // set_directive = r#"(set|SET) "#, word_num => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
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
        // string = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 18)
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
        // string? = string => ActionFn(38);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
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
        // string? =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 19)
    }
}
pub use self::__parse__Code::CodeParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__data_directives {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::preprocessor_util as util;
    use util::LabelType;
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
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 0, 0, 0, 20,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 0, 0, 0, 0, 20,
        // State 2
        0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 25, 26, 27, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 25, 26, 27, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20, 0, 0, 0, 0, -20,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, -21, 0, 0, 0, 0, -21,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19, 0, 0, 0, 0, -19,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, -37, -37, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, -23, 0, 0, 0, 0, -23,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, 0, 0, 0, 0, -24,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, -22, 0, 0, 0, 0, -22,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, -26, -26, -26, 0, 0, 0, 0, -26,
        // State 24
        0, 0, 0, 0, 0, 0, -18, 0, 0, -18, -18, -18, 0, 0, 0, 0, -18,
        // State 25
        0, 0, 0, 0, 0, 0, -17, 0, 0, -17, -17, -17, 0, 0, 0, 0, -17,
        // State 26
        0, 0, 0, 0, 0, 0, -16, 0, 0, -16, -16, -16, 0, 0, 0, 0, -16,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, 0, 0, 0, 0, -32,
        // State 28
        0, 0, 0, -48, 0, 0, -48, 0, 0, -48, -48, -48, 0, 0, 0, 0, -48,
        // State 29
        0, 0, 0, -47, 0, 0, -47, 0, 0, -47, -47, -47, 0, 0, 0, 0, -47,
        // State 30
        0, 0, 0, -46, 0, 0, -46, 0, 0, -46, -46, -46, 0, 0, 0, 0, -46,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, -42, -42, -42, 0, 0, 0, 0, -42,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, 0, 0, 0, 0, -25,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, -31, 0, 0, 0, 0, -31,
        // State 34
        0, 0, 0, 12, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 13, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 14, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 15, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, -28, 0, 0, 0, 0, -28,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 0, 0, -34,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, -27, 0, 0, 0, 0, -27,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, 0, 0, 0, -33,
        // State 42
        0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30, -30, 0, 0, 0, 0, -30,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 0, 0, -36,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29, -29, 0, 0, 0, 0, -29,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 0, 0, -35,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 17 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -14,
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
        -20,
        // State 16
        -21,
        // State 17
        0,
        // State 18
        -19,
        // State 19
        0,
        // State 20
        -23,
        // State 21
        -24,
        // State 22
        -22,
        // State 23
        -26,
        // State 24
        -18,
        // State 25
        -17,
        // State 26
        -16,
        // State 27
        -32,
        // State 28
        -48,
        // State 29
        -47,
        // State 30
        -46,
        // State 31
        -42,
        // State 32
        -25,
        // State 33
        -31,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -28,
        // State 39
        -34,
        // State 40
        -27,
        // State 41
        -33,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        -30,
        // State 47
        -36,
        // State 48
        -29,
        // State 49
        -35,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            10 => match state {
                5 => 32,
                11 => 42,
                13 => 44,
                _ => 23,
            },
            11 => 1,
            12 => match state {
                1 => 20,
                _ => 15,
            },
            13 => match state {
                1 => 21,
                _ => 16,
            },
            14 => 17,
            17 => match state {
                1 => 22,
                _ => 18,
            },
            20 => match state {
                4 => 31,
                6 => 33,
                7 => 34,
                8 => 35,
                9 => 36,
                10 => 37,
                12 => 43,
                14 => 45,
                _ => 27,
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
            r###""MACRO""###,
            r###""[""###,
            r###""]""###,
            r###""{""###,
            r###""}""###,
            r###"r#"(db|DB) "#"###,
            r###"r#"(dw|DW) "#"###,
            r###"r#"(set|SET) "#"###,
            r###"r#"0(b|B)[0-1]+"#"###,
            r###"r#"0(x|X)[0-9A-Fa-f]+"#"###,
            r###"r#"[0-9]+"#"###,
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
            __action(state, 17 - 1)
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
            Token(8, _) if true => Some(0),
            Token(9, _) if true => Some(1),
            Token(10, _) if true => Some(2),
            Token(11, _) if true => Some(3),
            Token(12, _) if true => Some(4),
            Token(13, _) if true => Some(5),
            Token(14, _) if true => Some(6),
            Token(15, _) if true => Some(7),
            Token(16, _) if true => Some(8),
            Token(0, _) if true => Some(9),
            Token(1, _) if true => Some(10),
            Token(2, _) if true => Some(11),
            Token(3, _) if true => Some(12),
            Token(4, _) if true => Some(13),
            Token(5, _) if true => Some(14),
            Token(6, _) if true => Some(15),
            Token(7, _) if true => Some(16),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => match __token {
                Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) if true => __Symbol::Variant0(__tok0),
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
                // __data_directives = data_directives => ActionFn(1);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(context, out, input, __sym0);
                return Some(Ok(__nt));
            }
            14 => {
                __reduce14(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                // byte_num = r#"[0-9]+"# => ActionFn(61);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action61::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
            }
            16 => {
                // byte_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(62);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action62::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
            }
            17 => {
                // byte_num = r#"0(b|B)[0-1]+"# => ActionFn(63);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action63::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
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
                __reduce24(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
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
                // label = r#"[_a-zA-Z][_a-zA-Z0-9]*:"# => ActionFn(27);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action27::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 14)
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
                // word_num = r#"[0-9]+"# => ActionFn(64);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action64::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
            }
            46 => {
                // word_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(65);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action65::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
            }
            47 => {
                // word_num = r#"0(b|B)[0-1]+"# => ActionFn(66);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action66::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
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
        // (<string> ",") = string, "," => ActionFn(42);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(context, out, input, __sym0, __sym1);
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
        // (<string> ",")* =  => ActionFn(40);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action40::<>(context, out, input, &__start, &__end);
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
        // (<string> ",")* = (<string> ",")+ => ActionFn(41);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(context, out, input, __sym0);
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
        // (<string> ",")+ = string, "," => ActionFn(45);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action45::<>(context, out, input, __sym0, __sym1);
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
        // (<string> ",")+ = (<string> ",")+, string, "," => ActionFn(46);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action46::<>(context, out, input, __sym0, __sym1, __sym2);
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
        // @L =  => ActionFn(37);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action37::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 3)
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
        // @R =  => ActionFn(35);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action35::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // Code = data_directives => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
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
        // CommaSepList<string> = string => ActionFn(67);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
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
        // CommaSepList<string> =  => ActionFn(68);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action68::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 6)
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
        // CommaSepList<string> = (<string> ",")+, string => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
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
        // CommaSepList<string> = (<string> ",")+ => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
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
        // __Code = Code => ActionFn(0);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 7)
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
        // __macro_def = macro_def => ActionFn(2);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 9)
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
        // data_directives = set_directive => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = db_directive => ActionFn(5);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = dw_directive => ActionFn(6);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = data_directives, set_directive => ActionFn(7);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action7::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // data_directives = data_directives, db_directive => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // data_directives = data_directives, dw_directive => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // db_directive = label, r#"(db|DB) "#, byte_num => ActionFn(52);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
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
        // db_directive = r#"(db|DB) "#, byte_num => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce26<
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
        // db_directive = label, r#"(db|DB) "#, "[", word_num, "]" => ActionFn(53);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action53::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 12)
    }
    pub(crate) fn __reduce27<
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
        // db_directive = r#"(db|DB) "#, "[", word_num, "]" => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 12)
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
        // db_directive = label, r#"(db|DB) "#, "[", word_num, ";", byte_num, "]" => ActionFn(54);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action54::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 12)
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
        // db_directive = r#"(db|DB) "#, "[", word_num, ";", byte_num, "]" => ActionFn(16);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action16::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 12)
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
        // dw_directive = label, r#"(dw|DW) "#, word_num => ActionFn(55);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action55::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 13)
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
        // dw_directive = r#"(dw|DW) "#, word_num => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action18::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 13)
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
        // dw_directive = label, r#"(dw|DW) "#, "[", word_num, "]" => ActionFn(56);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action56::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 13)
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
        // dw_directive = r#"(dw|DW) "#, "[", word_num, "]" => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 13)
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
        // dw_directive = label, r#"(dw|DW) "#, "[", word_num, ";", word_num, "]" => ActionFn(57);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action57::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 13)
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
        // dw_directive = r#"(dw|DW) "#, "[", word_num, ";", word_num, "]" => ActionFn(22);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action22::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 13)
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
        // macro_def = "MACRO", string, "(", CommaSepList<string>, ")", "{", raw_code, "}" => ActionFn(23);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action23::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (8, 15)
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
        // raw_code = string => ActionFn(24);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
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
        // raw_code = raw_code, string => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 16)
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
        // raw_code = raw_code, ";" => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 16)
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
        // set_directive = r#"(set|SET) "#, word_num => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
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
        // string = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 18)
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
        // string? = string => ActionFn(38);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
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
        // string? =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 19)
    }
}
pub use self::__parse__data_directives::data_directivesParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__macro_def {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::preprocessor_util as util;
    use util::LabelType;
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
        0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0,
        // State 2
        0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0,
        // State 3
        0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0,
        // State 5
        0, 0, 0, 18, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 9, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        -43, -43, -43, -43, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, -43, 0,
        // State 9
        0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, -9, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -11, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 14
        0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 15
        0, 0, 0, -39, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, -39, 0,
        // State 16
        0, 0, 0, -40, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, -40, 0,
        // State 17
        0, 0, 0, -41, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, -41, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 17 + integer]
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
        0,
        // State 6
        -15,
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
        0,
        // State 17
        0,
        // State 18
        -38,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 3,
            6 => 9,
            15 => 6,
            16 => 5,
            18 => match state {
                2 => 10,
                3 => 11,
                4 => 15,
                5 => 16,
                _ => 7,
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
            r###""MACRO""###,
            r###""[""###,
            r###""]""###,
            r###""{""###,
            r###""}""###,
            r###"r#"(db|DB) "#"###,
            r###"r#"(dw|DW) "#"###,
            r###"r#"(set|SET) "#"###,
            r###"r#"0(b|B)[0-1]+"#"###,
            r###"r#"0(x|X)[0-9A-Fa-f]+"#"###,
            r###"r#"[0-9]+"#"###,
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
            __action(state, 17 - 1)
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
            Token(8, _) if true => Some(0),
            Token(9, _) if true => Some(1),
            Token(10, _) if true => Some(2),
            Token(11, _) if true => Some(3),
            Token(12, _) if true => Some(4),
            Token(13, _) if true => Some(5),
            Token(14, _) if true => Some(6),
            Token(15, _) if true => Some(7),
            Token(16, _) if true => Some(8),
            Token(0, _) if true => Some(9),
            Token(1, _) if true => Some(10),
            Token(2, _) if true => Some(11),
            Token(3, _) if true => Some(12),
            Token(4, _) if true => Some(13),
            Token(5, _) if true => Some(14),
            Token(6, _) if true => Some(15),
            Token(7, _) if true => Some(16),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => match __token {
                Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) if true => __Symbol::Variant0(__tok0),
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
                // __macro_def = macro_def => ActionFn(2);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(context, out, input, __sym0);
                return Some(Ok(__nt));
            }
            15 => {
                // byte_num = r#"[0-9]+"# => ActionFn(61);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action61::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
            }
            16 => {
                // byte_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(62);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action62::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
            }
            17 => {
                // byte_num = r#"0(b|B)[0-1]+"# => ActionFn(63);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action63::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
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
                __reduce24(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
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
                // label = r#"[_a-zA-Z][_a-zA-Z0-9]*:"# => ActionFn(27);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action27::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 14)
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
                // word_num = r#"[0-9]+"# => ActionFn(64);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action64::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
            }
            46 => {
                // word_num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(65);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action65::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
            }
            47 => {
                // word_num = r#"0(b|B)[0-1]+"# => ActionFn(66);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action66::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant8(__nt), __end));
                (1, 20)
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
        // (<string> ",") = string, "," => ActionFn(42);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(context, out, input, __sym0, __sym1);
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
        // (<string> ",")* =  => ActionFn(40);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action40::<>(context, out, input, &__start, &__end);
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
        // (<string> ",")* = (<string> ",")+ => ActionFn(41);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(context, out, input, __sym0);
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
        // (<string> ",")+ = string, "," => ActionFn(45);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action45::<>(context, out, input, __sym0, __sym1);
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
        // (<string> ",")+ = (<string> ",")+, string, "," => ActionFn(46);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action46::<>(context, out, input, __sym0, __sym1, __sym2);
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
        // @L =  => ActionFn(37);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action37::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 3)
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
        // @R =  => ActionFn(35);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action35::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
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
        // Code = data_directives => ActionFn(3);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
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
        // CommaSepList<string> = string => ActionFn(67);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
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
        // CommaSepList<string> =  => ActionFn(68);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action68::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 6)
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
        // CommaSepList<string> = (<string> ",")+, string => ActionFn(69);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action69::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
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
        // CommaSepList<string> = (<string> ",")+ => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
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
        // __Code = Code => ActionFn(0);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 7)
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
        // __data_directives = data_directives => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
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
        // data_directives = set_directive => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = db_directive => ActionFn(5);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = dw_directive => ActionFn(6);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
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
        // data_directives = data_directives, set_directive => ActionFn(7);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action7::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // data_directives = data_directives, db_directive => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // data_directives = data_directives, dw_directive => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 11)
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
        // db_directive = label, r#"(db|DB) "#, byte_num => ActionFn(52);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 12)
    }
    pub(crate) fn __reduce25<
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
        // db_directive = r#"(db|DB) "#, byte_num => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 12)
    }
    pub(crate) fn __reduce26<
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
        // db_directive = label, r#"(db|DB) "#, "[", word_num, "]" => ActionFn(53);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action53::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 12)
    }
    pub(crate) fn __reduce27<
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
        // db_directive = r#"(db|DB) "#, "[", word_num, "]" => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 12)
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
        // db_directive = label, r#"(db|DB) "#, "[", word_num, ";", byte_num, "]" => ActionFn(54);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant6(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action54::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 12)
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
        // db_directive = r#"(db|DB) "#, "[", word_num, ";", byte_num, "]" => ActionFn(16);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action16::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 12)
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
        // dw_directive = label, r#"(dw|DW) "#, word_num => ActionFn(55);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action55::<>(context, out, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 13)
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
        // dw_directive = r#"(dw|DW) "#, word_num => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action18::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 13)
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
        // dw_directive = label, r#"(dw|DW) "#, "[", word_num, "]" => ActionFn(56);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action56::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (5, 13)
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
        // dw_directive = r#"(dw|DW) "#, "[", word_num, "]" => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(context, out, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (4, 13)
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
        // dw_directive = label, r#"(dw|DW) "#, "[", word_num, ";", word_num, "]" => ActionFn(57);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant8(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action57::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (7, 13)
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
        // dw_directive = r#"(dw|DW) "#, "[", word_num, ";", word_num, "]" => ActionFn(22);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant8(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant8(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action22::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (6, 13)
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
        // macro_def = "MACRO", string, "(", CommaSepList<string>, ")", "{", raw_code, "}" => ActionFn(23);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant1(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action23::<>(context, out, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (8, 15)
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
        // raw_code = string => ActionFn(24);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 16)
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
        // raw_code = raw_code, string => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 16)
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
        // raw_code = raw_code, ";" => ActionFn(26);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action26::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 16)
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
        // set_directive = r#"(set|SET) "#, word_num => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 17)
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
        // string = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 18)
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
        // string? = string => ActionFn(38);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
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
        // string? =  => ActionFn(39);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action39::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 19)
    }
}
pub use self::__parse__macro_def::macro_defParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::util::preprocessor_util as util;
    use util::LabelType;
    use crate::preprocessor_error;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^((db|DB) )", false),
            ("^((dw|DW) )", false),
            ("^((set|SET) )", false),
            ("^(0(b|B)[0-1]+)", false),
            ("^(0(x|X)[0-9A-Fa-f]+)", false),
            ("^([0-9]+)", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*:)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(,)", false),
            ("^(;)", false),
            ("^(MACRO)", false),
            ("^(\\[)", false),
            ("^(\\])", false),
            ("^(\\{)", false),
            ("^(\\})", false),
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
    (_, __1, _): (usize, (), usize),
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
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
) -> ()
{
    {
        out.data.push(format!("set {}",n));
    }
}

#[allow(unused_variables)]
fn __action11<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
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
fn __action12<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
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
fn __action13<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
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
fn __action14<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
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
fn __action15<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
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
fn __action16<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
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
fn __action17<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
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
fn __action18<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
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
fn __action19<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
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
fn __action20<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
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
fn __action21<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
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
fn __action22<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
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
fn __action23<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, params, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        println!("{}",r);

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
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
fn __action25<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, r, _): (usize, String, usize),
    (_, s, _): (usize, String, usize),
) -> String
{
    format!("{} {}",r,s)
}

#[allow(unused_variables)]
fn __action26<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    (_, r, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("{};\n",r)
}

#[allow(unused_variables)]
fn __action27<
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
fn __action28<
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
fn __action29<
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
fn __action30<
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
fn __action31<
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
fn __action32<
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
fn __action33<
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
fn __action34<
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
fn __action35<
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
fn __action36<
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
fn __action37<
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
fn __action38<
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
fn __action39<
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
fn __action40<
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
fn __action41<
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
fn __action42<
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
fn __action43<
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
fn __action44<
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
fn __action45<
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
    let __temp0 = __action42(
        context,
        out,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        context,
        out,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action46<
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
    let __temp0 = __action42(
        context,
        out,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action47<
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
    let __temp0 = __action40(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        context,
        out,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action48<
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
    let __temp0 = __action41(
        context,
        out,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        context,
        out,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action49<
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
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action50<
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
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action51<
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
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, u8, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
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
fn __action53<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u16, usize),
    __4: (usize, &'input str, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
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
fn __action54<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u16, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, u8, usize),
    __6: (usize, &'input str, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
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
fn __action55<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, u16, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
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
fn __action56<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u16, usize),
    __4: (usize, &'input str, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
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
fn __action57<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut util::Output,
    input: &'input str,
    __0: (usize, String, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, u16, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, u16, usize),
    __6: (usize, &'input str, usize),
) -> ()
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
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
fn __action58<
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
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action59<
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
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action60<
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
    let __temp0 = __action37(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action61<
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
    let __temp0 = __action35(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
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
    let __temp0 = __action35(
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
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
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
    let __temp0 = __action35(
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
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
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
    let __temp0 = __action35(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
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
    let __temp0 = __action35(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action66<
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
    let __temp0 = __action35(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        context,
        out,
        input,
        __0,
        __temp0,
    )
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
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action38(
        context,
        out,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
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
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action39(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        context,
        out,
        input,
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
    __0: (usize, ::std::vec::Vec<String>, usize),
    __1: (usize, String, usize),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        context,
        out,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        context,
        out,
        input,
        __0,
        __temp0,
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
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action39(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
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
