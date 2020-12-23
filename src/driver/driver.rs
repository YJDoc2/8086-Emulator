use super::error_helper::get_err_pos;
use super::interrupts::{int_13, int_21};
use super::preprocess::preprocess;
use super::print::PrintParser;
use super::user_interface::user_interface;
use emulator_8086_lib as lib;
use lib::util::data_util::{get_byte_reg, ByteReg};
use lib::InterpreterContext;
use lib::LabelType;
use lib::PreprocessorContext;
use lib::VM;
use lib::{get_flag_state, Flags};
use lib::{DataParser, Interpreter, State};
use regex::Regex;

pub struct CMDDriver {
    input: String,
    interpreted: bool,
}

impl CMDDriver {
    pub fn new(ip: String, interpreted: bool) -> Self {
        return CMDDriver {
            input: ip,
            interpreted: interpreted,
        };
    }

    pub fn run(&self) {
        // remove comments
        let r = Regex::new(r";.*\n").unwrap();
        let uncommented = r.replace_all(&self.input, "\n").to_string();
        // run the preprocessor
        let (lh, pctx, mut out) = match preprocess(&uncommented) {
            Ok(a) => a,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        // destructure the context
        let PreprocessorContext {
            macro_nesting_counter: _,
            data_counter: _,
            label_map: lmap,
            macro_map: _,
            mapper,
            fn_map,
            undefined_labels,
        } = pctx;

        // check if all jump labels are defined are not
        for (pos, l) in undefined_labels.iter() {
            match lmap.get(l) {
                Some(_) => {}
                None => {
                    let (line, pos, start, end) = get_err_pos(&lh, *pos);
                    println!(
                        "Label {} used but not defined at {} :{} : {}",
                        l,
                        line,
                        start - pos,
                        &uncommented[start..end]
                    );
                }
            }
        }

        let mut idx; // for iterating through code

        // check if the start is defined or not
        match lmap.get("start") {
            Some(l) => match l.get_type() {
                LabelType::DATA => {
                    println!("Error : necessary label 'start' is not found in code");
                    return;
                }
                LabelType::CODE => idx = l.map,
            },
            None => {
                println!("Error : necessary label 'start' is not found in code");
                return;
            }
        }

        // now we have checked out and sorted out all issues with syntax and all
        // now we can set the data and run the code
        let source_map = mapper.get_source_map();
        let mut ictx = InterpreterContext {
            fn_map: fn_map,
            label_map: lmap,
            call_stack: Vec::new(),
        };

        let data_parser = DataParser::new();
        let mut vm = VM::new();
        let mut ctr = 0;
        for i in out.data.iter() {
            match data_parser.parse(&mut vm, &mut ctr, i) {
                Ok(_) => {}
                Err(e) => {
                    // should not reach here, as all error of syntax should have checked in preprocessor
                    println!(
                        "Internal Error : Should not have reached here in data parser\nError : {}",
                        e
                    );
                    return;
                }
            }
        }

        // contingency, so we do not go over the end
        out.code.push("hlt".to_owned());
        let interpreter = Interpreter::new();
        let printer = PrintParser::new();

        loop {
            let tf = get_flag_state(vm.arch.flag, Flags::TRAP);
            if self.interpreted || tf {
                let pos = source_map.get(&idx).unwrap();
                let (line, _, start, end) = get_err_pos(&lh, *pos);
                println!(
                    "About to execute line {} : {}",
                    line,
                    &uncommented[start..end]
                );
                if tf {
                    println!("Trap flag is set");
                }
                user_interface(&vm);
            }
            match interpreter.parse(idx, &mut vm, &mut ictx, &out.code[idx]) {
                Err(e) => {
                    println!(
                        "Internal Error : Should not have reached here in data parser\nError : {}",
                        e
                    );
                    return;
                }
                Ok(s) => match s {
                    State::HALT => {
                        return;
                    }
                    State::PRINT => {
                        let pos = source_map.get(&idx).unwrap();
                        let (line, _, start, end) = get_err_pos(&lh, *pos);
                        println!("Output of line {} : {} :", line, &uncommented[start..end]);
                        match printer.parse(&vm, &out.code[idx]) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Internal Error : Should not have reached here in print parser\nError : {}",e);
                                return;
                            }
                        }
                        // go to next command
                        idx += 1;
                    }
                    State::JMP(next) => {
                        // jump to next commnand
                        idx = next;
                    }
                    State::NEXT => {
                        // we can do this without check, as we have inserted a 'hlt' in the code at end
                        idx += 1;
                    }
                    State::INT(int) => {
                        match int {
                            0 => {
                                let pos = source_map.get(&idx).unwrap();
                                let (line, _, start, end) = get_err_pos(&lh, *pos);
                                println!(
                                    "Attempt to divide by 0 : int 0 at {} : {}",
                                    line,
                                    &uncommented[start..end]
                                );
                                println!("Exiting");
                                return;
                            }
                            0x3 => {
                                let pos = source_map.get(&idx).unwrap();
                                let (line, _, _, _) = get_err_pos(&lh, *pos);
                                println!("Int 3 at line {}", line);
                                user_interface(&vm);
                            }
                            0x10 => {
                                let ah = get_byte_reg(&vm, ByteReg::AH);
                                if ah != 0xA && ah != 0x13 {
                                    let pos = source_map.get(&idx).unwrap();
                                    let (line, _, start, end) = get_err_pos(&lh, *pos);
                                    println!(
                                        "Error at line {} : {}, value of AH = {} is not supported for int 0x10",
                                        line,
                                        &uncommented[start..end],
                                        ah
                                    );
                                    println!("Exiting");
                                    return;
                                }
                                int_13(&vm, ah);
                            }
                            0x21 => {
                                let ah = get_byte_reg(&vm, ByteReg::AH);
                                if ah != 0x1 && ah != 0x2 && ah != 0xA {
                                    let pos = source_map.get(&idx).unwrap();
                                    let (line, _, start, end) = get_err_pos(&lh, *pos);
                                    println!(
                                        "Error at line {} : {}, value of AH = {} is not supported for int 0x10",
                                        line,
                                        &uncommented[start..end],
                                        ah
                                    );
                                    println!("Exiting");
                                    return;
                                }
                                int_21(&mut vm, ah);
                            }
                            _ => {
                                println!("Internal Error : Should not have reached here in interrupt parser\nError : int {} not supported",int);
                                return;
                            }
                        }

                        // go to next command
                        idx += 1;
                    }
                    State::REPEAT => { /* Do nothing, as we have to repeat the same instruction*/ }
                },
            }
        }
    }
}
