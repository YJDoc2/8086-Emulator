use crate::arch::{
    FLAG_AUX_CARRY, FLAG_CARRY, FLAG_DIRECTION, FLAG_INTERRUPT, FLAG_OVERFLOW, FLAG_PARITY,
    FLAG_SIGN, FLAG_TRAP, FLAG_ZERO,
};

#[allow(non_camel_case_types)]
pub enum Flags {
    OVERFLOW,
    DIRECTION,
    INTERRUPT,
    TRAP,
    SIGN,
    ZERO,
    AUX_CARRY,
    PARITY,
    CARRY,
}

pub fn get_flag_value(reg: u16, flag: Flags) -> u16 {
    match flag {
        Flags::OVERFLOW => reg & FLAG_OVERFLOW,
        Flags::DIRECTION => reg & FLAG_DIRECTION,
        Flags::INTERRUPT => reg & FLAG_INTERRUPT,
        Flags::TRAP => reg & FLAG_TRAP,
        Flags::SIGN => reg & FLAG_SIGN,
        Flags::ZERO => reg & FLAG_ZERO,
        Flags::AUX_CARRY => reg & FLAG_AUX_CARRY,
        Flags::PARITY => reg & FLAG_PARITY,
        Flags::CARRY => reg & FLAG_CARRY,
    }
}

#[test]
fn test_get_flag_value() {
    let reg: u16 = 0b0000000000000000;
    assert_eq!(get_flag_value(reg, Flags::CARRY), 0);
    assert_eq!(get_flag_value(reg, Flags::AUX_CARRY), 0);

    let reg: u16 = 0b0000100000010001; // overflow,auxillary carry , carry set
    assert_ne!(get_flag_value(reg, Flags::OVERFLOW), 0);
    assert_ne!(get_flag_value(reg, Flags::CARRY), 0);
    assert_ne!(get_flag_value(reg, Flags::AUX_CARRY), 0);
}

pub fn set_flag(reg: &mut u16, flag: Flags) {
    match flag {
        Flags::OVERFLOW => *reg = *reg | FLAG_OVERFLOW,
        Flags::DIRECTION => *reg = *reg | FLAG_DIRECTION,
        Flags::INTERRUPT => *reg = *reg | FLAG_INTERRUPT,
        Flags::TRAP => *reg = *reg | FLAG_TRAP,
        Flags::SIGN => *reg = *reg | FLAG_SIGN,
        Flags::ZERO => *reg = *reg | FLAG_ZERO,
        Flags::AUX_CARRY => *reg = *reg | FLAG_AUX_CARRY,
        Flags::PARITY => *reg = *reg | FLAG_PARITY,
        Flags::CARRY => *reg = *reg | FLAG_CARRY,
    };
}

#[test]
fn test_set_flag() {
    let mut reg: u16 = 0b0000000000000000;

    set_flag(&mut reg, Flags::CARRY);
    assert_ne!(get_flag_value(reg, Flags::CARRY), 0);

    set_flag(&mut reg, Flags::AUX_CARRY);
    assert_ne!(get_flag_value(reg, Flags::AUX_CARRY), 0);
}

pub fn unset_flag(reg: &mut u16, flag: Flags) {
    match flag {
        Flags::OVERFLOW => *reg = *reg & !FLAG_OVERFLOW,
        Flags::DIRECTION => *reg = *reg & !FLAG_DIRECTION,
        Flags::INTERRUPT => *reg = *reg & !FLAG_INTERRUPT,
        Flags::TRAP => *reg = *reg & !FLAG_TRAP,
        Flags::SIGN => *reg = *reg & !FLAG_SIGN,
        Flags::ZERO => *reg = *reg & !FLAG_ZERO,
        Flags::AUX_CARRY => *reg = *reg & !FLAG_AUX_CARRY,
        Flags::PARITY => *reg = *reg & !FLAG_PARITY,
        Flags::CARRY => *reg = *reg & !FLAG_CARRY,
    };
}

#[test]
fn test_unset_flag() {
    let mut reg: u16 = u16::MAX;

    unset_flag(&mut reg, Flags::CARRY);
    assert_eq!(get_flag_value(reg, Flags::CARRY), 0);

    unset_flag(&mut reg, Flags::AUX_CARRY);
    assert_eq!(get_flag_value(reg, Flags::AUX_CARRY), 0);
}
