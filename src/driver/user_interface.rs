use super::print::PrintParser;
use emulator_8086_lib as lib;
use lib::VM;
use std::io::Write;
pub fn user_interface(vm: &VM) {
    loop {
        print!(">>> ");
        let _ = std::io::stdout().flush(); // we can ignore failure of writing to stdout
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(s) => s,
            Err(e) => {
                println!("Error in reading stdin : {}", e);
                return;
            }
        };
        let input = input.trim().to_ascii_lowercase();
        if input == "n" || input == "next" {
            return;
        }
        if input == "q" || input == "quit" {
            println!("Exiting");
            std::process::exit(0);
        }
        let printer = PrintParser::new();
        match printer.parse(vm, &input) {
            Ok(_) => {}
            Err(_) => println!("Invalid input , only next/n or print commands are accepted"),
        }
    }
}
