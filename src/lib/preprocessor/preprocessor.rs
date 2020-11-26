// auto-generated: "lalrpop 0.19.1"
// sha256: 3122b5759eff36d5cbbda46ac7dd8ae6e9cee810ce76737c7ac1a8f4925dea
use crate::util::preprocessor_util as util;
use crate::preprocessor_error;
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__code {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::util::preprocessor_util as util;
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
        Variant1(usize),
        Variant2(()),
        Variant3(u16),
    }
    const __ACTION: &[i8] = &[
        // State 0
        4, 5, 6,
        // State 1
        4, 5, 6,
        // State 2
        -4, -4, -4,
        // State 3
        -8, -8, -8,
        // State 4
        -7, -7, -7,
        // State 5
        -6, -6, -6,
        // State 6
        -5, -5, -5,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 3 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -3,
        // State 2
        -4,
        // State 3
        -8,
        // State 4
        -7,
        // State 5
        -6,
        // State 6
        -5,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => 1,
            4 => match state {
                1 => 6,
                _ => 2,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###"r#"0(b|B)[0-1]+"#"###,
            r###"r#"0(x|X)[0-9A-Fa-f]+"#"###,
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
    pub struct __StateMachine<'input, 's>
    where 
    {
        context: &'s mut util::Context,
        out: &'s mut Vec<String>,
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
            __action(state, 3 - 1)
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
            Token(0, _) if true => Some(0),
            Token(1, _) if true => Some(1),
            Token(2, _) if true => Some(2),
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
            0 | 1 | 2 => match __token {
                Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct codeParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl codeParser {
        pub fn new() -> codeParser {
            let __builder = super::__intern_token::new_builder();
            codeParser {
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
            out: &'s mut Vec<String>,
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
        out: &'s mut Vec<String>,
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
                // __code = code => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(context, out, input, __sym0);
                return Some(Ok(__nt));
            }
            3 => {
                __reduce3(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(context, out, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                // num = r#"[0-9]+"# => ActionFn(11);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action11::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 4)
            }
            6 => {
                // num = r#"0(x|X)[0-9A-Fa-f]+"# => ActionFn(12);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action12::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 4)
            }
            7 => {
                // num = r#"0(b|B)[0-1]+"# => ActionFn(13);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action13::<>(context, out, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 4)
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
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u16, usize)
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
        context: &'s mut util::Context,
        out: &'s mut Vec<String>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(7);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action7::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut Vec<String>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(6);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action6::<>(context, out, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut Vec<String>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code = num => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(context, out, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        context: &'s mut util::Context,
        out: &'s mut Vec<String>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // code = code, num => ActionFn(2);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action2::<>(context, out, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 3)
    }
}
pub use self::__parse__code::codeParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::util::preprocessor_util as util;
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
    out: &'s mut Vec<String>,
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
    out: &'s mut Vec<String>,
    input: &'input str,
    (_, n, _): (usize, u16, usize),
) -> ()
{
    {out.push(format!("{}",n));}
}

#[allow(unused_variables)]
fn __action2<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    (_, _, _): (usize, (), usize),
    (_, n, _): (usize, u16, usize),
) -> ()
{
    {out.push(format!("{}",n));}
}

#[allow(unused_variables)]
fn __action3<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
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
fn __action4<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
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
fn __action5<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
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
fn __action6<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action7<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action8<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action7(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action9<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action7(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action10<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action7(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        context,
        out,
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action11<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action6(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action12<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action6(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        context,
        out,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action13<
    'input,
    's,
>(
    context: &'s mut util::Context,
    out: &'s mut Vec<String>,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<u16,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action6(
        context,
        out,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
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
