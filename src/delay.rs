//! Hardware delays, using Cortex-m systick.

// Based on `stm32l4xx-hal`.

use cast::u32;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use crate::traits::ClockCfg;

/// System timer (SysTick) as a delay provider
pub struct Delay {
    syst: SYST,
    systick_speed: u32,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new<C: ClockCfg>(mut syst: SYST, clocks: &C) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay {
            syst,
            systick_speed: clocks.systick(),
        }
    }

    /// Delay using the Cortex-M systick for a certain duration, ms.
    pub fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }

    /// Delay using the Cortex-M systick for a certain duration, µs. This is the core delay
    /// code all other functions, including the EH trait ones call indirectly.
    pub fn delay_us(&mut self, us: u32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.
        const MAX_RVR: u32 = 0x00FF_FFFF;

        let mut total_rvr = us * (self.systick_speed / 1_000_000);

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            self.syst.set_reload(current_rvr);
            self.syst.clear_current();
            self.syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !self.syst.has_wrapped() {}

            self.syst.disable_counter();
        }
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.syst
    }
}

/// Delay, with sSstem timer (SysTick) as the provider.
impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        Delay::delay_ms(self, ms);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        Delay::delay_ms(self, u32(ms));
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        Delay::delay_ms(self, u32(ms));
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        Delay::delay_us(self, us);
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        Delay::delay_us(self, u32(us));
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        Delay::delay_us(self, u32(us));
    }
}
