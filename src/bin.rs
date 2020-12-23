mod driver;
use clap::{App, Arg};
use driver::CMDDriver;
use std::fs;
use std::path::Path;

fn main() {
    // get command line args using clap
    let matches = App::new("8086 Emulator")
        .version("0.1.0")
        .author("Yashodhan Joshi")
        .about("A commandline 8086 emulator / VM / interpreter")
        .arg(
            Arg::with_name("file_path")
                .takes_value(true)
                .help("Input assembly file path"),
        )
        .arg(
            Arg::with_name("interpreted")
                .short("i")
                .long("interpreted")
                .takes_value(false)
                .help("To run in interpreted mode"),
        )
        .get_matches();

    // check if input file argument exists
    let filepath = match matches.value_of("file_path") {
        None => {
            println!("Input File is necessary argument...\nExiting.");
            std::process::exit(1);
        }
        Some(a) => a,
    };
    // if to run in interpreter mode or not
    let interpreted = matches.is_present("interpreted");

    // check if the input file exists
    if !Path::new(filepath).exists() {
        println!("Given Input File does not exist...\nExiting");
        std::process::exit(1);
    }

    // read file
    let input = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            println!("Error Reading file : {}\nExiting", e);
            std::process::exit(1);
        }
    };

    // create driver
    let driver = CMDDriver::new(input, interpreted);
    // run program
    driver.run();
    // put a blank line
    println!();
}
