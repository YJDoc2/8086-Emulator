// auto-generated: "lalrpop 0.19.1"
// sha256: a0f392c8d6adaf198aba24ca11b0fbbea2458beb6d2af62554928e2d419ce1e
use crate::util::interpreter_util::{Context,State};
use crate::util::preprocessor_util::LabelType;
use crate::util::flag_util::*;
use crate::arch::FLAG_CARRY;
use crate::vm::VM;
use crate::error;
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Interpreter {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::interpreter_util::{Context,State};
    use crate::util::preprocessor_util::LabelType;
    use crate::util::flag_util::*;
    use crate::arch::FLAG_CARRY;
    use crate::vm::VM;
    use crate::error;
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
        Variant1(usize),
        Variant2(State),
        Variant3(bool),
        Variant4(String),
    }
    const __ACTION: &[i8] = &[
        // State 0
        3, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 34 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        -5,
        // State 4
        -41,
        // State 5
        -3,
        // State 6
        -43,
        // State 7
        -42,
        // State 8
        -4,
        // State 9
        -8,
        // State 10
        -11,
        // State 11
        -13,
        // State 12
        -9,
        // State 13
        -14,
        // State 14
        0,
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
        0,
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
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        -40,
        // State 38
        -7,
        // State 39
        -10,
        // State 40
        -12,
        // State 41
        -38,
        // State 42
        -39,
        // State 43
        -6,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 3,
            4 => 4,
            5 => 5,
            6 => 1,
            7 => 6,
            8 => match state {
                2 => 43,
                _ => 41,
            },
            9 => 7,
            10 => 8,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""call""###,
            r###""clc""###,
            r###""cld""###,
            r###""cli""###,
            r###""cmc""###,
            r###""hlt""###,
            r###""ja""###,
            r###""jae""###,
            r###""jb""###,
            r###""jbe""###,
            r###""jc""###,
            r###""jcxz""###,
            r###""je""###,
            r###""jg""###,
            r###""jge""###,
            r###""jl""###,
            r###""jle""###,
            r###""jmp""###,
            r###""jnc""###,
            r###""jne""###,
            r###""jno""###,
            r###""jnp""###,
            r###""jns""###,
            r###""jo""###,
            r###""jp""###,
            r###""js""###,
            r###""loop""###,
            r###""loope""###,
            r###""loopne""###,
            r###""ret""###,
            r###""stc""###,
            r###""std""###,
            r###""sti""###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*"#"###,
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
    pub struct __StateMachine<'input, 's, '__2>
    where 
    {
        current: usize,
        vm: &'__2 mut VM,
        context: &'s mut Context,
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input (), &'s ())>,
    }
    impl<'input, 's, '__2> __state_machine::ParserDefinition for __StateMachine<'input, 's, '__2>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = State;
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
            __action(state, 34 - 1)
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
                self.current,
                self.vm,
                self.context,
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
            Token(1, _) if true => Some(0),
            Token(2, _) if true => Some(1),
            Token(3, _) if true => Some(2),
            Token(4, _) if true => Some(3),
            Token(5, _) if true => Some(4),
            Token(6, _) if true => Some(5),
            Token(7, _) if true => Some(6),
            Token(8, _) if true => Some(7),
            Token(9, _) if true => Some(8),
            Token(10, _) if true => Some(9),
            Token(11, _) if true => Some(10),
            Token(12, _) if true => Some(11),
            Token(13, _) if true => Some(12),
            Token(14, _) if true => Some(13),
            Token(15, _) if true => Some(14),
            Token(16, _) if true => Some(15),
            Token(17, _) if true => Some(16),
            Token(18, _) if true => Some(17),
            Token(19, _) if true => Some(18),
            Token(20, _) if true => Some(19),
            Token(21, _) if true => Some(20),
            Token(22, _) if true => Some(21),
            Token(23, _) if true => Some(22),
            Token(24, _) if true => Some(23),
            Token(25, _) if true => Some(24),
            Token(26, _) if true => Some(25),
            Token(27, _) if true => Some(26),
            Token(28, _) if true => Some(27),
            Token(29, _) if true => Some(28),
            Token(30, _) if true => Some(29),
            Token(31, _) if true => Some(30),
            Token(32, _) if true => Some(31),
            Token(33, _) if true => Some(32),
            Token(0, _) if true => Some(33),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 => match __token {
                Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(32, __tok0) | Token(33, __tok0) | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct InterpreterParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl InterpreterParser {
        pub fn new() -> InterpreterParser {
            let __builder = super::__intern_token::new_builder();
            InterpreterParser {
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
            current: usize,
            vm: &mut VM,
            context: &'s mut Context,
            input: &'input str,
        ) -> Result<State, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    current,
                    vm,
                    context,
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
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<Result<State,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                // __Interpreter = Interpreter => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(current, vm, context, input, __sym0);
                return Some(Ok(__nt));
            }
            5 => {
                // call = "call", name_string => ActionFn(46);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant4(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action46::<>(current, vm, context, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (2, 4)
            }
            6 => {
                __reduce6(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                // jumps_loops = jumps_condition, name_string => ActionFn(47);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant4(__symbols);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action47::<>(current, vm, context, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (2, 7)
            }
            38 => {
                __reduce38(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                // ret = "ret" => ActionFn(48);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action48::<>(current, vm, context, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (1, 9)
            }
            40 => {
                __reduce40(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            41 => {
                __reduce41(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            42 => {
                __reduce42(current, vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
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
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, State, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
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
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(42);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action42::<>(current, vm, context, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(41);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action41::<>(current, vm, context, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Interpreter = control => ActionFn(1);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Interpreter = transfer => ActionFn(2);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "stc" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce7<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "clc" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce8<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "cmc" => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce9<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "std" => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce10<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "cld" => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce11<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "sti" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce12<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "cli" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce13<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "hlt" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce14<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jmp" => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce15<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "ja" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce16<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jae" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce17<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jb" => ActionFn(12);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce18<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jbe" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce19<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jc" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce20<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "je" => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce21<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jg" => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce22<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jge" => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce23<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jl" => ActionFn(18);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce24<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jle" => ActionFn(19);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce25<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jnc" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce26<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jne" => ActionFn(21);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce27<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jno" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce28<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jnp" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce29<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jns" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce30<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jo" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce31<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jp" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce32<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "js" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce33<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "jcxz" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce34<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "loop" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce35<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "loope" => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce36<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // jumps_condition = "loopne" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce38<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // name_string = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce40<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // transfer = call => ActionFn(3);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce41<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // transfer = ret => ActionFn(4);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce42<
        'input,
        's,
    >(
        current: usize,
        vm: &mut VM,
        context: &'s mut Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // transfer = jumps_loops => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(current, vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 10)
    }
}
pub use self::__parse__Interpreter::InterpreterParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::util::interpreter_util::{Context,State};
    use crate::util::preprocessor_util::LabelType;
    use crate::util::flag_util::*;
    use crate::arch::FLAG_CARRY;
    use crate::vm::VM;
    use crate::error;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^([A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("^(call)", false),
            ("^(clc)", false),
            ("^(cld)", false),
            ("^(cli)", false),
            ("^(cmc)", false),
            ("^(hlt)", false),
            ("^(ja)", false),
            ("^(jae)", false),
            ("^(jb)", false),
            ("^(jbe)", false),
            ("^(jc)", false),
            ("^(jcxz)", false),
            ("^(je)", false),
            ("^(jg)", false),
            ("^(jge)", false),
            ("^(jl)", false),
            ("^(jle)", false),
            ("^(jmp)", false),
            ("^(jnc)", false),
            ("^(jne)", false),
            ("^(jno)", false),
            ("^(jnp)", false),
            ("^(jns)", false),
            ("^(jo)", false),
            ("^(jp)", false),
            ("^(js)", false),
            ("^(loop)", false),
            ("^(loope)", false),
            ("^(loopne)", false),
            ("^(ret)", false),
            ("^(stc)", false),
            ("^(std)", false),
            ("^(sti)", false),
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
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, State, usize),
) -> State
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, State, usize),
) -> State
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, State, usize),
) -> State
{
    __0
}

#[allow(unused_variables)]
fn __action3<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, State, usize),
) -> State
{
    __0
}

#[allow(unused_variables)]
fn __action4<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, State, usize),
) -> State
{
    __0
}

#[allow(unused_variables)]
fn __action5<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, State, usize),
) -> State
{
    __0
}

#[allow(unused_variables)]
fn __action6<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, String, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
       match context.fn_map.get(&n){
         Some(pos) => {context.call_stack.push(current); Ok(State::JMP(*pos)) },
         None => error!(start,end,format!("Internal Error : call to undefined procedure {}",n))
       }
    }
}

#[allow(unused_variables)]
fn __action7<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
      match context.call_stack.pop() {
         Some(p) => {Ok(State::JMP(p))},
         None => error!(start,end,"Error : ret is encountered without corresponding call".to_owned())
      }
   }
}

#[allow(unused_variables)]
fn __action8<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, take, _): (usize, bool, usize),
    (_, n, _): (usize, String, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
       match context.label_map.get(&n){

          Some(l) => {

             match l.get_type(){
               
               LabelType::DATA => error!(start,end,format!("Internal Error : jump to data type label {}",n)),
               
               LabelType::CODE => {
                  if take {
                     Ok(State::JMP(l.map))
                  }else{
                     Ok(State::NEXT)
                  }
               }
             }
          }
          // No label found
          None => error!(start,end,format!("Internal Error : jump to undefined label {}",n))
       }
    }
}

#[allow(unused_variables)]
fn __action9<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    true
}

#[allow(unused_variables)]
fn __action10<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    !get_flag_state(vm.arch.flag,Flags::CARRY) && !get_flag_state(vm.arch.flag,Flags::ZERO)
}

#[allow(unused_variables)]
fn __action11<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    !get_flag_state(vm.arch.flag,Flags::CARRY)
}

#[allow(unused_variables)]
fn __action12<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::CARRY)
}

#[allow(unused_variables)]
fn __action13<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::CARRY) || get_flag_state(vm.arch.flag,Flags::ZERO)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::CARRY)
}

#[allow(unused_variables)]
fn __action15<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::ZERO)
}

#[allow(unused_variables)]
fn __action16<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    !get_flag_state(vm.arch.flag,Flags::ZERO) && get_flag_state(vm.arch.flag,Flags::SIGN) == get_flag_state(vm.arch.flag,Flags::OVERFLOW)
}

#[allow(unused_variables)]
fn __action17<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::SIGN) == get_flag_state(vm.arch.flag,Flags::OVERFLOW)
}

#[allow(unused_variables)]
fn __action18<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::SIGN) != get_flag_state(vm.arch.flag,Flags::OVERFLOW)
}

#[allow(unused_variables)]
fn __action19<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::ZERO) && get_flag_state(vm.arch.flag,Flags::SIGN) != get_flag_state(vm.arch.flag,Flags::OVERFLOW)
}

#[allow(unused_variables)]
fn __action20<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    !get_flag_state(vm.arch.flag,Flags::CARRY)
}

#[allow(unused_variables)]
fn __action21<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    !get_flag_state(vm.arch.flag,Flags::ZERO)
}

#[allow(unused_variables)]
fn __action22<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    !get_flag_state(vm.arch.flag,Flags::OVERFLOW)
}

#[allow(unused_variables)]
fn __action23<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    !get_flag_state(vm.arch.flag,Flags::PARITY)
}

#[allow(unused_variables)]
fn __action24<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    !get_flag_state(vm.arch.flag,Flags::SIGN)
}

#[allow(unused_variables)]
fn __action25<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::OVERFLOW)
}

#[allow(unused_variables)]
fn __action26<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::PARITY)
}

#[allow(unused_variables)]
fn __action27<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    get_flag_state(vm.arch.flag,Flags::SIGN)
}

#[allow(unused_variables)]
fn __action28<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    vm.arch.cx == 0
}

#[allow(unused_variables)]
fn __action29<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    {
      vm.arch.cx -= 1;
      vm.arch.cx != 0
   }
}

#[allow(unused_variables)]
fn __action30<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    {
      vm.arch.cx -= 1;
      vm.arch.cx != 0 && get_flag_state(vm.arch.flag,Flags::ZERO)
   }
}

#[allow(unused_variables)]
fn __action31<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    {
      vm.arch.cx -= 1;
      vm.arch.cx != 0 && !get_flag_state(vm.arch.flag,Flags::ZERO)
   }
}

#[allow(unused_variables)]
fn __action32<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {set_flag(&mut vm.arch.flag,Flags::CARRY); State::NEXT}
}

#[allow(unused_variables)]
fn __action33<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {unset_flag(&mut vm.arch.flag,Flags::CARRY); State::NEXT}
}

#[allow(unused_variables)]
fn __action34<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {if (vm.arch.flag & FLAG_CARRY) != 0 {vm.arch.flag &= !FLAG_CARRY}else{vm.arch.flag |= FLAG_CARRY}; State::NEXT}
}

#[allow(unused_variables)]
fn __action35<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {set_flag(&mut vm.arch.flag,Flags::DIRECTION); State::NEXT}
}

#[allow(unused_variables)]
fn __action36<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {unset_flag(&mut vm.arch.flag,Flags::DIRECTION); State::NEXT}
}

#[allow(unused_variables)]
fn __action37<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {set_flag(&mut vm.arch.flag,Flags::INTERRUPT); State::NEXT}
}

#[allow(unused_variables)]
fn __action38<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {unset_flag(&mut vm.arch.flag,Flags::INTERRUPT); State::NEXT}
}

#[allow(unused_variables)]
fn __action39<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    State::HALT
}

#[allow(unused_variables)]
fn __action40<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    (_, v, _): (usize, &'input str, usize),
) -> String
{
    {
        v.to_owned()
    }
}

#[allow(unused_variables)]
fn __action41<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action42<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action43<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, usize, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action42(
        current,
        vm,
        context,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        current,
        vm,
        context,
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action44<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    __0: (usize, bool, usize),
    __1: (usize, String, usize),
    __2: (usize, usize, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action42(
        current,
        vm,
        context,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        current,
        vm,
        context,
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action45<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action42(
        current,
        vm,
        context,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        current,
        vm,
        context,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action46<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action41(
        current,
        vm,
        context,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        current,
        vm,
        context,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action47<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    __0: (usize, bool, usize),
    __1: (usize, String, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action41(
        current,
        vm,
        context,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        current,
        vm,
        context,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action48<
    'input,
    's,
>(
    current: usize,
    vm: &mut VM,
    context: &'s mut Context,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<State,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action41(
        current,
        vm,
        context,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        current,
        vm,
        context,
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
