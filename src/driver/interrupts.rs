use emulator_8086_lib as lib;
use lib::util::data_util::{get_byte_reg, set_byte_reg, ByteReg};
use lib::VM;

pub fn int_13(vm: &VM, ah: u8) {
    if ah == 0xA {
        let al = get_byte_reg(vm, ByteReg::AL);
        for _ in 0..vm.arch.cx {
            print!("{}", al as char);
        }
        return;
    }
    if ah == 0x13 {
        let l = vm.arch.cx as usize;
        let dl = get_byte_reg(vm, ByteReg::DL);
        let start = vm.arch.es as usize * 0x10 + vm.arch.bp as usize;
        for _ in 0..dl {
            print!(" ");
        }
        for i in 0..l {
            print!("{}", vm.mem[start + i] as char);
        }
    }
}

pub fn int_21(vm: &mut VM, ah: u8) {
    if ah == 0x1 {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(s) => s,
            Err(e) => {
                println!("Error in reading stdin : {}", e);
                return;
            }
        };
        let byte = match input.bytes().nth(0) {
            Some(a) => a,
            None => 0,
        };
        set_byte_reg(vm, ByteReg::AL, byte);
    }
    if ah == 0x2 {
        let dl = get_byte_reg(vm, ByteReg::DL);
        print!("{}", dl as char);
        set_byte_reg(vm, ByteReg::AL, dl);
    }
    if ah == 0xA {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(s) => s,
            Err(e) => {
                println!("Error in reading stdin : {}", e);
                return;
            }
        };
        let start = vm.arch.ds as usize * 0x10 + vm.arch.dx as usize;
        let required = vm.mem[start] as usize;
        let max: u8;
        if input.len() < required {
            max = input.len() as u8;
            vm.mem[start + 1] = max - 1;
        } else {
            max = required as u8;
            vm.mem[start + 1] = max;
        }
        let mut ctr = 0;
        for i in input.bytes() {
            vm.mem[start + 2 + ctr] = i;
            ctr += 1;
        }
    }
}
