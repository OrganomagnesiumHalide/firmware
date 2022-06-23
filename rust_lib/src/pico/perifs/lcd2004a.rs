use super::i2c::I2C;
use crate::pico::pins::Pin;
use core::{ffi::c_void, mem};
use rust_bridge::c_functions;

pub struct LCD2004a {
    ptr: *mut c_void,
}

impl LCD2004a {
    /// `from_i2c` creates an LCD object from an i2c.
    /// # Errors
    /// * `LCDError::WrongI2C` if it's given an incorrect i2c channel.
    /// * `LCDError::InitError` if it fails to initialize the device.
    /// * `LCDError::ErrCodeTooLarge` is an assertion error. If the library returns this, there's a bug in the library
    pub fn from_i2c<T: Pin, Q: Pin>(i2c: I2C<T, Q>) -> Result<LCD2004a, LCDError> {
        let mut err_code: i8 = -1;
        let ptr = unsafe {
            c_functions::c_lcd_init(
                i2c.get_num(),
                0x27,
                i2c.sda.get_pin(),
                i2c.scl.get_pin(),
                &mut err_code as *mut i8,
            )
        };
        mem::drop(i2c);
        match err_code {
            0 => Ok(LCD2004a { ptr }),
            c_functions::WRONG_I2C_ASSERT_ERROR => Err(LCDError::WrongI2C),
            c_functions::FAILED_TO_INIT_LCD_DEVICE => Err(LCDError::InitError),
            _ => Err(LCDError::ErrCodeTooLarge),
        }
    }
    pub fn display<'a, S: Into<&'a str>>(&mut self, string: S, line: u8) {
        unsafe { c_functions::c_lcd_clear(self.ptr) };
        let s: &'a str = string.into();
        for (pos, c) in s.as_bytes().iter().enumerate() {
            unsafe {
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_possible_wrap)]
                c_functions::c_lcd_putch(
                    self.ptr,
                    line,
                    if pos <= 128 { pos as i32 } else { 129 },
                    *c,
                );
            };
        }
    }
}

pub enum LCDError {
    InitError,
    ErrCodeTooLarge,
    WrongI2C,
}
