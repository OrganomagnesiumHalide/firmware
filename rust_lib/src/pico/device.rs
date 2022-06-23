use core::time::Duration;

use super::pins::{
    Pin0, Pin1, Pin10, Pin11, Pin12, Pin13, Pin14, Pin15, Pin16, Pin17, Pin18, Pin19, Pin2, Pin20,
    Pin21, Pin22, Pin25, Pin26, Pin27, Pin28, Pin3, Pin4, Pin5, Pin6, Pin7, Pin8, Pin9,
};

type Pins = (
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
    Pin16,
    Pin17,
    Pin18,
    Pin19,
    Pin20,
    Pin21,
    Pin22,
    Pin25,
    Pin26,
    Pin27,
    Pin28,
);
pub struct Pico {
    pins: Option<Pins>,
}
unsafe impl Send for Pico {}

impl Pico {
    /// This initializes the device. You can get pins from it, but only once.
    /// # Safety
    /// - If you call this twice, you risk undefined behavior.
    #[must_use]
    pub unsafe fn new() -> Self {
        Pico {
            pins: Option::Some((
                Pin0::new(),
                Pin1::new(),
                Pin2::new(),
                Pin3::new(),
                Pin4::new(),
                Pin5::new(),
                Pin6::new(),
                Pin7::new(),
                Pin8::new(),
                Pin9::new(),
                Pin10::new(),
                Pin11::new(),
                Pin12::new(),
                Pin13::new(),
                Pin14::new(),
                Pin15::new(),
                Pin16::new(),
                Pin17::new(),
                Pin18::new(),
                Pin19::new(),
                Pin20::new(),
                Pin21::new(),
                Pin22::new(),
                Pin25::new(),
                Pin26::new(),
                Pin27::new(),
                Pin28::new(),
            )),
        }
    }
    pub fn get_pins(&mut self) -> Option<Pins> {
        self.pins.take()
    }

    #[allow(clippy::unused_self)]
    pub fn sleep(&mut self, ms: Duration) {
        unsafe {
            let amnt_ms = ms.as_millis().try_into().unwrap_or(u32::MAX);
            rust_bridge::c_functions::c_device_sleep(amnt_ms);
        }
    }
}
