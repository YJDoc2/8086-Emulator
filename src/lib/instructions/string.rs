use crate::util::address::*;
use crate::util::data_util::*;
use crate::util::flag_util::*;
use crate::util::interpreter_util::has_even_parity;
use crate::vm::VM;

struct FlagsToSet {
    zero: bool,
    parity: bool,
    sign: bool,
    overflow: bool,
    auxillary: bool,
    carry: bool,
}

fn set_flags(vm: &mut VM, flags: FlagsToSet) {
    // zero flag
    if flags.zero {
        set_flag(&mut vm.arch.flag, Flags::ZERO);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::ZERO);
    }
    // parity flag
    if flags.parity {
        set_flag(&mut vm.arch.flag, Flags::PARITY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::PARITY);
    }

    // sign flag
    if flags.sign {
        set_flag(&mut vm.arch.flag, Flags::SIGN);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::SIGN);
    }

    // overflow flag
    if flags.overflow {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    // carry/borrow flag
    if flags.carry {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    // aux carry / borrow
    if flags.auxillary {
        set_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    }
}

pub fn movs_byte(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    vm.mem[dest_addr] = vm.mem[src_addr];

    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }
}

pub fn movs_word(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    // byte 1
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    vm.mem[dest_addr] = vm.mem[src_addr];
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }
    // byte 2
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    vm.mem[dest_addr] = vm.mem[src_addr];
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }
}

pub fn loads_byte(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    set_byte_reg(vm, ByteReg::AL, vm.mem[src_addr]);
    if df {
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }
}

pub fn loads_word(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    // byte 1
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    set_byte_reg(vm, ByteReg::AL, vm.mem[src_addr]);
    if df {
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }
    // byte 2
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    set_byte_reg(vm, ByteReg::AH, vm.mem[src_addr]);
    if df {
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }
}

pub fn stos_byte(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let val = get_byte_reg(vm, ByteReg::AL);
    vm.mem[dest_addr] = val;
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
    }
}

pub fn stos_word(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    // byte 1
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let val = get_byte_reg(vm, ByteReg::AL);
    vm.mem[dest_addr] = val;
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
    }
    // byte 2
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let val = get_byte_reg(vm, ByteReg::AH);
    vm.mem[dest_addr] = val;
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
    }
}

pub fn cmps_byte(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    // byte 1
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let src = vm.mem[src_addr];
    let dest = vm.mem[dest_addr];
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }

    let diff = (src as i16 - dest as i16) as u8;

    set_flags(
        vm,
        FlagsToSet {
            zero: diff == 0,
            parity: has_even_parity(diff),
            sign: diff & 1 << 7 != 0,
            overflow: (src as i16 - dest as i16) < i8::MIN as i16,
            carry: src < dest,
            auxillary: src & 0b1111 < dest & 0b1111,
        },
    );
}

pub fn cmps_word(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    // byte 1
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let src = vm.mem[src_addr];
    let dest = vm.mem[dest_addr];
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }

    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.si);
    let dest_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let src = src as u16 | (vm.mem[src_addr] as u16) << 8;
    let dest = dest as u16 | (vm.mem[dest_addr] as u16) << 8;
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
        vm.arch.si = (vm.arch.si as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
        vm.arch.si = (vm.arch.si as u32 + 1) as u16;
    }

    let diff = (src as i32 - dest as i32) as u16;

    set_flags(
        vm,
        FlagsToSet {
            zero: diff == 0,
            parity: has_even_parity(diff as u8),
            sign: diff & 1 << 15 != 0,
            overflow: (src as i32 - dest as i32) < i16::MIN as i32,
            carry: src < dest,
            auxillary: src & 0b1111 < dest & 0b1111,
        },
    );
}

pub fn scas_byte(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    // byte 1
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let src = vm.mem[src_addr];
    let dest = get_byte_reg(vm, ByteReg::AL);
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
    }

    let diff = (src as i16 - dest as i16) as u8;

    set_flags(
        vm,
        FlagsToSet {
            zero: diff == 0,
            parity: has_even_parity(diff),
            sign: diff & 1 << 7 != 0,
            overflow: (src as i16 - dest as i16) < i8::MIN as i16,
            carry: src < dest,
            auxillary: src & 0b1111 < dest & 0b1111,
        },
    );
}

pub fn scas_word(vm: &mut VM) {
    let df = get_flag_state(vm.arch.flag, Flags::DIRECTION);
    // byte 1
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let src = vm.mem[src_addr];
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
    }
    let src_addr = Address::calculate_from_offset(vm.arch.ds, vm.arch.di);
    let src = src as u16 | (vm.mem[src_addr] as u16) << 8;
    if df {
        vm.arch.di = (vm.arch.di as i32 - 1) as u16;
    } else {
        vm.arch.di = (vm.arch.di as u32 + 1) as u16;
    }
    let dest = vm.arch.ax;
    let diff = (src as i32 - dest as i32) as u16;

    set_flags(
        vm,
        FlagsToSet {
            zero: diff == 0,
            parity: has_even_parity(diff as u8),
            sign: diff & 1 << 15 != 0,
            overflow: (src as i32 - dest as i32) < i16::MIN as i32,
            carry: src < dest,
            auxillary: src & 0b1111 < dest & 0b1111,
        },
    );
}
