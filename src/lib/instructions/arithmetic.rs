use crate::util::data_util::*;
use crate::util::flag_util::*;
use crate::util::interpreter_util::{has_even_parity, DivByZero};
use crate::vm::VM;

struct FlagsToSet {
    zero: bool,
    parity: bool,
    sign: bool,
    overflow: bool,
    auxillary: bool,
    carry: bool,
}

fn set_all_flags(vm: &mut VM, flags: FlagsToSet) {
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

pub fn aaa(vm: &mut VM) {
    let al = get_byte_reg(vm, ByteReg::AL);
    if al & 0x0F > 9 || get_flag_state(vm.arch.flag, Flags::AUX_CARRY) {
        set_byte_reg(vm, ByteReg::AL, ((al as u16 + 6) & 0x0F) as u8);
        let ah = get_byte_reg(vm, ByteReg::AH);
        set_byte_reg(vm, ByteReg::AH, (ah as u16 + 1) as u8);
        set_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
}

pub fn aad(vm: &mut VM) {
    let al = get_byte_reg(vm, ByteReg::AL);
    let ah = get_byte_reg(vm, ByteReg::AH);
    let res = ((ah as u16 * 10) + al as u16) as u8;
    set_byte_reg(vm, ByteReg::AL, res);
    set_byte_reg(vm, ByteReg::AH, 0);
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
}

pub fn aam(vm: &mut VM) {
    let al = get_byte_reg(vm, ByteReg::AL);
    set_byte_reg(vm, ByteReg::AH, al / 10);
    set_byte_reg(vm, ByteReg::AL, al % 10);
    let res = vm.arch.ax;
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 15,
        res == 0,
        has_even_parity(al % 10), // only lower 8 bytes are considered, i.e. al, which now have al %10
    );
}

pub fn aas(vm: &mut VM) {
    let al = get_byte_reg(vm, ByteReg::AL);
    if al & 0x0F > 9 || get_flag_state(vm.arch.flag, Flags::AUX_CARRY) {
        let ah = get_byte_reg(vm, ByteReg::AH);
        set_byte_reg(vm, ByteReg::AL, (al as i16 - 6) as u8 & 0x0F);
        set_byte_reg(vm, ByteReg::AH, (ah as i16 - 1) as u8);
        set_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
}

pub fn daa(vm: &mut VM) {
    let al = get_byte_reg(vm, ByteReg::AL);
    let mut overflow: bool = false;
    if al & 0x0F > 9 || get_flag_state(vm.arch.flag, Flags::AUX_CARRY) {
        set_byte_reg(vm, ByteReg::AL, (al as u16 + 6) as u8);
        set_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    }
    let al = get_byte_reg(vm, ByteReg::AL);
    if al > 0x9F || get_flag_state(vm.arch.flag, Flags::CARRY) {
        let temp = al as u16 + 0x60;
        overflow = temp > 255;
        set_byte_reg(vm, ByteReg::AL, temp as u8);
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let res = get_byte_reg(vm, ByteReg::AL);
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
    if overflow {
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
}

pub fn das(vm: &mut VM) {
    let al = get_byte_reg(vm, ByteReg::AL);
    if al & 0x0F > 9 || get_flag_state(vm.arch.flag, Flags::AUX_CARRY) {
        set_byte_reg(vm, ByteReg::AL, (al as i16 - 6) as u8);
        set_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    }
    let al = get_byte_reg(vm, ByteReg::AL);
    if al > 0x9F || get_flag_state(vm.arch.flag, Flags::CARRY) {
        let temp = al as i16 - 0x60;
        set_byte_reg(vm, ByteReg::AL, temp as u8);
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    let res = get_byte_reg(vm, ByteReg::AL);
    set_flag_helper(
        &mut vm.arch.flag,
        res >= 1 << 7,
        res == 0,
        has_even_parity(res),
    );
}

pub fn cbw(vm: &mut VM) {
    let al = get_byte_reg(vm, ByteReg::AL);
    let sign = al & 1 << 7 != 0; //  has sign bit set or not
    if sign {
        set_byte_reg(vm, ByteReg::AH, u8::MAX);
    } else {
        set_byte_reg(vm, ByteReg::AH, 0);
    }
}

pub fn cwd(vm: &mut VM) {
    let ax = vm.arch.ax;
    let sign = ax & 1 << 15 != 0; //  has sign bit set or not
    if sign {
        vm.arch.dx = u16::MAX;
    } else {
        vm.arch.dx = 0;
    }
}

#[inline]
pub fn byte_dec(vm: &mut VM, val: &mut u8) -> Result<(), DivByZero> {
    *val = byte_sub(vm, *val, 1);
    Ok(())
}

#[inline]
pub fn byte_inc(vm: &mut VM, val: &mut u8) -> Result<(), DivByZero> {
    *val = byte_add(vm, *val, 1);
    Ok(())
}

#[inline]
pub fn byte_neg(vm: &mut VM, val: &mut u8) -> Result<(), DivByZero> {
    let res;
    if *val as i8 == -128 {
        res = *val;
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        res = (!*val).wrapping_add(1);
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    if (*val & 0xF) + 1 > 0x0F {
        // auxillary borrow will only set if lower nibble generates carry
        set_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    }
    let sign_change;
    if *val == 0 {
        sign_change = get_flag_state(vm.arch.flag, Flags::SIGN);
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        sign_change = res >= 1 << 7;
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    set_flag_helper(
        &mut vm.arch.flag,
        sign_change,
        res == 0,
        has_even_parity(res),
    );
    *val = res;
    Ok(())
}

pub fn byte_mul(vm: &mut VM, val: &mut u8) -> Result<(), DivByZero> {
    let al = get_byte_reg(vm, ByteReg::AL);
    let res = al as u16 * (*val as u16);
    let ah = get_byte_reg(vm, ByteReg::AH);
    if ah == 0 {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    vm.arch.ax = res;
    Ok(())
}

pub fn byte_imul(vm: &mut VM, val: &mut u8) -> Result<(), DivByZero> {
    let al = get_byte_reg(vm, ByteReg::AL);
    let res = al as u16 as i16 * (*val as i8 as i16);
    let ah = get_byte_reg(vm, ByteReg::AH);
    if ah != u8::MAX {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    vm.arch.ax = res as u16;
    Ok(())
}

pub fn byte_div(vm: &mut VM, val: &mut u8) -> Result<(), DivByZero> {
    if *val == 0 {
        return Err(DivByZero);
    }

    let dividend = vm.arch.ax;
    let quotient = dividend / *val as u16;
    let remainder = dividend % *val as u16;

    set_byte_reg(vm, ByteReg::AH, remainder as u8);
    set_byte_reg(vm, ByteReg::AL, quotient as u8);
    Ok(())
}

pub fn byte_idiv(vm: &mut VM, val: &mut u8) -> Result<(), DivByZero> {
    if *val == 0 {
        return Err(DivByZero);
    }

    let dividend = vm.arch.ax as i16 as i32;
    let quotient = dividend / (*val as i8 as i32);
    let remainder = dividend % (*val as i8 as i32);

    set_byte_reg(vm, ByteReg::AH, remainder as u8);
    set_byte_reg(vm, ByteReg::AL, quotient as u8);
    Ok(())
}

#[inline]
pub fn word_dec(vm: &mut VM, val: &mut u16) -> Result<(), DivByZero> {
    *val = word_sub(vm, *val, 1);
    Ok(())
}

#[inline]
pub fn word_inc(vm: &mut VM, val: &mut u16) -> Result<(), DivByZero> {
    *val = word_add(vm, *val, 1);
    Ok(())
}

#[inline]
pub fn word_neg(vm: &mut VM, val: &mut u16) -> Result<(), DivByZero> {
    let res;
    if *val as i16 == -32768 {
        res = *val;
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        res = (!*val).wrapping_add(1);
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    if (*val & 0xF) + 1 > 0x0F {
        // auxillary borrow will only set if lower nibble generates carry
        set_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::AUX_CARRY);
    }
    let sign_change;
    if *val == 0 {
        sign_change = get_flag_state(vm.arch.flag, Flags::SIGN);
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
    } else {
        sign_change = res >= 1 << 15;
        set_flag(&mut vm.arch.flag, Flags::CARRY);
    }
    set_flag_helper(
        &mut vm.arch.flag,
        sign_change,
        res == 0,
        has_even_parity(res as u8),
    );

    *val = res;
    Ok(())
}

pub fn word_mul(vm: &mut VM, val: &mut u16) -> Result<(), DivByZero> {
    let ax = vm.arch.ax;
    let res = ax as u32 * (*val as u32);
    let upper = ((res & 0xFFFF0000) >> 16) as u16;
    if upper == 0 {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    vm.arch.ax = (res & 0x0000FFFF) as u16;
    vm.arch.dx = upper;
    Ok(())
}

pub fn word_imul(vm: &mut VM, val: &mut u16) -> Result<(), DivByZero> {
    let ax = vm.arch.ax as i16;
    let res = ax as i32 * (*val as i16 as i32);
    let upper = ((res as u32 & 0xFFFF0000) >> 16) as u16;
    if upper != u16::MAX {
        set_flag(&mut vm.arch.flag, Flags::CARRY);
        set_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    } else {
        unset_flag(&mut vm.arch.flag, Flags::CARRY);
        unset_flag(&mut vm.arch.flag, Flags::OVERFLOW);
    }
    vm.arch.ax = (res & 0x0000FFFF) as u16;
    vm.arch.dx = upper;
    Ok(())
}

pub fn word_div(vm: &mut VM, val: &mut u16) -> Result<(), DivByZero> {
    if *val == 0 {
        return Err(DivByZero);
    }

    let dividend = ((vm.arch.dx as u32) << 16) as u32 | vm.arch.ax as u32;
    let quotient = dividend / *val as u32;
    let remainder = dividend % *val as u32;

    vm.arch.ax = quotient as u16;
    vm.arch.dx = remainder as u16;
    Ok(())
}

pub fn word_idiv(vm: &mut VM, val: &mut u16) -> Result<(), DivByZero> {
    if *val == 0 {
        return Err(DivByZero);
    }

    let dividend = ((vm.arch.dx as u32) << 16 | vm.arch.ax as u32) as i32;
    let quotient = dividend / (*val as i16 as i32);
    let remainder = dividend % (*val as i16 as i32);

    vm.arch.ax = quotient as u16;
    vm.arch.dx = remainder as u16;
    Ok(())
}

pub fn byte_add(vm: &mut VM, op1: u8, op2: u8) -> u8 {
    let temp = op1 as u16 + op2 as u16;
    let res = temp as u8;
    let seventh_carry = (op1 & 0x7f) + (op2 & 0x7f) > 0x7f;
    let eighth_carry = temp > u8::MAX as u16;
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: seventh_carry ^ eighth_carry,
            parity: has_even_parity(res),
            sign: res & 0x80 != 0,
            carry: eighth_carry,
            auxillary: ((op1 & 0xF) + (op2 & 0xF)) > 0x0F,
        },
    );
    res
}

pub fn byte_adc(vm: &mut VM, op1: u8, op2: u8) -> u8 {
    let mut temp = op1 as u16 + op2 as u16;
    let mut carry = 0;
    if get_flag_state(vm.arch.flag, Flags::CARRY) {
        temp += 1;
        carry = 1;
    }
    let seventh_carry = (op1 & 0x7f).wrapping_add(op2 & 0x7f).wrapping_add(carry) > 0x7f;
    let eighth_carry = temp > u8::MAX as u16;
    let res = temp as u8;

    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: seventh_carry ^ eighth_carry,
            parity: has_even_parity(res),
            sign: res & 0x80 != 0,
            carry: eighth_carry,
            auxillary: ((op1 & 0xF) + (op2 & 0xF) + carry) > 0xF,
        },
    );

    res
}

pub fn byte_sub(vm: &mut VM, op1: u8, op2: u8) -> u8 {
    let temp = op1 as i16 - op2 as i16;
    let res = temp as u16 as u8;
    let seventh_borrow = op1 & 0x7F < op2 & 0x7F;
    let eighth_borrow = op1 < op2;
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: seventh_borrow ^ eighth_borrow,
            parity: has_even_parity(res),
            sign: res & 0x80 != 0,
            carry: eighth_borrow,
            auxillary: (op1 & 0xF) < (op2 & 0xF),
        },
    );
    res
}

pub fn byte_sbb(vm: &mut VM, op1: u8, op2: u8) -> u8 {
    let mut temp = op1 as i16 - op2 as i16;
    let mut borrow = 0;
    if get_flag_state(vm.arch.flag, Flags::CARRY) {
        temp -= 1;
        borrow = 1;
    }
    let res = temp as u16 as u8;
    let seventh_borrow = op1 & 0x7F < (op2 & 0x7F) + borrow;
    let eighth_borrow = op1 < op2.wrapping_add(borrow);
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: seventh_borrow ^ eighth_borrow,
            parity: has_even_parity(res),
            sign: res & 0x80 != 0,
            carry: eighth_borrow,
            auxillary: (op1 & 0xF) < (op2.wrapping_add(borrow)) & 0xF,
        },
    );
    res
}

pub fn byte_cmp(vm: &mut VM, op1: u8, op2: u8) -> u8 {
    let res = op1 as i16 - op2 as i16;
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: res < i8::MIN as i16,
            parity: has_even_parity(res as u8),
            sign: res as u16 >= 1 << 7,
            carry: op1 < op2,
            auxillary: (op1 & 0xF) < (op2 & 0xF),
        },
    );
    op1
}

// ---- WORD

pub fn word_add(vm: &mut VM, op1: u16, op2: u16) -> u16 {
    let temp = op1 as u32 + op2 as u32;
    let res = temp as u16;
    let fifteenth_carry = (op1 & 0x7fff).wrapping_add(op2 & 0x7fff) > 0x7fff;
    let sixteenth_carry = temp > u16::MAX as u32;
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: fifteenth_carry ^ sixteenth_carry,
            parity: has_even_parity(res as u8),
            sign: res & 0x8000 != 0,
            carry: sixteenth_carry,
            auxillary: (op1 & 0xF).wrapping_add(op2 & 0xF) > 0xF,
        },
    );
    res
}

pub fn word_adc(vm: &mut VM, op1: u16, op2: u16) -> u16 {
    let mut temp = op1 as u32 + op2 as u32;
    let mut carry = 0;
    if get_flag_state(vm.arch.flag, Flags::CARRY) {
        temp += 1;
        carry = 1;
    }
    let res = temp as u16;
    let fifteenth_carry = (op1 & 0x7fff)
        .wrapping_add(op2 & 0x7fff)
        .wrapping_add(carry)
        > 0x7fff;
    let sixteenth_carry = temp > u16::MAX as u32;
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: fifteenth_carry ^ sixteenth_carry,
            parity: has_even_parity(res as u8),
            sign: res & 0x8000 != 0,
            carry: sixteenth_carry,
            auxillary: (op1 & 0xF).wrapping_add(op2 & 0xF).wrapping_add(carry) > 0xF,
        },
    );
    res
}

pub fn word_sub(vm: &mut VM, op1: u16, op2: u16) -> u16 {
    let temp = op1 as i32 - op2 as i32;
    let res = temp as u32 as u16;
    let fifteenth_borrow = (op1 & 0x7fff) < (op2 & 0x7fff);
    let sixteenth_borrow = op1 < op2;
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: fifteenth_borrow ^ sixteenth_borrow,
            parity: has_even_parity(res as u8),
            sign: res & 0x8000 != 0,
            carry: sixteenth_borrow,
            auxillary: op1 & 0xF < op2 & 0xF,
        },
    );
    res
}

pub fn word_sbb(vm: &mut VM, op1: u16, op2: u16) -> u16 {
    let mut temp = op1 as i32 - op2 as i32;
    let mut borrow = 0;
    if get_flag_state(vm.arch.flag, Flags::CARRY) {
        temp -= 1;
        borrow = 1;
    }
    let res = temp as u32 as u16;
    let fifteenth_borrow = (op1 & 0x7fff) < (op2 & 0x7fff) + borrow;
    let sixteenth_borrow = op1 < op2.wrapping_add(borrow);
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: fifteenth_borrow ^ sixteenth_borrow,
            parity: has_even_parity(res as u8),
            sign: res & 0x8000 != 0,
            carry: op1 < op2,
            auxillary: op1 & 0xF < (op2.wrapping_add(borrow) & 0xF),
        },
    );
    res
}

pub fn word_cmp(vm: &mut VM, op1: u16, op2: u16) -> u16 {
    let res = op1 as i32 - op2 as i32;
    set_all_flags(
        vm,
        FlagsToSet {
            zero: res == 0,
            overflow: res < i16::MIN as i32,
            parity: has_even_parity(res as u8),
            sign: res as i16 as u16 >= 1 << 15,
            carry: op1 < op2,
            auxillary: op1 & 0xF < op2 & 0xF,
        },
    );
    op1
}
