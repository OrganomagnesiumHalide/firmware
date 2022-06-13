#![cfg_attr(not(test), no_std)]
use core::time::Duration;

use crate::pico::perifs::InternalLED;
use pico::device::Pico;

pub mod pico;

/// # Panics
/// - This should avoid panicking as much as possible.
/// - It panics if we try to get pins mulitple times, as that would allow reading and writing from them in different places causing errors.
pub fn main() {
    let mut pico = unsafe { Pico::new() };

    // This is only called once, so it shouldn't panic

    let (
        _pin0,
        _pin1,
        _pin2,
        _pin3,
        _pin4,
        _pin5,
        _pin6,
        _pin7,
        _pin8,
        _pin9,
        _pin10,
        _pin11,
        _pin12,
        _pin13,
        _pin14,
        _pin15,
        _pin16,
        _pin17,
        _pin18,
        _pin19,
        _pin20,
        _pin21,
        _pin22,
        pin25,
        _pin26,
        _pin27,
        _pin28,
    ) = pico.get_pins().unwrap();
    let mut led: InternalLED = pin25.into();
    loop {
        blink(&mut pico, &mut led);
    }
}

pub fn blink(pico: &mut Pico, led: &mut InternalLED) {
    led.turn_on();
    pico.sleep(Duration::from_secs(1));
    led.turn_off();
    pico.sleep(Duration::from_secs(1));
}
