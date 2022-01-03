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

/// Structure used to prepare and run the program
/// The run method on this will run the preparser, initialize data and then run the code
pub struct CMDDriver {
    input: String,
    interpreted: bool,
}

impl CMDDriver {
    /// create new CMDDriver
    /// ip is the program string
    /// interpreted is flag check to display user prompt after every instruction
    pub fn new(ip: String, interpreted: bool) -> Self {
        CMDDriver {
            input: ip,
            interpreted,
        }
    }

    /// This method will compile the program then create vm, initialize it and run the interpreter
    pub fn run(&self) {
        // remove comments
        let r = Regex::new(r";.*\n?").unwrap();
        let uncommented = r.replace_all(&self.input, "\n").to_string();
        // run the preprocessor
        let (lh, pctx, mut out) = match preprocess(&uncommented) {
            Ok(a) => a,
            Err(e) => {
                // if any error , print and return
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
                    let (line, start, end) = get_err_pos(&lh, *pos);
                    println!(
                        "Label {} used but not defined at {} :{} : {}",
                        l,
                        line,
                        end - pos,
                        &uncommented[start..end]
                    );
                    return;
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
            fn_map,
            label_map: lmap,
            call_stack: Vec::new(),
        };

        // create data parser and vm
        // vm is left to initialize as long as possible , as it allocates 1 MB on heap
        let data_parser = DataParser::new();
        let mut vm = VM::new();

        // this is for the data counter required by data parser
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
        vm.arch.ds = 0;
        // contingency, so we do not go over the end
        // and as the user will probably not add hlt at end, this is a good thing
        // even the examples do not contain hlt at end
        out.code.push("hlt".to_owned());

        // create interpreter and print commands parser
        let interpreter = Interpreter::new();
        let printer = PrintParser::new();

        loop {
            let tf = get_flag_state(vm.arch.flag, Flags::TRAP);

            // if trap flag is set, or interpreted is enabled, display user prompt
            // the second condition is to check that the instruction is not the hlt that we have inserted
            // as it does not have any mapping
            if (self.interpreted || tf) && idx <= out.code.len() - 2 {
                // idx is 0 based, but line numbers are 1 based

                let pos = source_map.get(&idx).unwrap();
                let (line, start, end) = get_err_pos(&lh, *pos + 5);
                // show which line is to be executed
                println!(
                    "About to execute line {} : {}",
                    line,
                    &uncommented[start..end]
                );
                if tf {
                    println!("Trap flag is set");
                }
                // go to user interface
                user_interface(&vm, &printer);
            }
            match interpreter.parse(idx, &mut vm, &mut ictx, &out.code[idx]) {
                Err(e) => {
                    // should not reach here, as all error of syntax should have checked in preprocessor
                    println!(
                        "Internal Error : Should not have reached here in interpreter parser\nError : {}",
                        e
                    );
                    return;
                }
                Ok(s) => match s {
                    State::HALT => {
                        // stop and return
                        return;
                    }
                    State::PRINT => {
                        let pos = source_map.get(&idx).unwrap();
                        let (line, start, end) = get_err_pos(&lh, *pos);
                        // show which print line is running
                        println!("Output of line {} : {} :", line, &uncommented[start..end]);
                        match printer.parse(&vm, &out.code[idx]) {
                            Ok(_) => {}
                            Err(e) => {
                                // should not reach here, as all error of syntax should have checked in preprocessor
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
                                // divide by 0 error
                                let pos = source_map.get(&idx).unwrap();
                                let (line, start, end) = get_err_pos(&lh, *pos);
                                println!(
                                    "Attempt to divide by 0 : int 0 at {} : {}",
                                    line,
                                    &uncommented[start..end]
                                );
                                println!("Exiting");
                                return;
                            }
                            0x3 => {
                                // debugging interrupt
                                let pos = source_map.get(&idx).unwrap();
                                let (line, _, _) = get_err_pos(&lh, *pos);
                                println!("Int 3 at line {}", line);
                                user_interface(&vm, &printer);
                            }
                            0x10 => {
                                // BIOS interrupt
                                let ah = get_byte_reg(&vm, ByteReg::AH);
                                if ah != 0xA && ah != 0x13 {
                                    let pos = source_map.get(&idx).unwrap();
                                    let (line, start, end) = get_err_pos(&lh, *pos);
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
                                // DOS interrupts
                                let ah = get_byte_reg(&vm, ByteReg::AH);
                                if ah != 0x1 && ah != 0x2 && ah != 0xA {
                                    let pos = source_map.get(&idx).unwrap();
                                    let (line, start, end) = get_err_pos(&lh, *pos);
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
