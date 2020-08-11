#![feature(llvm_asm)]

#![no_std]
#![no_main]

use ruduino::Register;

use ruduino::cores::atmega328p::{DDRB, PORTB};

#[no_mangle]
fn main() {
    DDRB::set_mask_raw(0xFFu8);
    loop {
        PORTB::set_mask_raw(0xFF);
        delay();

        PORTB::unset_mask_raw(0xFF);
        delay();
    }
}

fn delay() {
    for _ in 0..400000 {
        unsafe { llvm_asm!("" :::: "volatile")}
    }
}
