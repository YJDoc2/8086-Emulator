use crate::arch::FLAG_CARRY;
use crate::util::flag_util::*;
use crate::util::interpreter_util::*;
use crate::vm::VM;

#[inline]
fn set_flag_helper(flag: &mut u16, sign: bool, zero: bool, parity: bool) {
    if sign {
        set_flag(flag, Flags::SIGN);
    } else {
        unset_flag(flag, Flags::SIGN);
    }
    if zero {
        set_flag(flag, Flags::ZERO);
    } else {
        unset_flag(flag, Flags::ZERO);
    }
    if parity {
        set_flag(flag, Flags::PARITY);
    } else {
        unset_flag(flag, Flags::PARITY);
    }
}

pub fn byte_and(vm: &mut VM, dest: u8, source: u8) -> u8 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest & source;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
    return res;
}

pub fn byte_or(vm: &mut VM, dest: u8, source: u8) -> u8 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest | source;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );

    return res;
}

pub fn byte_xor(vm: &mut VM, dest: u8, source: u8) -> u8 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest ^ source;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
    return res;
}

pub fn byte_test(vm: &mut VM, dest: u8, source: u8) -> u8 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest & source;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
    return dest;
}

pub fn word_and(vm: &mut VM, dest: u16, source: u16) -> u16 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest & source;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 15,
        res == 0,
        has_even_parity(res as u8),
    );
    return res;
}

pub fn word_or(vm: &mut VM, dest: u16, source: u16) -> u16 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest | source;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 15,
        res == 0,
        has_even_parity(res as u8),
    );
    return res;
}

pub fn word_xor(vm: &mut VM, dest: u16, source: u16) -> u16 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest ^ source;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 15,
        res == 0,
        has_even_parity(res as u8),
    );
    return res;
}

pub fn word_test(vm: &mut VM, dest: u16, source: u16) -> u16 {
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    unset_flag(&mut vm.arch.flag, Flags::CARRY);
    let res = dest & source;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 15,
        res == 0,
        has_even_parity(res as u8),
    );
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
        if t & 1 << 8 != 0 {
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
        res = t as u8;
        // if sign bit is same as next bit
        // as this is supposed to be correct only in 1 bit shifts,
        // we can directly compare msb of val and res
        if val & 1 << 7 == res & 1 << 7 {
            unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        } else {
            set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
        }
    }
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
    return res;
}

pub fn byte_sar(vm: &mut VM, val: u8, num: u8) -> u8 {
    let res: u8;

    let msb = val & 1 << 7;
    if num > 9 {
        // kind of optimization, as shifting byte number more than 8 times, it will become zero or max
        if msb != 0 {
            res = u8::MAX;
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            res = 0;
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
    } else {
        // instead of loop if msb is 1, shift the u8 max and OR it with number
        res = val >> num | if msb == 0 { 0 } else { u8::MAX << 8 - num };
        if res & 1 == 1 {
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
    }
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW); // always clear overflow
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
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
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
    return res;
}

pub fn byte_rol(vm: &mut VM, val: u8, num: u8) -> u8 {
    let num = num % 8;
    let res = val << num | val >> (8 - num);
    if res & 1 == 1 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    if val & 1 << 7 == res & 1 << 7 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    return res;
}

pub fn byte_ror(vm: &mut VM, val: u8, num: u8) -> u8 {
    let num = num % 8;
    let res = val >> num | val << (8 - num);
    if res & 1 << 7 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    if val & 1 << 7 == res & 1 << 7 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    return res;
}

pub fn byte_rcl(vm: &mut VM, val: u8, num: u8) -> u8 {
    let num = num % 9;
    let v = val as u16 | (vm.arch.flag & FLAG_CARRY) << 8; // add carry as the 9th bit
    let mask = u16::MAX << num;
    let rotated_part = v & mask;
    let res = v << num | rotated_part >> 9 - num;
    if res & 1 << 8 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let res = res as u8;
    if val & 1 << 7 == res & 1 << 7 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    return res;
}

pub fn byte_rcr(vm: &mut VM, val: u8, num: u8) -> u8 {
    let num = num % 9;
    let v = val as u16 | (vm.arch.flag & FLAG_CARRY) << 8; // add carry as the 9th bit
    let mask = u16::MAX >> num;
    let rotated_part = v & mask;
    let res = v >> num | rotated_part << 9 - num;
    if res & 1 << 8 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let res = res as u8;
    if val & 1 << 7 == res & 1 << 7 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    return res;
}

pub fn word_sal(vm: &mut VM, val: u16, num: u16) -> u16 {
    let res: u16;
    if num > 17 {
        // kind of optimization, as shifting byte number more than 8 times, it will become zero
        res = 0;
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        let t = (val as u32) << num;
        if t & 1 << 16 != 0 {
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
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 15,
        res == 0,
        has_even_parity(res as u8),
    );
    return res;
}

pub fn word_sar(vm: &mut VM, val: u16, num: u16) -> u16 {
    let res: u16;

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
        res = val >> num | if msb == 0 { 0 } else { u16::MAX << 16 - num };
        if res & 1 == 1 {
            set_flag(&mut vm.arch.flag, Flags::CARRY);
        } else {
            unset_flag(&mut vm.arch.flag, Flags::CARRY);
        }
    }
    unset_flag(&mut vm.arch.flag, Flags::OVERFLOW); // always clear overflow
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 15,
        res == 0,
        has_even_parity(res as u8),
    );
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
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 15,
        res == 0,
        has_even_parity(res as u8),
    );
    return res;
}

pub fn word_rol(vm: &mut VM, val: u16, num: u16) -> u16 {
    let num = num % 16;
    let res = val << num | val >> (16 - num);
    if res & 1 == 1 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    if val & 1 << 15 == res & 1 << 15 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    return res;
}

pub fn word_ror(vm: &mut VM, val: u16, num: u16) -> u16 {
    let num = num % 16;
    let res = val >> num | val << (16 - num);
    if res & 1 << 15 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    if val & 1 << 15 == res & 1 << 15 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    return res;
}

pub fn word_rcl(vm: &mut VM, val: u16, num: u16) -> u16 {
    let num = num % 17;
    let v = val as u32 | ((vm.arch.flag & FLAG_CARRY) as u32) << 16;
    let mask = u32::MAX << num;
    let rotated_part = v & mask;
    let res = v << num | rotated_part >> 17 - num;
    if res & 1 << 16 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let res = res as u16;
    if val & 1 << 15 == res & 1 << 15 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    return res;
}

pub fn word_rcr(vm: &mut VM, val: u16, num: u16) -> u16 {
    let num = num % 17;
    let v = val as u32 | ((vm.arch.flag & FLAG_CARRY) as u32) << 16;
    let mask = u32::MAX >> num;
    let rotated_part = v & mask;
    let res = v >> num | rotated_part << 17 - num;
    if res & 1 << 16 != 0 {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let res = res as u16;
    if val & 1 << 15 == res & 1 << 15 {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    return res;
}
