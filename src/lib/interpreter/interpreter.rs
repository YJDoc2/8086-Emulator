// auto-generated: "lalrpop 0.19.1"
// sha256: cd8bd1871ae51c927dddc06edf6eb464aa058a2ac6a41901ad524b8ed5b445
use crate::util::interpreter_util::{Context,State};
use crate::util::flag_util::*;
use crate::arch::FLAG_CARRY;
use crate::vm::VM;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Interpreter {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::interpreter_util::{Context,State};
    use crate::util::flag_util::*;
    use crate::arch::FLAG_CARRY;
    use crate::vm::VM;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(State),
    }
    const __ACTION: &[i8] = &[
        // State 0
        4, 5, 6, 7, 8, 9, 10, 11,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 8 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -2,
        // State 2
        -1,
        // State 3
        -4,
        // State 4
        -7,
        // State 5
        -9,
        // State 6
        -5,
        // State 7
        -10,
        // State 8
        -3,
        // State 9
        -6,
        // State 10
        -8,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 1,
            2 => 2,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""clc""###,
            r###""cld""###,
            r###""cli""###,
            r###""cmc""###,
            r###""hlt""###,
            r###""stc""###,
            r###""std""###,
            r###""sti""###,
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
        vm: &'__2 mut VM,
        context: &'s Context,
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
            __action(state, 8 - 1)
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
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
            Token(3, _) if true => Some(3),
            Token(4, _) if true => Some(4),
            Token(5, _) if true => Some(5),
            Token(6, _) if true => Some(6),
            Token(7, _) if true => Some(7),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) if true => __Symbol::Variant0(__tok0),
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
            vm: &mut VM,
            context: &'s Context,
            input: &'input str,
        ) -> Result<State, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
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
        vm: &mut VM,
        context: &'s Context,
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
                __reduce0(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                // __Interpreter = Interpreter => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(vm, context, input, __sym0);
                return Some(Ok(__nt));
            }
            2 => {
                __reduce2(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(vm, context, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
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
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, State, usize)
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
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Interpreter = control => ActionFn(1);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "stc" => ActionFn(2);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "clc" => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "cmc" => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        's,
    >(
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "std" => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "cld" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce7<
        'input,
        's,
    >(
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "sti" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce8<
        'input,
        's,
    >(
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "cli" => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce9<
        'input,
        's,
    >(
        vm: &mut VM,
        context: &'s Context,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // control = "hlt" => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(vm, context, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
}
pub use self::__parse__Interpreter::InterpreterParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::util::interpreter_util::{Context,State};
    use crate::util::flag_util::*;
    use crate::arch::FLAG_CARRY;
    use crate::vm::VM;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(clc)", false),
            ("^(cld)", false),
            ("^(cli)", false),
            ("^(cmc)", false),
            ("^(hlt)", false),
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
    vm: &mut VM,
    context: &'s Context,
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
    vm: &mut VM,
    context: &'s Context,
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
    vm: &mut VM,
    context: &'s Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {set_flag(&mut vm.arch.flag,Flags::CARRY); State::NEXT}
}

#[allow(unused_variables)]
fn __action3<
    'input,
    's,
>(
    vm: &mut VM,
    context: &'s Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {unset_flag(&mut vm.arch.flag,Flags::CARRY); State::NEXT}
}

#[allow(unused_variables)]
fn __action4<
    'input,
    's,
>(
    vm: &mut VM,
    context: &'s Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {if (vm.arch.flag & FLAG_CARRY) != 0 {vm.arch.flag &= !FLAG_CARRY}else{vm.arch.flag |= FLAG_CARRY}; State::NEXT}
}

#[allow(unused_variables)]
fn __action5<
    'input,
    's,
>(
    vm: &mut VM,
    context: &'s Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {set_flag(&mut vm.arch.flag,Flags::DIRECTION); State::NEXT}
}

#[allow(unused_variables)]
fn __action6<
    'input,
    's,
>(
    vm: &mut VM,
    context: &'s Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {unset_flag(&mut vm.arch.flag,Flags::DIRECTION); State::NEXT}
}

#[allow(unused_variables)]
fn __action7<
    'input,
    's,
>(
    vm: &mut VM,
    context: &'s Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {set_flag(&mut vm.arch.flag,Flags::INTERRUPT); State::NEXT}
}

#[allow(unused_variables)]
fn __action8<
    'input,
    's,
>(
    vm: &mut VM,
    context: &'s Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    {unset_flag(&mut vm.arch.flag,Flags::INTERRUPT); State::NEXT}
}

#[allow(unused_variables)]
fn __action9<
    'input,
    's,
>(
    vm: &mut VM,
    context: &'s Context,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> State
{
    State::HALT
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
