// auto-generated: "lalrpop 0.19.1"
// sha256: 343fe112a32cced8d01b96e3fae44fbd133a0a5afacdf923022cbf589a6e4f
use crate::util::{address::*,data_util::separate_bytes};
use crate::vm::VM;
use crate::error;
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Data {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::{address::*,data_util::separate_bytes};
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
        Variant2(()),
        Variant3(i8),
        Variant4(i16),
        Variant5(u8),
        Variant6(u16),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 2, 3, 4, 0, 0, 0,
        // State 1
        0, 5, 0, 0, 0, 0, 15, 16, 17,
        // State 2
        0, 6, 0, 0, 0, 0, 20, 21, 22,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 16, 26,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        -16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        -15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        -17, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -21, 0, -21, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        7, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 29, 0, 0, 0, 0, 0, 0,
        // State 25
        -20, 0, -21, 0, 0, 0, 0, 0, 0,
        // State 26
        8, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        -18, 0, 30, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 33, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 34, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 9 + integer]
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
        0,
        // State 7
        0,
        // State 8
        -6,
        // State 9
        -4,
        // State 10
        -5,
        // State 11
        -3,
        // State 12
        -7,
        // State 13
        -16,
        // State 14
        -10,
        // State 15
        -15,
        // State 16
        -20,
        // State 17
        -11,
        // State 18
        -18,
        // State 19
        -14,
        // State 20
        -17,
        // State 21
        -21,
        // State 22
        -19,
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
        -8,
        // State 29
        -12,
        // State 30
        0,
        // State 31
        0,
        // State 32
        -9,
        // State 33
        -13,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 8,
            4 => 9,
            5 => 10,
            6 => match state {
                4 => 23,
                _ => 12,
            },
            7 => match state {
                5 => 26,
                _ => 17,
            },
            8 => 11,
            9 => 13,
            10 => match state {
                3 => 22,
                4 => 24,
                5 => 27,
                6 => 30,
                7 => 31,
                _ => 18,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###"",""###,
            r###""[""###,
            r###""]""###,
            r###""db""###,
            r###""dw""###,
            r###""set""###,
            r###"r#"\"[[:ascii:]]*\""#"###,
            r###"r#"-[0-9]+"#"###,
            r###"r#"[0-9]+"#"###,
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
        vm: &'s mut VM,
        counter: &'__2 mut usize,
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
            __action(state, 9 - 1)
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
                self.vm,
                self.counter,
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
            Token(3, _) if true => Some(0),
            Token(4, _) if true => Some(1),
            Token(5, _) if true => Some(2),
            Token(6, _) if true => Some(3),
            Token(7, _) if true => Some(4),
            Token(8, _) if true => Some(5),
            Token(0, _) if true => Some(6),
            Token(1, _) if true => Some(7),
            Token(2, _) if true => Some(8),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => match __token {
                Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct DataParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl DataParser {
        pub fn new() -> DataParser {
            let __builder = super::__intern_token::new_builder();
            DataParser {
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
            vm: &'s mut VM,
            counter: &mut usize,
            input: &'input str,
        ) -> Result<(), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    vm,
                    counter,
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
        vm: &'s mut VM,
        counter: &mut usize,
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
                __reduce0(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                // __Data = Data => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(vm, counter, input, __sym0);
                return Some(Ok(__nt));
            }
            6 => {
                __reduce6(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                // s_byte_num = r#"-[0-9]+"# => ActionFn(25);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action25::<>(vm, counter, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 6)
            }
            15 => {
                __reduce15(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                // s_word_num = r#"-[0-9]+"# => ActionFn(26);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action26::<>(vm, counter, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant4(__nt), __end));
                (1, 7)
            }
            17 => {
                __reduce17(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(vm, counter, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                // u_byte_num = r#"[0-9]+"# => ActionFn(27);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action27::<>(vm, counter, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant5(__nt), __end));
                (1, 9)
            }
            20 => {
                // u_word_num = r#"[0-9]+"# => ActionFn(28);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action28::<>(vm, counter, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 10)
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
    ) -> (usize, (), usize)
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
    ) -> (usize, i16, usize)
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
    ) -> (usize, i8, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u16, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
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
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(20);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action20::<>(vm, counter, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(19);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action19::<>(vm, counter, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Data = set => ActionFn(1);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(vm, counter, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Data = db => ActionFn(2);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(vm, counter, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Data = dw => ActionFn(3);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(vm, counter, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db = "db", s_byte_num => ActionFn(5);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action5::<>(vm, counter, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce7<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db = "db", "[", u_word_num, "]" => ActionFn(6);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action6::<>(vm, counter, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (4, 4)
    }
    pub(crate) fn __reduce8<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db = "db", "[", s_byte_num, ",", u_word_num, "]" => ActionFn(7);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action7::<>(vm, counter, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (6, 4)
    }
    pub(crate) fn __reduce9<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // db = "db", r#"\"[[:ascii:]]*\""# => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(vm, counter, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 4)
    }
    pub(crate) fn __reduce10<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw = "dw", s_word_num => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(vm, counter, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce11<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw = "dw", "[", u_word_num, "]" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(vm, counter, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (4, 5)
    }
    pub(crate) fn __reduce12<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw = "dw", "[", s_word_num, ",", u_word_num, "]" => ActionFn(11);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action11::<>(vm, counter, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (6, 5)
    }
    pub(crate) fn __reduce13<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // dw = "dw", r#"\"[[:ascii:]]*\""# => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(vm, counter, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce15<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // s_byte_num = u_byte_num => ActionFn(18);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(vm, counter, input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce17<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // s_word_num = u_word_num => ActionFn(16);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(vm, counter, input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce18<
        'input,
        's,
    >(
        vm: &'s mut VM,
        counter: &mut usize,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // set = "set", u_word_num => ActionFn(4);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action4::<>(vm, counter, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 8)
    }
}
pub use self::__parse__Data::DataParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::util::{address::*,data_util::separate_bytes};
    use crate::vm::VM;
    use crate::error;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(\"[\u{0}-\u{7f}]*\")", false),
            ("^(\\-[0-9]+)", false),
            ("^([0-9]+)", false),
            ("^(,)", false),
            ("^(\\[)", false),
            ("^(\\])", false),
            ("^(db)", false),
            ("^(dw)", false),
            ("^(set)", false),
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
    vm: &'s mut VM,
    counter: &mut usize,
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
    vm: &'s mut VM,
    counter: &mut usize,
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
    vm: &'s mut VM,
    counter: &mut usize,
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
    vm: &'s mut VM,
    counter: &mut usize,
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
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
) -> ()
{
    {
        vm.arch.ds = n;
        *counter =0;
    }
}

#[allow(unused_variables)]
fn __action5<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, i8, usize),
) -> ()
{
    { 
        let addr = Address::calculate_from_offset(vm.arch.ds,*counter);
        vm.mem[addr] = n as u8;
        *counter += 1;
    }
}

#[allow(unused_variables)]
fn __action6<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        let mut addr = Address::calculate_from_offset(vm.arch.ds,*counter);
        for _ in 0..n{
            vm.mem[addr] = 0;
            addr = inc_addr(addr,1);
        }
        *counter += n as usize;
    }
}

#[allow(unused_variables)]
fn __action7<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, i8, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        let mut addr = Address::calculate_from_offset(vm.arch.ds,*counter);
        for _ in 0..n{
            vm.mem[addr] = v as u8;
            addr = inc_addr(addr,1);
        }
        *counter += n as usize;
    }
}

#[allow(unused_variables)]
fn __action8<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, q, _): (usize, &'input str, usize),
) -> ()
{
    {
        let mut addr = Address::calculate_from_offset(vm.arch.ds,*counter);
        // the slice skips the quotes
        for i in (&q[1..q.len()-1]).bytes(){
            vm.mem[addr] = i;
            addr =inc_addr(addr,1);
        }
        *counter += q.len() -2;
    }
}

#[allow(unused_variables)]
fn __action9<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, i16, usize),
) -> ()
{
    {
        let addr = Address::calculate_from_offset(vm.arch.ds,*counter);
        let (hb,lb) = separate_bytes(n); 
        vm.mem[addr] = lb;
        vm.mem[inc_addr(addr,1)] = hb;
        *counter += 2 as usize;
    }
}

#[allow(unused_variables)]
fn __action10<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        let mut addr = Address::calculate_from_offset(vm.arch.ds,*counter);
        for _ in 0..2*n{
            vm.mem[addr] = 0;
            addr = inc_addr(addr,1);
        }
        *counter += 2*n as usize;
    }
}

#[allow(unused_variables)]
fn __action11<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, i16, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u16, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ()
{
    {
        let mut addr = Address::calculate_from_offset(vm.arch.ds,*counter);
        let (hb,lb) = separate_bytes(v); 
        for _ in 0..n{
            vm.mem[addr] = lb;
            addr = inc_addr(addr,1);
            vm.mem[addr] = hb;
            addr = inc_addr(addr,1);
        }
        *counter += 2*n as usize;
    }
}

#[allow(unused_variables)]
fn __action12<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, q, _): (usize, &'input str, usize),
) -> ()
{
    {
        let mut addr = Address::calculate_from_offset(vm.arch.ds,*counter);
        // the slice skips the quotes
        for i in (&q[1..q.len()-1]).bytes(){
            vm.mem[addr] = i;
            addr = inc_addr(addr,1);
            vm.mem[addr] = 0;
            addr = inc_addr(addr,1);
        }
        *counter += 2*(q.len()-2);
    }
}

#[allow(unused_variables)]
fn __action13<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match u16::from_str_radix(n,10){
            Ok(v) => Ok(v),
            Err(_) => error!(start,end,"Invalid Value, must be between 0-65535".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action14<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match u8::from_str_radix(n,10){
            Ok(v) => Ok(v),
            Err(_) => error!(start,end,"Invalid Value, must be between 0-255".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action15<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<i16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match i16::from_str_radix(n,10){
            Ok(v) => Ok(v),
            Err(_) => error!(start,end,"Invalid Value, must be between 0-65535".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action16<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, n, _): (usize, u16, usize),
) -> i16
{
    n as i16
}

#[allow(unused_variables)]
fn __action17<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<i8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match i8::from_str_radix(n,10){
            Ok(v) => Ok(v),
            Err(_) => error!(start,end,"Invalid Value, must be between 0-255".to_owned())
        }
    }
}

#[allow(unused_variables)]
fn __action18<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    (_, n, _): (usize, u8, usize),
) -> i8
{
    n as i8
}

#[allow(unused_variables)]
fn __action19<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action20<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action21<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<i8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action20(
        vm,
        counter,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        vm,
        counter,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action22<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<i16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action20(
        vm,
        counter,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        vm,
        counter,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action23<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action20(
        vm,
        counter,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        vm,
        counter,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action24<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action20(
        vm,
        counter,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        vm,
        counter,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action25<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<i8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action19(
        vm,
        counter,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        vm,
        counter,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<i16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action19(
        vm,
        counter,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        vm,
        counter,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action27<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u8,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action19(
        vm,
        counter,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        vm,
        counter,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
    's,
>(
    vm: &'s mut VM,
    counter: &mut usize,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action19(
        vm,
        counter,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        vm,
        counter,
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
