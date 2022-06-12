#![cfg_attr(not(test), no_std)]
use core::time::Duration;

use crate::pico::perifs::InternalLED;
use pico::device::Pico;

pub mod pico;

pub fn main() {
    let mut pico = unsafe { Pico::new() };

    // This is only called once, so it shouldn't panic

    let pins = pico.get_pins().unwrap();
    let mut led: InternalLED = pins.25.into();
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
