// auto-generated: "lalrpop 0.19.1"
// sha256: 835bfbf79129e774617127bf2841a6f186cc4172e36e8c3bed93dc93372934e
use emulator_8086_lib as lib;
use lib::vm::{VM,MB};
use lib::{get_flag_state, Flags};
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Print {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use emulator_8086_lib as lib;
    use lib::vm::{VM,MB};
    use lib::{get_flag_state, Flags};
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
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 7, 0, 0,
        // State 1
        0, 3, 0, 0, 0, 0, 11,
        // State 2
        0, 0, 0, 0, 0, 0, 11,
        // State 3
        0, 0, 0, 0, 0, 0, 11,
        // State 4
        0, 0, 0, 0, 0, 0, 11,
        // State 5
        0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 8, 2, 0, 9, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0,
        // State 9
        4, 5, 0, 0, 0, 0, 0,
        // State 10
        -9, -9, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 7 + integer]
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
        -8,
        // State 6
        0,
        // State 7
        -3,
        // State 8
        -4,
        // State 9
        0,
        // State 10
        -9,
        // State 11
        -7,
        // State 12
        -5,
        // State 13
        -6,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 5,
            4 => match state {
                2 => 11,
                3 => 12,
                4 => 13,
                _ => 9,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""->""###,
            r###"":""###,
            r###""flags""###,
            r###""mem""###,
            r###""print""###,
            r###""reg""###,
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
        vm: &'__2 VM,
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
            __action(state, 7 - 1)
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
            Token(0, _) if true => Some(6),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 => match __token {
                Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct PrintParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl PrintParser {
        pub fn new() -> PrintParser {
            let __builder = super::__intern_token::new_builder();
            PrintParser {
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
            vm: &VM,
            input: &'input str,
        ) -> Result<(), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    vm,
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
        vm: &VM,
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
                __reduce0(vm, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(vm, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(vm, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(vm, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(vm, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                // Print = "print", "mem", raw_addr, ":", raw_addr => ActionFn(4);
                assert!(__symbols.len() >= 5);
                let __sym4 = __pop_Variant1(__symbols);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = match super::__action4::<>(vm, input, __sym0, __sym1, __sym2, __sym3, __sym4) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant2(__nt), __end));
                (5, 2)
            }
            6 => {
                __reduce6(vm, input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                // __Print = Print => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(vm, input, __sym0);
                return Some(Ok(__nt));
            }
            8 => {
                // raw_addr = r#"[0-9]+"# => ActionFn(10);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action10::<>(vm, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
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
        vm: &VM,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @L =  => ActionFn(8);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action8::<>(vm, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        vm: &VM,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // @R =  => ActionFn(7);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action7::<>(vm, input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        vm: &VM,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Print = "print", "flags" => ActionFn(1);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action1::<>(vm, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        vm: &VM,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Print = "print", "reg" => ActionFn(2);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action2::<>(vm, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        vm: &VM,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Print = "print", "mem", raw_addr, "->", raw_addr => ActionFn(3);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action3::<>(vm, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (5, 2)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        vm: &VM,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Print = "print", "mem", ":", raw_addr => ActionFn(5);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action5::<>(vm, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (4, 2)
    }
}
pub use self::__parse__Print::PrintParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use emulator_8086_lib as lib;
    use lib::vm::{VM,MB};
    use lib::{get_flag_state, Flags};
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^([0-9]+)", false),
            ("^(\\->)", false),
            ("^(:)", false),
            ("^(flags)", false),
            ("^(mem)", false),
            ("^(print)", false),
            ("^(reg)", false),
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
    vm: &VM,
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
    vm: &VM,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> ()
{
    {
        let of = get_flag_state(vm.arch.flag, Flags::OVERFLOW) as u8;
        let df = get_flag_state(vm.arch.flag, Flags::DIRECTION) as u8;
        let iflag = get_flag_state(vm.arch.flag, Flags::INTERRUPT) as u8;
        let tf = get_flag_state(vm.arch.flag, Flags::TRAP) as u8;
        let sf = get_flag_state(vm.arch.flag, Flags::SIGN) as u8;
        let zf = get_flag_state(vm.arch.flag, Flags::ZERO) as u8;
        let af = get_flag_state(vm.arch.flag, Flags::AUX_CARRY) as u8;
        let pf = get_flag_state(vm.arch.flag, Flags::PARITY) as u8;
        let cf = get_flag_state(vm.arch.flag, Flags::CARRY) as u8;

        println!(
            "OF : {}\tDF : {}\tIF : {}\tTF : {}\tSF : {}\t ZF : {}\tAF : {}\tPF : {}\tCF : {}",
            of, df, iflag, tf, sf, zf, af, pf, cf
        );
    }
}

#[allow(unused_variables)]
fn __action2<
    'input,
    's,
>(
    vm: &VM,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> ()
{
    {
        println!("AX : 0x{:04X}\t\tSP : 0x{:04X}",vm.arch.ax,vm.arch.sp);
        println!("BX : 0x{:04X}\t\tBP : 0x{:04X}",vm.arch.bx,vm.arch.bp);
        println!("CX : 0x{:04X}\t\tSI : 0x{:04X}",vm.arch.cx,vm.arch.si);
        println!("DX : 0x{:04X}\t\tDI : 0x{:04X}",vm.arch.dx,vm.arch.di);
        println!();
        println!("CS : 0x{:04X}\t\tSS : 0x{:04X}",vm.arch.cs,vm.arch.ss);
        println!("DS : 0x{:04X}\t\tES : 0x{:04X}",vm.arch.ds,vm.arch.es);
    }
}

#[allow(unused_variables)]
fn __action3<
    'input,
    's,
>(
    vm: &VM,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, start, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> ()
{
    {
        if start > end{
            println!("Starting address is less than end address : {} > {}",start,end);
            return;
        }
        let mut ctr = 0;
        for i in start..=end{
            print!("{:02X}\t",vm.mem[i]);
            if (ctr+1)%8 ==0{
                print!("\t");
            }
            if (ctr+1)%16 ==0{
                println!();
            }
            ctr = (ctr+1)%16;
        }
        if ctr != 0 {
            println!();
        }
    }
}

#[allow(unused_variables)]
fn __action4<
    'input,
    's,
>(
    vm: &VM,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, start, _): (usize, usize, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, offset, _): (usize, usize, usize),
) -> Result<(),__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        let end = start + offset;
        if end >= MB as usize{
            return Err(ParseError::UnrecognizedToken {
                token: (start, Token(0, ""), end),expected: vec!["Internal Error : start + offset > 1048576".to_owned()]
                })
        }
        let mut ctr = 0;
        for i in start..=end{
            print!("{:02X}\t",vm.mem[i]);
            if (ctr+1)%8 ==0{
                print!("\t");
            }
            if (ctr+1)%16 ==0{
                println!();
            }
            ctr = (ctr+1)%16;
        }
        if ctr != 0 {
            println!();
        }
        return Ok(());
    }
}

#[allow(unused_variables)]
fn __action5<
    'input,
    's,
>(
    vm: &VM,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, offset, _): (usize, usize, usize),
) -> ()
{
    {
        let start = vm.arch.ds as usize * 0x10;
        let end = start + offset;
        if end >= MB as usize{
            println!("Error : End address overflowing memory space : DS * 0x10 =  {}, end address = {}",start,end);
            return;
        }
        let mut ctr = 0;
        for i in start..=end{
            print!("{:02X}\t",vm.mem[i]);
            if (ctr+1)%8 ==0{
                print!("\t");
            }
            if (ctr+1)%16 ==0{
                println!();
            }
            ctr = (ctr+1)%16;
        }
        if ctr != 0 {
            println!();
        }
        
    }
}

#[allow(unused_variables)]
fn __action6<
    'input,
    's,
>(
    vm: &VM,
    input: &'input str,
    (_, start, _): (usize, usize, usize),
    (_, n, _): (usize, &'input str, usize),
    (_, end, _): (usize, usize, usize),
) -> Result<usize,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        match usize::from_str_radix(n,10){
            Ok(v) => Ok(v%MB as usize),
            Err(_) => Err(ParseError::UnrecognizedToken {
                token: (start, Token(0, ""), end),expected: vec!["Internal Error : Invalid Value, must be between 0-1048576".to_owned()]
                }),
        }
    }
}

#[allow(unused_variables)]
fn __action7<
    'input,
    's,
>(
    vm: &VM,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action8<
    'input,
    's,
>(
    vm: &VM,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action9<
    'input,
    's,
>(
    vm: &VM,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> Result<usize,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action8(
        vm,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        vm,
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
    vm: &VM,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> Result<usize,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action7(
        vm,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        vm,
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
