use crate::vm::VM;
const LOWER_BYTE: i16 = 255;

#[derive(Copy, Clone)]
/// Enum to denote Byte length registers of 8086
pub enum ByteReg {
    AL,
    AH,
    BL,
    BH,
    CL,
    CH,
    DL,
    DH,
}

#[derive(Copy, Clone)]
/// Enum to denote Word length registers of 8086
pub enum WordReg {
    AX,
    BX,
    CX,
    DX,
    SS,
    CS,
    DS,
    ES,
    SP,
    BP,
    SI,
    DI,
}

#[inline]
/// Used to separate lower and higher byte of u16 value
/// returns tuple containing (higher,lower) byte
pub fn separate_bytes(val: i16) -> (u8, u8) {
    let lb = (val & LOWER_BYTE) as u8;
    let hb = ((val & !LOWER_BYTE) >> 8) as u8;
    (hb, lb)
}

#[test]
fn test_separate_byte() {
    let (hb, lb) = separate_bytes(0b0000111111110000);
    assert_eq!(lb, 0b11110000);
    assert_eq!(hb, 0b00001111);
}

/// Returns value of selected byte register
pub fn get_byte_reg(vm: &VM, reg: ByteReg) -> u8 {
    const LB: u16 = LOWER_BYTE as u16;
    const HB: u16 = !LB;
    match reg {
        ByteReg::AL => (vm.arch.ax & LB) as u8,
        ByteReg::AH => ((vm.arch.ax & HB) >> 8) as u8,
        ByteReg::BL => (vm.arch.bx & LB) as u8,
        ByteReg::BH => ((vm.arch.bx & HB) >> 8) as u8,
        ByteReg::CL => (vm.arch.cx & LB) as u8,
        ByteReg::CH => ((vm.arch.cx & HB) >> 8) as u8,
        ByteReg::DL => (vm.arch.dx & LB) as u8,
        ByteReg::DH => ((vm.arch.dx & HB) >> 8) as u8,
    }
}

#[test]
fn test_get_byte_reg() {
    let mut vm = VM::new();
    vm.arch.ax = u16::MAX;
    vm.arch.bx = u8::MAX as u16;
    vm.arch.cx = (u8::MAX as u16) << 8;

    assert_eq!(get_byte_reg(&vm, ByteReg::AL), 255);
    assert_eq!(get_byte_reg(&vm, ByteReg::AH), 255);
    assert_eq!(get_byte_reg(&vm, ByteReg::BL), 255);
    assert_eq!(get_byte_reg(&vm, ByteReg::BH), 0);
    assert_eq!(get_byte_reg(&vm, ByteReg::CL), 0);
    assert_eq!(get_byte_reg(&vm, ByteReg::CH), 255);
}

/// sets value of selected byte register
pub fn set_byte_reg(vm: &mut VM, reg: ByteReg, val: u8) {
    const LB: u16 = LOWER_BYTE as u16;
    const HB: u16 = !LB;
    match reg {
        ByteReg::AL => vm.arch.ax = (vm.arch.ax & HB) | val as u16,
        ByteReg::AH => vm.arch.ax = (vm.arch.ax & LB) | (val as u16) << 8,
        ByteReg::BL => vm.arch.bx = (vm.arch.bx & HB) | val as u16,
        ByteReg::BH => vm.arch.bx = (vm.arch.bx & LB) | (val as u16) << 8,
        ByteReg::CL => vm.arch.cx = (vm.arch.cx & HB) | val as u16,
        ByteReg::CH => vm.arch.cx = (vm.arch.cx & LB) | (val as u16) << 8,
        ByteReg::DL => vm.arch.dx = (vm.arch.dx & HB) | val as u16,
        ByteReg::DH => vm.arch.dx = (vm.arch.dx & LB) | (val as u16) << 8,
    };
}

#[test]
fn test_set_byte_reg() {
    let mut vm = VM::new();

    set_byte_reg(&mut vm, ByteReg::AL, 255);
    assert_eq!(get_byte_reg(&vm, ByteReg::AL), 255);

    set_byte_reg(&mut vm, ByteReg::AH, 255);
    assert_eq!(get_byte_reg(&vm, ByteReg::AH), 255);
    assert_eq!(vm.arch.ax, u16::MAX);

    set_byte_reg(&mut vm, ByteReg::BL, 255);
    assert_eq!(get_byte_reg(&vm, ByteReg::BL), 255);
    assert_eq!(get_byte_reg(&vm, ByteReg::BH), 0);

    set_byte_reg(&mut vm, ByteReg::CH, 255);
    assert_eq!(get_byte_reg(&vm, ByteReg::CL), 0);
    assert_eq!(get_byte_reg(&vm, ByteReg::CH), 255);
}

/// Helper function to get word register value
pub fn get_word_reg_val(vm: &VM, reg: WordReg) -> u16 {
    match reg {
        WordReg::AX => vm.arch.ax,
        WordReg::BX => vm.arch.bx,
        WordReg::CX => vm.arch.cx,
        WordReg::DX => vm.arch.dx,
        WordReg::SS => vm.arch.ss,
        WordReg::DS => vm.arch.ds,
        WordReg::CS => vm.arch.cs,
        WordReg::ES => vm.arch.es,
        WordReg::SI => vm.arch.si,
        WordReg::DI => vm.arch.di,
        WordReg::SP => vm.arch.sp,
        WordReg::BP => vm.arch.bp,
    }
}

/// Helper function to set word register value
pub fn set_word_reg_val(vm: &mut VM, reg: WordReg, val: u16) {
    match reg {
        WordReg::AX => vm.arch.ax = val,
        WordReg::BX => vm.arch.bx = val,
        WordReg::CX => vm.arch.cx = val,
        WordReg::DX => vm.arch.dx = val,
        WordReg::SS => vm.arch.ss = val,
        WordReg::DS => vm.arch.ds = val,
        WordReg::CS => vm.arch.cs = val,
        WordReg::ES => vm.arch.es = val,
        WordReg::SI => vm.arch.si = val,
        WordReg::DI => vm.arch.di = val,
        WordReg::SP => vm.arch.sp = val,
        WordReg::BP => vm.arch.bp = val,
    }
}
