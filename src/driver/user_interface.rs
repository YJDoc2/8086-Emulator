use super::print::PrintParser;
use emulator_8086_lib as lib;
use lib::VM;
use std::io::Write;

/// Function to give user prompt for interpreted mode, ot int 3
pub fn user_interface(vm: &VM, printer: &PrintParser) {
    loop {
        print!(">>> "); // prompt
                        // we use print! not println!, as we do not want user to give input on newline, so we flush instead
        let _ = std::io::stdout().flush(); // we can ignore failure of writing to stdout

        // take input
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(s) => s,
            Err(e) => {
                println!("Error in reading stdin : {}", e);
                return;
            }
        };

        // trim input to skip spaces and newlines
        let input = input.trim().to_ascii_lowercase();

        // just continue to next program line, so return
        if input == "n" || input == "next" {
            return;
        }

        // exit
        if input == "q" || input == "quit" {
            println!("Exiting");
            std::process::exit(0);
        }

        match printer.parse(vm, &input) {
            Ok(_) => {}
            Err(_) => println!("Invalid input , only next/n or print commands are accepted"),
        }
    }
}
