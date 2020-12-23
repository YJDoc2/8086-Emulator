use crate::arch::{
    FLAG_AUX_CARRY, FLAG_CARRY, FLAG_DIRECTION, FLAG_INTERRUPT, FLAG_OVERFLOW, FLAG_PARITY,
    FLAG_SIGN, FLAG_TRAP, FLAG_ZERO,
};

#[allow(non_camel_case_types)]
/// Enum to denote flags of 8086
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

/// Funtion to check if a flag is set or not
pub fn get_flag_state(reg: u16, flag: Flags) -> bool {
    match flag {
        Flags::OVERFLOW => reg & FLAG_OVERFLOW != 0,
        Flags::DIRECTION => reg & FLAG_DIRECTION != 0,
        Flags::INTERRUPT => reg & FLAG_INTERRUPT != 0,
        Flags::TRAP => reg & FLAG_TRAP != 0,
        Flags::SIGN => reg & FLAG_SIGN != 0,
        Flags::ZERO => reg & FLAG_ZERO != 0,
        Flags::AUX_CARRY => reg & FLAG_AUX_CARRY != 0,
        Flags::PARITY => reg & FLAG_PARITY != 0,
        Flags::CARRY => reg & FLAG_CARRY != 0,
    }
}

#[test]
fn test_get_flag_state() {
    let reg: u16 = 0b0000000000000000;
    assert_eq!(get_flag_state(reg, Flags::CARRY), false);
    assert_eq!(get_flag_state(reg, Flags::AUX_CARRY), false);

    let reg: u16 = 0b0000100000010001; // overflow,auxillary carry , carry set
    assert_eq!(get_flag_state(reg, Flags::OVERFLOW), true);
    assert_eq!(get_flag_state(reg, Flags::CARRY), true);
    assert_eq!(get_flag_state(reg, Flags::AUX_CARRY), true);
}

/// Function to set a particular flag
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
    assert_eq!(get_flag_state(reg, Flags::CARRY), true);

    set_flag(&mut reg, Flags::AUX_CARRY);
    assert_eq!(get_flag_state(reg, Flags::AUX_CARRY), true);
}

/// Function to unset particular flag
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
    assert_eq!(get_flag_state(reg, Flags::CARRY), false);

    unset_flag(&mut reg, Flags::AUX_CARRY);
    assert_eq!(get_flag_state(reg, Flags::AUX_CARRY), false);
}
