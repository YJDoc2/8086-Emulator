use crate::util::flag_util::*;
use crate::util::interpreter_util::*;
use crate::vm::VM;

pub type ByteOp = fn(&mut VM, u8, u8) -> u8;
pub type WordOp = fn(&mut VM, u16, u16) -> u16;

pub fn byte_placeholder(_: &mut VM, _: u8, _: u8) -> u8 {
    0
}

pub fn word_placeholder(_: &mut VM, _: u16, _: u16) -> u16 {
    0
}

pub fn byte_and(vm: &mut VM, dest: u8, source: u8) -> u8 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest & source;
    if res >= 1 << 7 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res as u16) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}

pub fn byte_or(vm: &mut VM, dest: u8, source: u8) -> u8 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);

    let res = dest | source;

    if res >= 1 << 7 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res as u16) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}

pub fn byte_xor(vm: &mut VM, dest: u8, source: u8) -> u8 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest ^ source;

    if res >= 1 << 7 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res as u16) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}

pub fn byte_test(vm: &mut VM, dest: u8, source: u8) -> u8 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest & source;
    if res >= 1 << 7 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res as u16) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return dest;
}

pub fn word_and(vm: &mut VM, dest: u16, source: u16) -> u16 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest & source;
    if res >= 1 << 15 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}

pub fn word_or(vm: &mut VM, dest: u16, source: u16) -> u16 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest | source;
    if res >= 1 << 15 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}

pub fn word_xor(vm: &mut VM, dest: u16, source: u16) -> u16 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest ^ source;

    if res >= 1 << 15 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}

pub fn word_test(vm: &mut VM, dest: u16, source: u16) -> u16 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest & source;
    if res >= 1 << 15 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return dest;
}

pub fn byte_sal(vm: &mut VM, val: u8, num: u8) -> u8 {
    let res: u8;
    if num > 9 {
        // kind of optimization, as shifting byte number more than 8 times, it will become zero
        res = 0;
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        let t = (val as u16) << num;
        if t & 1 << 8 == 1 << 8 {
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
        res = t as u8;
        if val & 1 << 7 == res & 1 << 7 {
            // if sign bit is same as next bit
            unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        } else {
            set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        }
    }

    if res >= 1 << 7 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res as u16) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}

pub fn byte_sar(vm: &mut VM, val: u8, num: u8) -> u8 {
    let mut res: u8;

    let msb = val & 1 << 7;
    if num > 9 {
        // kind of optimization, as shifting byte number more than 8 times, it will become zero
        if msb != 0 {
            res = u8::MAX;
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            res = 0;
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
    } else {
        res = val;
        for _ in 0..num {
            res = (res >> 1) | msb;
        }
        if res & 1 == 1 {
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
    }
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW); // always clear overflow
    if res >= 1 << 7 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res as u16) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}

pub fn byte_shr(vm: &mut VM, val: u8, num: u8) -> u8 {
    let res: u8;
    if num > 9 {
        // kind of optimization, as shifting byte number more than 8 times, it will become zero
        res = 0;
    } else {
        if val & 1 << 7 == 0 {
            // if sign bit retains its value
            unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        } else {
            set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        }
        let t = (val as u16) >> num;
        if t & 1 == 1 {
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
        res = t as u8;
    }
    if res >= 1 << 7 {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }
    if res == 0 {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    if has_even_parity(res as u16) {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }
    return res;
}
