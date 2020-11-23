use emulator_8086_lib as lib;

pub fn main() {
    println!("{}", lib::arch::LOWER_NIBBLE);
    println!("{:x}", u16::MAX);
}
