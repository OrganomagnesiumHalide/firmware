use core::time::Duration;

use super::pins::{
    Pin0, Pin1, Pin10, Pin11, Pin12, Pin13, Pin14, Pin15, Pin16, Pin17, Pin18, Pin19, Pin2, Pin20,
    Pin21, Pin22, Pin23, Pin24, Pin25, Pin3, Pin4, Pin5, Pin6, Pin7, Pin8, Pin9,
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
    Pin23,
    Pin24,
    Pin25,
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
                Pin0 { _private: () },
                Pin1 { _private: () },
                Pin2 { _private: () },
                Pin3 { _private: () },
                Pin4 { _private: () },
                Pin5 { _private: () },
                Pin6 { _private: () },
                Pin7 { _private: () },
                Pin8 { _private: () },
                Pin9 { _private: () },
                Pin10 { _private: () },
                Pin11 { _private: () },
                Pin12 { _private: () },
                Pin13 { _private: () },
                Pin14 { _private: () },
                Pin15 { _private: () },
                Pin16 { _private: () },
                Pin17 { _private: () },
                Pin18 { _private: () },
                Pin19 { _private: () },
                Pin20 { _private: () },
                Pin21 { _private: () },
                Pin22 { _private: () },
                Pin23 { _private: () },
                Pin24 { _private: () },
                Pin25 { _private: () },
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
