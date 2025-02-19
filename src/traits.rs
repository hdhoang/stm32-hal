//! Traits used across the library. These may find general use beyond this library, and this
//! module may eventually be moved to a standalone crate.

/// Is a set of speeds valid?
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ClocksValid {
    Valid,
    NotValid,
}

/// This trait allows you to return information about a common's speeds.
/// It's used for configuring peripherals.
pub trait ClockCfg {
    /// System clock speed, in Hz.
    fn sysclk(&self) -> u32;

    /// HCLK speed, in Hz. Ie AHB bus, core, memory, and DMA.
    fn hclk(&self) -> u32;

    /// Cortex System timer speed, in Hz.
    fn systick(&self) -> u32;

    /// USB speed, in Hz.
    fn usb(&self) -> u32;

    /// APB1 peripheral common speed, in Hz.
    fn apb1(&self) -> u32;

    /// APB1 timer common speed, in Hz.
    fn apb1_timer(&self) -> u32;

    /// APB2 timer common speed, in Hz.
    fn apb2(&self) -> u32;

    /// APB2 peripheral common speed, in Hz.
    fn apb2_timer(&self) -> u32;

    // Todo: Optional APB3, APB4, AHB2, AHB4, HCLK3. These are for H7.

    /// Validate that the clocks speeds are all within the acceptable range
    /// for the MCU
    /// // todo Separate USB validation? back to `Validation enum`, or keep it simple?
    fn validate_speeds(&self) -> ClocksValid;
}
