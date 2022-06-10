use super::pins::Pin25;

pub struct InternalLED {
    pin: u8,
    is_on: bool,
}

impl InternalLED {
    pub fn turn_on(&mut self) {
        // 1 becomes true, turning on the LED
        unsafe {
            rust_bridge::c_functions::c_gpio_put(self.pin, 1);
        }
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        // 0 becomes false, turning off the LED
        unsafe {
            rust_bridge::c_functions::c_gpio_put(self.pin, 0);
        }
        self.is_on = false;
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

// Pin 25 is the internal pin
impl From<Pin25> for InternalLED {
    fn from(_: Pin25) -> Self {
        unsafe {
            rust_bridge::c_functions::c_gpio_init(25);
            rust_bridge::c_functions::c_gpio_set_dir(25, 1);
        }

        InternalLED {
            pin: 25,
            is_on: false,
        }
    }
}
