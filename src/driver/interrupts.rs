use emulator_8086_lib as lib;
use lib::util::data_util::{get_byte_reg, set_byte_reg, ByteReg};
use lib::VM;

/// Function to implement BIOS interrupt 0x13
pub fn int_13(vm: &VM, ah: u8) {
    // print a single char cx times
    if ah == 0xA {
        let al = get_byte_reg(vm, ByteReg::AL);
        for _ in 0..vm.arch.cx {
            print!("{}", al as char);
        }
        return;
    }
    if ah == 0x13 {
        // print a string
        let l = vm.arch.cx as usize; // length
        let dl = get_byte_reg(vm, ByteReg::DL); // start col
        let start = vm.arch.es as usize * 0x10 + vm.arch.bp as usize;
        // pad till start col
        for _ in 0..dl {
            print!(" ");
        }
        // print actual string
        for i in 0..l {
            print!("{}", vm.mem[start + i] as char);
        }
    }
}

/// Function to implement DOS interrupt 0x21
pub fn int_21(vm: &mut VM, ah: u8) {
    // read single char
    if ah == 0x1 {
        let mut input = String::new();
        // read the whole line
        match std::io::stdin().read_line(&mut input) {
            Ok(s) => s,
            Err(e) => {
                println!("Error in reading stdin : {}", e);
                return;
            }
        };
        // take the first byte, if nothing read, default to 0
        let byte = match input.bytes().nth(0) {
            Some(a) => a,
            None => 0,
        };
        set_byte_reg(vm, ByteReg::AL, byte);
    }
    // to print a single char
    if ah == 0x2 {
        let dl = get_byte_reg(vm, ByteReg::DL);
        print!("{}", dl as char);
        set_byte_reg(vm, ByteReg::AL, dl);
    }

    // to read string of given length
    if ah == 0xA {
        let mut input = String::new();
        // read line
        match std::io::stdin().read_line(&mut input) {
            Ok(s) => s,
            Err(e) => {
                println!("Error in reading stdin : {}", e);
                return;
            }
        };
        // storage address
        let start = vm.arch.ds as usize * 0x10 + vm.arch.dx as usize;
        // how many are actually supposed to read
        let required = vm.mem[start] as usize;
        let max: u8;
        if input.len() < required {
            max = input.len() as u8;
            // store how many are read, skipping newline char
            vm.mem[start + 1] = max - 1;
        } else {
            max = required as u8;
            vm.mem[start + 1] = max;
        }
        // store the characters
        let mut ctr = 0;
        for i in input.bytes() {
            vm.mem[start + 2 + ctr] = i;
            ctr += 1;
        }
    }
}
