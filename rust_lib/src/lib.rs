#![allow(clippy::similar_names)]
#![cfg_attr(not(test), no_std)]
use core::time::Duration;

use pico::{
    device::Pico,
    perifs::{i2c::I2C, internal_led::InternalLED, lcd2004a::LCD2004a},
};

pub mod pico;

/// # Panics
/// - This should avoid panicking as much as possible.
/// - It panics if we try to get pins mulitple times, as that would allow reading and writing from them in different places causing errors.
pub fn main() {
    let mut pico = unsafe { Pico::new() };

    // This is only called once, so it shouldn't panic

    let (
        pin0,
        pin1,
        _pin2,
        _pin3,
        pin4,
        pin5,
        _pin6,
        _pin7,
        pin8,
        pin9,
        _pin10,
        _pin11,
        pin12,
        pin13,
        _pin14,
        _pin15,
        pin16,
        pin17,
        _pin18,
        _pin19,
        pin20,
        pin21,
        _pin22,
        pin25,
        _pin26,
        _pin27,
        _pin28,
    ) = pico.get_pins().unwrap();
    let mut led: InternalLED = pin25.into();
    let i2c = I2C::from_pins_4(
        pin4,
        pin5,
        (
            pin0, pin1, pin8, pin9, pin12, pin13, pin16, pin17, pin20, pin21,
        ),
    );
    let mut lcd_screen = match LCD2004a::from_i2c(i2c) {
        Ok(lcd_screen) => lcd_screen,
        Err(e) => match e {
            pico::perifs::lcd2004a::LCDError::InitError => panic!("InitError"),
            pico::perifs::lcd2004a::LCDError::ErrCodeTooLarge => panic!("ErrCodeTooLarge"),
            pico::perifs::lcd2004a::LCDError::WrongI2C => panic!("WrongI2C"),
        },
    };
    lcd_screen.display("This is a test", 0);
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

pub fn panic() {
    #[allow(clippy::empty_loop)]
    loop {}
}
