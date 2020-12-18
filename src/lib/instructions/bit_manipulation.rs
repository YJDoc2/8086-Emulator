use crate::arch::FLAG_CARRY;
use crate::util::flag_util::*;
use crate::util::interpreter_util::*;
use crate::vm::VM;

pub type ByteOp = fn(&mut VM, u8, u8) -> u8;
pub type WordOp = fn(&mut VM, u16, u16) -> u16;

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

pub fn byte_rol(vm: &mut VM, val: u8, num: u8) -> u8 {
    let num = num % 8;
    let part = u8::MAX << (8 - num);
    let sep = val & part;
    let ret = val << num | sep >> (8 - num);
    if ret & 1 == 1 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    if val & 1 << 7 == ret & 1 << 7 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    ret
}

pub fn byte_ror(vm: &mut VM, val: u8, num: u8) -> u8 {
    let num = num % 8;
    let part = u8::MAX >> (8 - num);
    let sep = val & part;
    let ret = val >> num | sep << (8 - num);
    if ret & 1 << 7 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    if val & 1 << 7 == ret & 1 << 7 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    ret
}

pub fn byte_rcl(vm: &mut VM, val: u8, num: u8) -> u8 {
    let num = num % 9;
    let v = val as u16 | (vm.arch.flag & FLAG_CARRY) << 8;
    let part = u16::MAX << num;
    let sep = v & part;
    let ret = v << num | sep >> 9 - num;
    if ret & 1 << 8 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let ret = ret as u8;
    if val & 1 << 7 == ret & 1 << 7 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    ret
}

pub fn byte_rcr(vm: &mut VM, val: u8, num: u8) -> u8 {
    let num = num % 9;
    let v = val as u16 | (vm.arch.flag & FLAG_CARRY) << 8;
    let part = u16::MAX >> num;
    let sep = v & part;
    let ret = v >> num | sep << 9 - num;
    if ret & 1 << 8 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let ret = ret as u8;
    if val & 1 << 7 == ret & 1 << 7 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    ret
}

// ---------------------------------WORD-------------

pub fn word_sal(vm: &mut VM, val: u16, num: u16) -> u16 {
    let res: u16;
    if num > 17 {
        // kind of optimization, as shifting byte number more than 8 times, it will become zero
        res = 0;
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        let t = (val as u32) << num;
        if t & 1 << 16 == 1 << 16 {
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
        res = t as u16;
        if val & 1 << 15 == res & 1 << 15 {
            // if sign bit is same as next bit
            unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        } else {
            set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        }
    }

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

pub fn word_sar(vm: &mut VM, val: u16, num: u16) -> u16 {
    let mut res: u16;

    let msb = val & 1 << 15;
    if num > 17 {
        // kind of optimization, as shifting byte number more than 8 times, it will become zero
        if msb != 0 {
            res = u16::MAX;
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

pub fn word_shr(vm: &mut VM, val: u16, num: u16) -> u16 {
    let res: u16;
    if num > 17 {
        // kind of optimization, as shifting byte number more than 8 times, it will become zero
        res = 0;
    } else {
        if val & 1 << 15 == 0 {
            // if sign bit retains its value
            unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        } else {
            set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        }
        let t = (val as u32) >> num;
        if t & 1 == 1 {
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
        res = t as u16;
    }
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

pub fn word_rol(vm: &mut VM, val: u16, num: u16) -> u16 {
    let num = num % 16;
    let part = u16::MAX << (16 - num);
    let sep = val & part;
    let ret = val << num | sep >> (16 - num);
    if ret & 1 == 1 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    if val & 1 << 15 == ret & 1 << 15 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    ret
}

pub fn word_ror(vm: &mut VM, val: u16, num: u16) -> u16 {
    let num = num % 16;
    let part = u16::MAX >> (16 - num);
    let sep = val & part;
    let ret = val >> num | sep << (16 - num);
    if ret & 1 << 15 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    if val & 1 << 15 == ret & 1 << 15 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    ret
}

pub fn word_rcl(vm: &mut VM, val: u16, num: u16) -> u16 {
    let num = num % 17;
    let v = val as u32 | ((vm.arch.flag & FLAG_CARRY) as u32) << 16;
    let part = u32::MAX << num;
    let sep = v & part;
    let ret = v << num | sep >> 17 - num;
    if ret & 1 << 16 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let ret = ret as u16;
    if val & 1 << 15 == ret & 1 << 15 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    ret
}

pub fn word_rcr(vm: &mut VM, val: u16, num: u16) -> u16 {
    let num = num % 17;
    let v = val as u32 | ((vm.arch.flag & FLAG_CARRY) as u32) << 16;
    let part = u32::MAX >> num;
    let sep = v & part;
    let ret = v >> num | sep << 17 - num;
    if ret & 1 << 16 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let ret = ret as u16;
    if val & 1 << 15 == ret & 1 << 15 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    ret
}
